use anchor_lang::prelude::*;

declare_id!("DzryzuZmt3y9qVsLzDMhZ6AHsarqhEDso5f34uCGvpaw");

#[program]
pub mod anchor_example {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;
    pub fn create_pool(ctx: Context<CreatePool>) -> ProgramResult {
        let pool = &mut ctx.accounts.pool_account;
        pool.id = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(init, payer = user, owner = user.key(), space = 8 + Pool::INIT_SPACE )]
    pub pool_account: Account<'info, Pool>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub id: u64,
}
