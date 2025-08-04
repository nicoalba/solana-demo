use anchor_lang::prelude::*;

declare_id!("HD3sxGps2pr36KvHZY4JjaQgG9otncF9SCovGvTvhvdp");

#[program]
pub mod solana_test_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
