//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
/// 
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::events::ToggleLockEvent;
use crate::state::Vault;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    /// The vault whose lock state will be toggled
    #[account(mut, has_one = vault_authority)]
    pub vault: Account<'info, Vault>,

    /// The authorized signer (must match vault.vault_authority)
    pub vault_authority: Signer<'info>,
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    let vault = &mut ctx.accounts.vault;

    // Toggle the lock state
    vault.locked = !vault.locked;

    // Emit event with new state
    emit!(ToggleLockEvent {
        vault: vault.key(),
        vault_authority: ctx.accounts.vault_authority.key(),
        locked: vault.locked,
    });

    Ok(())
}