use anchor_lang::prelude::*;

/* This is a very basic Rust hello-world program for Solana using the Anchor framework. It doesn't stsore any state
state or perform any complex operation; it simply logs a greeting message when the `initialize` function is called.
*/

declare_id!("EqYLJzQSwpqLa1ByR43TjARd8sxEsyaYnM8mGEGAWmg1");

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
