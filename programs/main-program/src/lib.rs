use anchor_lang::prelude::*;

declare_id!("9o9jDgtHAW2npTWDRGn5EFTpa6ALhivvjsx1Hj8JzxTH");

pub mod error;

#[program]
pub mod main_program {
    use error::MainProgramError;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        if ctx.accounts.pda_account.data > 10 {
            msg!("success");
        } else {
            msg!("error");
            return Err(MainProgramError::NotMoreThan10.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(owner = owner_program.key())]
    pub pda_account: Account<'info, MyData>,
    /// CHECK: ignore
    pub owner_program: UncheckedAccount<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct MyData {
    bump: u8,
    data: u64,
}
