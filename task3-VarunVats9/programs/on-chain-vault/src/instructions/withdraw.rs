//-------------------------------------------------------------------------------
///
/// TASK: Implement the withdraw functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::VaultError;
use crate::events::WithdrawEvent;
use crate::state::Vault;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        has_one = vault_authority,
        seeds = [b"vault", vault_authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub vault_authority: Signer<'info>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(!ctx.accounts.vault.locked, VaultError::VaultLocked);

    let vault_info = ctx.accounts.vault.to_account_info();
    let authority_info = ctx.accounts.vault_authority.to_account_info();

    let vault_balance = vault_info.lamports();
    require!(vault_balance >= amount, VaultError::InsufficientBalance);

    **vault_info.try_borrow_mut_lamports()? -= amount;
    **authority_info.try_borrow_mut_lamports()? += amount;

    emit!(WithdrawEvent {
        vault: ctx.accounts.vault.key(),
        vault_authority: ctx.accounts.vault.vault_authority,
        amount,
    });

    Ok(())
}