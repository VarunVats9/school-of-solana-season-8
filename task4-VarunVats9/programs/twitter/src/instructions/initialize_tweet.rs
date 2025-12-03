//-------------------------------------------------------------------------------
///
/// TASK: Implement the initialize tweet functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that topic and content don't exceed maximum lengths
/// - Initialize a new tweet account with proper PDA seeds
/// - Set tweet fields: topic, content, author, likes, dislikes, and bump
/// - Initialize counters (likes and dislikes) to zero
/// - Use topic in PDA seeds for tweet identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::system_program;

use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
) -> Result<()> {
    if topic.len() > TOPIC_LENGTH {
        return err!(TwitterError::TopicTooLong);
    }
    if content.len() > CONTENT_LENGTH {
        return err!(TwitterError::ContentTooLong);
    }

    let tweet_account = &mut ctx.accounts.tweet;
    tweet_account.topic = topic;
    tweet_account.content = content;
    tweet_account.tweet_author = ctx.accounts.tweet_authority.key();
    tweet_account.likes = 0;
    tweet_account.dislikes = 0;
    tweet_account.bump = ctx.bumps.tweet;

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct InitializeTweet<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub tweet_authority: Signer<'info>,
    #[account(
        init,
        payer = tweet_authority,
        space = 8 + // discriminator
                32 + // author pubkey
                4 + TOPIC_LENGTH + // topic String (4 bytes for length + max bytes)
                4 + CONTENT_LENGTH + // content String (4 bytes for length + max bytes)
                8 + // likes u64
                8 + // dislikes u64
                1, // bump
        seeds = [topic.as_bytes(), TWEET_SEED.as_bytes(), tweet_authority.key().as_ref()],
        bump
    )]
    pub tweet: Account<'info, Tweet>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}
