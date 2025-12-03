//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    let reaction_account = &ctx.accounts.tweet_reaction;
    let tweet_account = &mut ctx.accounts.tweet;
    
    match reaction_account.reaction {
        ReactionType::Like => {
            tweet_account.likes = tweet_account.likes.checked_sub(1).ok_or(TwitterError::MinLikesReached)?;
        }
        ReactionType::Dislike => {
            tweet_account.dislikes = tweet_account.dislikes.checked_sub(1).ok_or(TwitterError::MinDislikesReached)?;
        }
    }
    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        mut,
        close = reaction_author,
        has_one = reaction_author,
        seeds = [TWEET_REACTION_SEED.as_bytes(), reaction_author.key().as_ref(), tweet.key().as_ref()],
        constraint = tweet_reaction.parent_tweet == tweet.key(),
        bump = tweet_reaction.bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
