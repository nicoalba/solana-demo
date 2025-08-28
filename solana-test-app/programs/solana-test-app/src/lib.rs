use anchor_lang::prelude::*;

/* This is a very basic Rust hello-world program for Solana using the Anchor framework. It doesn't stsore any state
state or perform any complex operation; it simply logs a greeting message when the `initialize` function is called.
*/

declare_id!("EqYLJzQSwpqLa1ByR43TjARd8sxEsyaYnM8mGEGAWmg1"); // program ID

#[program] // module declaration
pub mod solana_test_app { // public module named solana_test_app
    use super::*; // imports all items from the parent module

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> { // defines a public function that takes a context parameter of type `Context<Initialize>` and returns a `Result<()>`.
        msg!("Greetings from: {:?}", ctx.program_id); // logs a greeting message that includes the program ID
        Ok(())  // indicates successful completion of the function
    }
}

#[derive(Accounts)] // derives the `Accounts` trait for the `Initialize` struct
pub struct Initialize {} // defines an empty struct named `Initialize` that serves as a context for the `initialize` function
