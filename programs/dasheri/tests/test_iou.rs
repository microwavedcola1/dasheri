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
async fn test_iou() {
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

    // Update prices
    mango_group_cookie.run_keeper(&mut test).await;

    // Init gateway
    let (gateway, _gateway_bump) = Pubkey::find_program_address(
        &[b"gateway".as_ref(), test.context.payer.pubkey().as_ref()],
        &test.dasheri_program_id,
    );
    let (deposit_iou_mint, _deposit_iou_mint_bump) = Pubkey::find_program_address(
        &[test.mints[test.mints.len() - 1].pubkey.unwrap().as_ref()],
        &test.dasheri_program_id,
    );
    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::IouInitGateway {
                gateway,
                deposit_iou_mint,
                token_mint: test.mints[test.mints.len() - 1].pubkey.unwrap(),
                admin: test.context.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
                rent: solana_program::sysvar::rent::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&dasheri::instruction::IouInitGateway {
            _gateway_bump,
            _deposit_iou_mint_bump,
        }),
    }];
    &mut test
        .process_transaction(
            &instructions,
            Some(&[&Keypair::from_base58_string(
                &test.context.payer.to_base58_string(),
            )]),
        )
        .await
        .unwrap();

    // Deposit
    let mango_group = test
        .load_account::<MangoGroup>(mango_group_cookie.address)
        .await;
    let root_bank_pk = mango_group.tokens[QUOTE_INDEX].root_bank;
    let root_bank = &mut test.load_account::<RootBank>(root_bank_pk).await;
    let node_bank_pk = root_bank.node_banks[0];
    let node_bank = test.load_account::<NodeBank>(node_bank_pk).await;

    let deposit_iou_account = spl_associated_token_account::get_associated_token_address(
        &test.users[0].pubkey(),
        &deposit_iou_mint,
    );

    let instructions = vec![Instruction {
        program_id: test.dasheri_program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::IouDepositIntoMangoAccount {
                mango_program: test.mango_program_id,
                mango_group: mango_group_cookie.address,
                mango_cache: mango_group.mango_cache.key(),
                root_bank: root_bank_pk,
                node_bank: node_bank_pk,
                node_bank_vault: node_bank.vault.key(),
                owner_token_account: test.token_accounts[15].key(),
                mango_account: mango_group_cookie.mango_accounts[0].address,
                deposit_iou_account,
                deposit_iou_mint,
                token_mint: test.mints[test.mints.len() - 1].pubkey.unwrap(),
                payer: test.users[0].pubkey(),
                gateway,
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
                associated_token_program: spl_associated_token_account::id(),
                rent: solana_program::sysvar::rent::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(
            &dasheri::instruction::IouDepositIntoMangoAccount { quantity: 100 },
        ),
    }];
    &mut test
        .process_transaction(
            &instructions,
            Some(&[&Keypair::from_base58_string(
                &test.users[0].to_base58_string(),
            )]),
        )
        .await
        .unwrap();
}
