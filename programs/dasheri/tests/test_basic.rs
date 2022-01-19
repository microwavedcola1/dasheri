use anchor_lang::Key;
use mango::state::{MangoGroup, NodeBank, RootBank, QUOTE_INDEX};
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::signer::Signer;

use program_test::*;

mod program_test;

#[allow(unaligned_references)]
#[tokio::test]
async fn test_basic() {
    // Setup
    let mut context = TestContext::new().await;
    let mango_group_cookie = MangoGroupCookie::default(&mut context).await;

    // Create mango account
    const ACCOUNT_NUM: u64 = 0_u64;
    let (mango_account, _) = Pubkey::find_program_address(
        &[
            &mango_group_cookie.address.as_ref(),
            &&context.solana.context.borrow().payer.pubkey().as_ref(),
            &ACCOUNT_NUM.to_le_bytes(),
        ],
        &context.mango.program_id,
    );
    let instructions = vec![Instruction {
        program_id: context.dasheri.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::CreateMangoAccount {
                mango_program: context.mango.program_id,
                mango_group: mango_group_cookie.address,
                mango_account: mango_account,
                owner: context.solana.context.borrow().payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::CreateMangoAccount {
            account_num: ACCOUNT_NUM,
        }),
    }];
    context
        .solana
        .process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();

    // Deposit
    let test_quote_mint_index = context.mints.len() - 1;
    let mango_group = context
        .load_account::<MangoGroup>(mango_group_cookie.address)
        .await;
    println!(
        "mango_group.tokens[test_quote_mint_index].root_bank {:?}",
        mango_group.tokens[QUOTE_INDEX].root_bank
    );
    let root_bank_pk = mango_group.tokens[QUOTE_INDEX].root_bank;
    let root_bank = context.load_account::<RootBank>(root_bank_pk).await;
    let node_bank_pk = root_bank.node_banks[0];
    let node_bank = context.load_account::<NodeBank>(node_bank_pk).await;

    let instructions = vec![Instruction {
        program_id: context.dasheri.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::Deposit {
                mango_program: context.mango.program_id,
                mango_group: mango_group_cookie.address,
                mango_cache: mango_group.mango_cache.key(),
                root_bank: root_bank_pk,
                node_bank: node_bank_pk,
                node_bank_vault: node_bank.vault.key(),
                owner_token_account: context.users[0].token_accounts[test_quote_mint_index].key(),
                mango_account: mango_account,
                owner: context.users[0].key.pubkey(),
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::Deposit { quantity: 100 }),
    }];
    context
        .solana
        .process_transaction(&instructions, Some(&[&context.users[0].key]))
        .await
        .unwrap();
}
