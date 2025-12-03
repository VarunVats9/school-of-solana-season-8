//-------------------------------------------------------------------------------
///
/// TASK: Implement the add reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Initialize a new reaction account with proper PDA seeds
/// - Increment the appropriate counter (likes or dislikes) on the tweet
/// - Set reaction fields: type, author, parent tweet, and bump
/// - Handle both Like and Dislike reaction types
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction_type: ReactionType) -> Result<()> {
    let reaction_account = &mut ctx.accounts.tweet_reaction;
    
    // Set reaction account fields
    reaction_account.reaction_author = ctx.accounts.reaction_author.key();
    reaction_account.parent_tweet = ctx.accounts.tweet.key();
    reaction_account.reaction = reaction_type.clone();
    reaction_account.bump = ctx.bumps.tweet_reaction;

    // Update tweet counters based on reaction type
    let tweet_account = &mut ctx.accounts.tweet;
    match reaction_type {
        ReactionType::Like => {
            tweet_account.likes = tweet_account.likes.checked_add(1).ok_or(TwitterError::MaxLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet_account.dislikes = tweet_account.dislikes.checked_add(1).ok_or(TwitterError::MaxDislikesReached)?;
        }
    }

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        init,
        payer = reaction_author,
        space = 8 + // discriminator
               32 + // author pubkey
               32 + // parent tweet pubkey
               1 + // reaction type
               1, // bump
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
