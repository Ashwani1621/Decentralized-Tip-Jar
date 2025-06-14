use anchor_lang::prelude::*;

declare_id!("45DArH4jbdaY8XjmgpZKpkp7DivBaJv17kU7YxMFL1K9");

#[program]
pub mod decentralized_tip_jar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
