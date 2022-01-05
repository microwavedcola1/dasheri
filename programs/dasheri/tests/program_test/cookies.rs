use std::mem::size_of;
use std::sync::Arc;

use fixed::types::I80F48;
use mango::ids::msrm_token;
use mango::state::{MangoCache, MangoGroup, NodeBank, RootBank};
use solana_program::pubkey::*;
use solana_sdk::signature::{Keypair, Signer};

use crate::utils::*;
use crate::{SolanaCookie, TestContext};

pub struct DasheriCookie {
    pub solana: Arc<SolanaCookie>,
    pub program_id: Pubkey,
}
pub struct MangoCookie {
    pub solana: Arc<SolanaCookie>,
    pub program_id: Pubkey,
}
pub struct SerumCookie {
    pub solana: Arc<SolanaCookie>,
    pub program_id: Pubkey,
}

pub struct MintCookie {
    pub index: usize,
    pub decimals: u8,
    pub unit: f64,
    pub base_lot: f64,
    pub quote_lot: f64,
    pub pubkey: Option<Pubkey>,
    pub authority: Keypair,
}

impl Clone for MintCookie {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            decimals: self.decimals,
            unit: self.unit,
            base_lot: self.base_lot,
            quote_lot: self.quote_lot,
            pubkey: self.pubkey.clone(),
            authority: clone_keypair(&self.authority),
        }
    }
}

pub struct UserCookie {
    pub key: Keypair,
    pub token_accounts: Vec<Pubkey>,
}

pub struct MangoGroupCookie {
    pub address: Pubkey,
    pub mango_group: MangoGroup,
    pub mango_cache: MangoCache,
}

impl MangoGroupCookie {
    #[allow(dead_code)]
    pub async fn default(test: &mut TestContext) -> Self {
        let mango_program_id = test.mango.program_id;
        let serum_program_id = test.serum.program_id;

        let mango_group_pk = test
            .create_account(size_of::<MangoGroup>(), &mango_program_id)
            .await;
        let mango_cache_pk = test
            .create_account(size_of::<MangoCache>(), &mango_program_id)
            .await;
        let (signer_pk, signer_nonce) =
            create_signer_key_and_nonce(&mango_program_id, &mango_group_pk);
        let admin_pk = &test.solana.context.borrow_mut().payer.pubkey();

        let quote_mint_pk = test.mints[test.quote_index].pubkey.unwrap();
        let quote_vault_pk = test.create_token_account(&signer_pk, &quote_mint_pk).await;
        let quote_node_bank_pk = test
            .create_account(size_of::<NodeBank>(), &mango_program_id)
            .await;
        let quote_root_bank_pk = test
            .create_account(size_of::<RootBank>(), &mango_program_id)
            .await;
        let dao_vault_pk = test.create_token_account(&signer_pk, &quote_mint_pk).await;
        let msrm_vault_pk = test.create_token_account(&signer_pk, &msrm_token::ID).await;
        let fees_vault_pk = test.create_token_account(&signer_pk, &quote_mint_pk).await;

        let quote_optimal_util = I80F48::from_num(0.7);
        let quote_optimal_rate = I80F48::from_num(0.06);
        let quote_max_rate = I80F48::from_num(1.5);

        let instructions = [mango::instruction::init_mango_group(
            &mango_program_id,
            &mango_group_pk,
            &signer_pk,
            &admin_pk,
            &quote_mint_pk,
            &quote_vault_pk,
            &quote_node_bank_pk,
            &quote_root_bank_pk,
            &dao_vault_pk,
            &msrm_vault_pk,
            &fees_vault_pk,
            &mango_cache_pk,
            &serum_program_id,
            signer_nonce,
            5,
            quote_optimal_util,
            quote_optimal_rate,
            quote_max_rate,
        )
        .unwrap()];

        test.solana
            .process_transaction(&instructions, None)
            .await
            .unwrap();

        let mango_group = test.load_account::<MangoGroup>(mango_group_pk).await;
        let mango_cache = test
            .load_account::<MangoCache>(mango_group.mango_cache)
            .await;

        MangoGroupCookie {
            address: mango_group_pk,
            mango_group: mango_group,
            mango_cache: mango_cache,
        }
    }
}
