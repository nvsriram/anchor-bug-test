use anchor_lang::prelude::*;

declare_id!("8tn1PnNDZnvu6okWXmrci2VL8kStTEYoKzsy9vKCi7kf");

#[program]
pub mod anchor_bug_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        ctx.accounts.pda_account.set_inner(MyData {
            bump: ctx.bumps.pda_account,
            data,
        });
        Ok(())
    }

    pub fn set_data(ctx: Context<SetData>, data: u64) -> Result<()> {
        ctx.accounts.pda_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space= 8 + MyData::INIT_SPACE, seeds=[b"init-seed", signer.key().as_ref()], bump)]
    pub pda_account: Account<'info, MyData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut, seeds=[b"init-seed", signer.key().as_ref()], bump=pda_account.bump)]
    pub pda_account: Account<'info, MyData>,
    pub signer: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct MyData {
    bump: u8,
    data: u64,
}
