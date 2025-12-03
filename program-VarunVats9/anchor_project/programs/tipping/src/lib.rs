use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("CeA7jNGCbhQvhAcWPceXNQtf13wm3oNiFtfD6tdU92PV");

#[program]
pub mod tipping {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.total_tips_sent = 0;
        user_stats.total_tips_received = 0;
        user_stats.bump = ctx.bumps.user_stats;
        Ok(())
    }

    pub fn tip(ctx: Context<Tip>, amount: u64) -> Result<()> {
        let signer = &ctx.accounts.signer;
        let recipient = &ctx.accounts.recipient;
        
        // Transfer SOL
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: signer.to_account_info(),
                to: recipient.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        // Update stats
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.total_tips_sent = user_stats.total_tips_sent.checked_add(amount).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 8 + 1, // discriminator + u64 + u64 + u8
        seeds = [b"user-stats", signer.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: We are just transferring SOL to this account
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"user-stats", signer.key().as_ref()],
        bump = user_stats.bump,
    )]
    pub user_stats: Account<'info, UserStats>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserStats {
    pub total_tips_sent: u64,
    pub total_tips_received: u64,
    pub bump: u8,
}
