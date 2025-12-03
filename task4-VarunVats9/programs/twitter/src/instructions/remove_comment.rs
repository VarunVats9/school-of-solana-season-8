use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use crate::states::*;

pub fn remove_comment(ctx: Context<RemoveCommentContext>) -> Result<()> {
    // The removal logic is handled by close and has_one constraints
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        mut,
        close = comment_author,
        seeds = [
            COMMENT_SEED.as_bytes(), 
            comment_author.key().as_ref(),
            &{
                let content_hash = hash(comment.content.as_bytes()).to_bytes();
                content_hash
            },
            comment.parent_tweet.as_ref()
        ],
        bump = comment.bump,
        has_one = comment_author
    )]
    pub comment: Account<'info, Comment>,
}
