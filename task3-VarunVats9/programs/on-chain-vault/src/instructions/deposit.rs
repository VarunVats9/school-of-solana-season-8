//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::system_program;
use anchor_lang::prelude::*;

use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    if ctx.accounts.vault.locked {
        return err!(VaultError::VaultLocked);
    }

    let user_lamports = ctx.accounts.user.lamports();
    if user_lamports < amount {
        return err!(VaultError::InsufficientBalance);
    }

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to:   ctx.accounts.vault.to_account_info(),
            },
        ),
        amount,
    )?;

    emit!(DepositEvent {
        vault: ctx.accounts.vault.key(),
        user:  ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}