use anchor_lang::Key;
use mango::state::{MangoGroup, NodeBank, RootBank, QUOTE_INDEX};
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::cookies::MangoGroupCookie;
use program_test::*;

mod program_test;

#[allow(unaligned_references)]
#[tokio::test]
async fn test_pool() {
    // Setup
    let config = MangoProgramTestConfig::default();
    let mut test = MangoProgramTest::start_new(&config).await;
    let mut mango_group_cookie = MangoGroupCookie::default(&mut test).await;
    MangoGroupCookie::full_setup(
        &mut mango_group_cookie,
        &mut test,
        config.num_users,
        config.num_mints - 1,
    )
    .await;

    // Create pool
    let (pool, bump) = Pubkey::find_program_address(
        &[b"pool".as_ref(), test.context.payer.pubkey().as_ref()],
        &test.dasheri_program_id,
    );
    let vault = spl_associated_token_account::get_associated_token_address(
        &pool,
        &test.mints[test.mints.len() - 1].pubkey.unwrap(),
    );
    create_pool(&mut test, &pool, bump, &vault).await;

    // Create pool account
    let (pool_account, bump) = Pubkey::find_program_address(
        &[
            b"pool_account".as_ref(),
            pool.as_ref(),
            test.users[0].pubkey().as_ref(),
        ],
        &test.dasheri_program_id,
    );
    create_pool_account(&mut test, &pool, &pool_account, bump).await;

    // Deposit into pool
    deposit_into_pool(&mut test, &pool, &vault, &pool_account).await;

    // Create mango account
    const ACCOUNT_NUM: u64 = 0_u64;
    let (mango_account, bump) = Pubkey::find_program_address(
        &[
            &mango_group_cookie.address.as_ref(),
            &pool.as_ref(),
            &ACCOUNT_NUM.to_le_bytes(),
        ],
        &test.mango_program_id,
    );
    create_mango_account(
        &mut test,
        &mut mango_group_cookie,
        &pool,
        &mango_account,
        bump,
        ACCOUNT_NUM,
    )
    .await;

    // Update cache
    mango_group_cookie.run_keeper(&mut test).await;

    // Deposit
    let mango_group = test
        .load_account::<MangoGroup>(mango_group_cookie.address)
        .await;
    deposit_into_mango_account(
        &mut test,
        &mut mango_group_cookie,
        &pool,
        &vault,
        &mango_account,
        &mango_group,
    )
    .await;
}

async fn create_pool(test: &mut MangoProgramTest, pool: &Pubkey, bump: u8, vault: &Pubkey) {
    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::PoolCreatePool {
                pool: *pool,
                vault: *vault,
                deposit_mint: test.mints[test.mints.len() - 1].pubkey.unwrap(),
                admin: test.context.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
                associated_token_program: spl_associated_token_account::id(),
                rent: solana_sdk::sysvar::rent::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::PoolCreatePool { bump }),
    }];

    test.process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();
}

async fn create_pool_account(
    test: &mut MangoProgramTest,
    pool: &Pubkey,
    pool_account: &Pubkey,
    bump: u8,
) {
    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::PoolCreatePoolAccount {
                pool_account: *pool_account,
                pool: *pool,
                user: test.users[0].pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::PoolCreatePoolAccount {
            bump,
        }),
    }];

    test.process_transaction(
        &instructions,
        Some(&[&Keypair::from_base58_string(
            &test.users[0].to_base58_string(),
        )]),
    )
    .await
    .unwrap();
}

async fn deposit_into_pool(
    test: &mut MangoProgramTest,
    pool: &Pubkey,
    vault: &Pubkey,
    pool_account: &Pubkey,
) {
    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::PoolDepositIntoPool {
                pool: *pool,
                vault: *vault,
                pool_account: *pool_account,
                deposit_token: test.token_accounts[15].key(),
                user: test.users[0].pubkey(),
                token_program: spl_token::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::PoolDepositIntoPool {
            amount: 100_000_000,
        }),
    }];
    test.process_transaction(
        &instructions,
        Some(&[&Keypair::from_base58_string(
            &test.users[0].to_base58_string(),
        )]),
    )
    .await
    .unwrap();
}

async fn create_mango_account(
    test: &mut MangoProgramTest,
    mango_group_cookie: &mut MangoGroupCookie,
    pool: &Pubkey,
    mango_account: &Pubkey,
    bump: u8,
    ACCOUNT_NUM: u64,
) {
    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::PoolCreateMangoAccount {
                mango_program: test.mango_program_id,
                mango_group: mango_group_cookie.address,
                mango_account: *mango_account,
                pool: *pool,
                payer: test.context.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::PoolCreateMangoAccount {
            account_num: ACCOUNT_NUM,
            bump,
        }),
    }];
    test.process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();
}

async fn deposit_into_mango_account(
    test: &mut MangoProgramTest,
    mango_group_cookie: &mut MangoGroupCookie,
    pool: &Pubkey,
    vault: &Pubkey,
    mango_account: &Pubkey,
    mango_group: &MangoGroup,
) {
    let root_bank_pk = mango_group.tokens[QUOTE_INDEX].root_bank;
    let root_bank = test.load_account::<RootBank>(root_bank_pk).await;
    let node_bank_pk = root_bank.node_banks[0];
    let node_bank = test.load_account::<NodeBank>(node_bank_pk).await;

    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::PoolDepositIntoMangoAccount {
                mango_program: test.mango_program_id,
                mango_group: mango_group_cookie.address,
                mango_cache: mango_group.mango_cache.key(),
                root_bank: root_bank_pk,
                node_bank: node_bank_pk,
                node_bank_vault: node_bank.vault.key(),
                owner_token_account: *vault,
                mango_account: *mango_account,
                pool: *pool,
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(
            &dasheri::instruction::PoolDepositIntoMangoAccount { quantity: 100 },
        ),
    }];
    test.process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();
}
