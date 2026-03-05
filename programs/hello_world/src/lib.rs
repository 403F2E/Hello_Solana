use anchor_lang::prelude::*;

declare_id!("CSM5fTZVx7WvjwBWnATbAQmHajba2MtShWmuFncnUJTp");

#[program]
pub mod hello_world {
    use super::*;

    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello world from 403f2e");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}
