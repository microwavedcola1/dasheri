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
async fn test_basic() {
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

    // Create mango account
    const ACCOUNT_NUM: u64 = 0_u64;
    let (mango_account, _) = Pubkey::find_program_address(
        &[
            &mango_group_cookie.address.as_ref(),
            &test.context.payer.pubkey().as_ref(),
            &ACCOUNT_NUM.to_le_bytes(),
        ],
        &test.mango_program_id,
    );
    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::CreateMangoAccount {
                mango_program: test.mango_program_id,
                mango_group: mango_group_cookie.address,
                mango_account: mango_account,
                owner: test.context.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::CreateMangoAccount {
            account_num: ACCOUNT_NUM,
        }),
    }];
    test.process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();

    // Update cache
    mango_group_cookie.run_keeper(&mut test).await;

    // Deposit
    let test_quote_mint_index = test.mints.len() - 1;
    let mango_group = test
        .load_account::<MangoGroup>(mango_group_cookie.address)
        .await;
    println!(
        "mango_group.tokens[test_quote_mint_index].root_bank {:?}",
        mango_group.tokens[QUOTE_INDEX].root_bank
    );
    let root_bank_pk = mango_group.tokens[QUOTE_INDEX].root_bank;
    let root_bank = test.load_account::<RootBank>(root_bank_pk).await;
    let node_bank_pk = root_bank.node_banks[0];
    let node_bank = test.load_account::<NodeBank>(node_bank_pk).await;

    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::DepositIntoMangoAccount {
                mango_program: test.mango_program_id,
                mango_group: mango_group_cookie.address,
                mango_cache: mango_group.mango_cache.key(),
                root_bank: root_bank_pk,
                node_bank: node_bank_pk,
                node_bank_vault: node_bank.vault.key(),
                owner_token_account: test.token_accounts[test_quote_mint_index].key(),
                mango_account: mango_account,
                owner: test.users[0].pubkey(),
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::Deposit { quantity: 100 }),
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
