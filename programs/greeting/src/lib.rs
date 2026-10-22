use anchor_lang::prelude::*;

declare_id!("AbTHMxTWAQrktofK8K723h9PhwesT9MZHCTfDfQJSBkL");

pub const MAX_GREETING_BYTES: usize = 200;
const GREETING_SEED: &[u8] = b"greeting";

#[program]
pub mod greeting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greeting: String) -> Result<()> {
        validate_greeting(&greeting)?;

        let state = &mut ctx.accounts.greeting_state;
        state.authority = ctx.accounts.authority.key();
        state.bump = ctx.bumps.greeting_state;
        state.message = greeting;
        msg!("Greeting initialized: {}", state.message);
        Ok(())
    }

    pub fn set_greeting(ctx: Context<SetGreeting>, greeting: String) -> Result<()> {
        validate_greeting(&greeting)?;

        ctx.accounts.greeting_state.message = greeting;
        msg!("Greeting updated: {}", ctx.accounts.greeting_state.message);
        Ok(())
    }

    pub fn close_greeting(_ctx: Context<CloseGreeting>) -> Result<()> {
        msg!("Greeting closed and rent recovered.");
        Ok(())
    }
}

fn validate_greeting(greeting: &str) -> Result<()> {
    require!(!greeting.trim().is_empty(), GreetingError::EmptyGreeting);
    require!(
        greeting.as_bytes().len() <= MAX_GREETING_BYTES,
        GreetingError::GreetingTooLong
    );
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GreetingState::SPACE,
        seeds = [GREETING_SEED, authority.key().as_ref()],
        bump
    )]
    pub greeting_state: Account<'info, GreetingState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetGreeting<'info> {
    #[account(
        mut,
        seeds = [GREETING_SEED, authority.key().as_ref()],
        bump = greeting_state.bump,
        has_one = authority
    )]
    pub greeting_state: Account<'info, GreetingState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseGreeting<'info> {
    #[account(
        mut,
        seeds = [GREETING_SEED, authority.key().as_ref()],
        bump = greeting_state.bump,
        has_one = authority,
        close = authority
    )]
    pub greeting_state: Account<'info, GreetingState>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct GreetingState {
    pub authority: Pubkey,
    pub bump: u8,
    pub message: String,
}

impl GreetingState {
    pub const SPACE: usize = 8 + 32 + 1 + 4 + MAX_GREETING_BYTES;
}

#[error_code]
pub enum GreetingError {
    #[msg("Greeting cannot be empty.")]
    EmptyGreeting,
    #[msg("Greeting must be 200 bytes or fewer.")]
    GreetingTooLong,
}
