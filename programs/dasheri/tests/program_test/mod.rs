use std::cell::RefCell;
use std::{str::FromStr, sync::Arc};

use mango::ids::msrm_token;
use mango_common::Loadable;
use solana_program::account_info::AccountInfo;
use solana_program::rent::Rent;
use solana_program::{program_option::COption, program_pack::Pack, system_instruction};
use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token::{state::*, *};

pub use cookies::*;
pub use solana::*;
pub use utils::*;

pub mod cookies;
pub mod solana;
pub mod utils;

// Flip for debugging
const RUST_LOG_DEFAULT: &str = "debug";
// const RUST_LOG_DEFAULT: &str = "solana_rbpf::vm=info,\
//              solana_program_runtime::stable_log=debug,\
//              solana_runtime::message_processor=debug,\
//              solana_runtime::system_instruction_processor=info,\
//              solana_program_test=info";

trait AddPacked {
    fn add_packable_account<T: Pack>(
        &mut self,
        pubkey: Pubkey,
        amount: u64,
        data: &T,
        owner: &Pubkey,
    );
}

impl AddPacked for ProgramTest {
    fn add_packable_account<T: Pack>(
        &mut self,
        pubkey: Pubkey,
        amount: u64,
        data: &T,
        owner: &Pubkey,
    ) {
        let mut account = solana_sdk::account::Account::new(amount, T::get_packed_len(), owner);
        data.pack_into_slice(&mut account.data);
        self.add_account(pubkey, account);
    }
}

pub struct MangoProgramTestConfig {
    pub compute_limit: u64,
    pub num_users: usize,
    pub num_mints: usize,
}

impl MangoProgramTestConfig {
    #[allow(dead_code)]
    pub fn default() -> Self {
        MangoProgramTestConfig {
            compute_limit: 200_000,
            num_users: 2,
            num_mints: 16,
        }
    }
}

pub struct TestContext {
    pub solana: Arc<SolanaCookie>,

    pub serum: SerumCookie,

    pub mango: MangoCookie,
    pub mints: Vec<MintCookie>,
    pub users: Vec<UserCookie>,
    pub quote_index: usize,

    pub dasheri: DasheriCookie,
}

impl TestContext {
    pub async fn new() -> Self {
        let dasheri_program_id =
            Pubkey::from_str(&"Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();

        let mut test = ProgramTest::new("dasheri", dasheri_program_id, processor!(dasheri::entry));

        test.set_compute_max_units(100000);

        let mango_program_id = Pubkey::new_unique();
        test.add_program("mango", mango_program_id, None);

        let serum_program_id = Pubkey::new_unique();
        test.add_program("serum_dex", serum_program_id, None);

        solana_logger::setup_with_default(RUST_LOG_DEFAULT);

        // Setup the environment

        // Mints
        let mut mints: Vec<MintCookie> = vec![
            MintCookie {
                index: 0,
                decimals: 6,
                unit: 10u64.pow(6) as f64,
                base_lot: 100 as f64,
                quote_lot: 10 as f64,
                pubkey: None,
                authority: Keypair::new(),
            },
            MintCookie {
                index: 1,
                decimals: 6,
                unit: 10u64.pow(6) as f64,
                base_lot: 0 as f64,
                quote_lot: 0 as f64,
                pubkey: None,
                authority: Keypair::new(),
            },
            MintCookie {
                index: 1,
                decimals: 6,
                unit: 10u64.pow(6) as f64,
                base_lot: 0 as f64,
                quote_lot: 0 as f64,
                pubkey: None,
                authority: Keypair::new(),
            },
        ];

        // Add MSRM mint
        test.add_packable_account(
            msrm_token::ID,
            u32::MAX as u64,
            &Mint {
                is_initialized: true,
                mint_authority: COption::Some(Pubkey::new_unique()),
                decimals: 6,
                ..Mint::default()
            },
            &spl_token::id(),
        );

        for mint_index in 0..mints.len() {
            let mint_pk: Pubkey;
            if mints[mint_index].pubkey.is_none() {
                mint_pk = Pubkey::new_unique();
            } else {
                mint_pk = mints[mint_index].pubkey.unwrap();
            }

            test.add_packable_account(
                mint_pk,
                u32::MAX as u64,
                &Mint {
                    is_initialized: true,
                    mint_authority: COption::Some(mints[mint_index].authority.pubkey()),
                    decimals: mints[mint_index].decimals,
                    ..Mint::default()
                },
                &spl_token::id(),
            );
            mints[mint_index].pubkey = Some(mint_pk);
        }
        let quote_index = mints.len() - 1;

        // Users
        let num_users = 4;
        let mut users = Vec::new();
        for _ in 0..num_users {
            let user_key = Keypair::new();
            test.add_account(
                user_key.pubkey(),
                solana_sdk::account::Account::new(
                    u32::MAX as u64,
                    0,
                    &solana_sdk::system_program::id(),
                ),
            );

            // give every user 10^18 (< 2^60) of every token
            // ~~ 1 trillion in case of 6 decimals
            let mut token_accounts = Vec::new();
            for mint_index in 0..mints.len() {
                let token_key = Pubkey::new_unique();
                test.add_packable_account(
                    token_key,
                    u32::MAX as u64,
                    &spl_token::state::Account {
                        mint: mints[mint_index].pubkey.unwrap(),
                        owner: user_key.pubkey(),
                        amount: 1_000_000_000_000_000_000,
                        state: spl_token::state::AccountState::Initialized,
                        ..spl_token::state::Account::default()
                    },
                    &spl_token::id(),
                );

                token_accounts.push(token_key);
            }
            users.push(UserCookie {
                key: user_key,
                token_accounts,
            });
        }

        let context = test.start_with_context().await;

        let solana = Arc::new(SolanaCookie {
            context: RefCell::new(context),
        });

        Self {
            solana: solana.clone(),
            serum: SerumCookie {
                solana: Arc::clone(&solana),
                program_id: serum_program_id,
            },
            mango: MangoCookie {
                solana: Arc::clone(&solana),
                program_id: mango_program_id,
            },
            dasheri: DasheriCookie {
                solana: Arc::clone(&solana),
                program_id: dasheri_program_id,
            },
            mints,
            users,
            quote_index,
        }
    }

    #[allow(dead_code)]
    pub async fn create_account(&mut self, size: usize, owner: &Pubkey) -> Pubkey {
        let keypair = Keypair::new();
        let rent = self.get_min_rent_for_size(size).await;

        let instructions = [system_instruction::create_account(
            &self.solana.context.borrow_mut().payer.pubkey(),
            &keypair.pubkey(),
            rent,
            size as u64,
            owner,
        )];

        self.solana
            .process_transaction(&instructions, Some(&[&keypair]))
            .await
            .unwrap();

        return keypair.pubkey();
    }

    async fn get_min_rent_for_size(&mut self, size: usize) -> u64 {
        self.solana
            .context
            .borrow_mut()
            .banks_client
            .get_sysvar::<Rent>()
            .await
            .unwrap()
            .minimum_balance(size)
    }

    #[allow(dead_code)]
    pub async fn create_token_account(&mut self, owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        let keypair = Keypair::new();
        let rent = self
            .get_min_rent_for_size(spl_token::state::Account::LEN)
            .await;

        let instructions = [
            system_instruction::create_account(
                &self.solana.context.borrow_mut().payer.pubkey(),
                &keypair.pubkey(),
                rent as u64,
                spl_token::state::Account::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_account(
                &spl_token::id(),
                &keypair.pubkey(),
                mint,
                owner,
            )
            .unwrap(),
        ];

        self.solana
            .process_transaction(&instructions, Some(&[&keypair]))
            .await
            .unwrap();
        return keypair.pubkey();
    }

    pub async fn load_account<T: Loadable>(&mut self, acc_pk: Pubkey) -> T {
        let mut acc = self
            .solana
            .context
            .borrow_mut()
            .banks_client
            .get_account(acc_pk)
            .await
            .unwrap()
            .unwrap();
        let acc_info: AccountInfo = (&acc_pk, &mut acc).into();
        return *T::load(&acc_info).unwrap();
    }
}
