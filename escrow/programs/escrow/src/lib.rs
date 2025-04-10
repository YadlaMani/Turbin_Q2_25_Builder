#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod instructions::*;
pub mod state;


declare_id!("HswXvDp2pd68ZofaXkcp2Mo64ZBXDSYapEcVCibZ2ahV");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>,seed:u64,deposit:u64,bumps:MakeBumps) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {

}
