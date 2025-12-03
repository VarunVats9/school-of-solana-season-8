//-------------------------------------------------------------------------------
///
/// TASK: Implement the add comment functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that comment content doesn't exceed maximum length
/// - Initialize a new comment account with proper PDA seeds
/// - Set comment fields: content, author, parent tweet, and bump
/// - Use content hash in PDA seeds for unique comment identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use anchor_lang::system_program;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    if comment_content.len() > COMMENT_LENGTH {
        return err!(TwitterError::CommentTooLong);
    }
    
    let comment_account = &mut ctx.accounts.comment;
    comment_account.content = comment_content;
    comment_account.comment_author = ctx.accounts.comment_author.key();
    comment_account.parent_tweet = ctx.accounts.tweet.key();
    comment_account.bump = ctx.bumps.comment;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        init,
        payer = comment_author,
        space = 8 + // discriminator
               32 + // author pubkey
               32 + // parent tweet pubkey
               4 + COMMENT_LENGTH + // comment content (4 bytes for string length)
               1, // bump
        seeds = [
            COMMENT_SEED.as_bytes(), 
            comment_author.key().as_ref(), 
            &{
                let content_hash = hash(comment_content.as_bytes()).to_bytes();
                content_hash
            },
            tweet.key().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
