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
            &context.solana.context.borrow_mut().payer.pubkey().as_ref(),
            &ACCOUNT_NUM.to_le_bytes(),
        ],
        &context.mango.program_id,
    );
    let instructions = vec![Instruction {
        program_id: context.dasheri.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &dasheri::accounts::CreateMangoAccount {
                mango_program_ai: context.mango.program_id,
                mango_group: mango_group_cookie.address,
                mango_account: mango_account,
                owner: context.solana.context.borrow_mut().payer.pubkey(),
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
}
