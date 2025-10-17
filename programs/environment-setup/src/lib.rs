use anchor_lang::prelude::*;

declare_id!("66ipm2FJeogTcLvJCUz15v1Zq2CzV3bD6zv2mjFYJ8Bb");

#[program]
pub mod environment_setup {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from Bit-cipher: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
