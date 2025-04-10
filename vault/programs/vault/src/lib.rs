use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer,transfer};

declare_id!("5EA4Nf4FFnETFquVwf4qBduGzqLuRtRP1r2MCC81zukp");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
       ctx.accounts.initialize(&ctx.bumps);
       Ok(())
    }
    pub fn deposit(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount);
        Ok(())
    } 
    pub fn withdraw(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.withdraw(amount);
        Ok(())
    }
    pub fn closeAccount(ctx:Context<CloseAccount>)->Result<()>{
        ctx.accounts.close();
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(init,payer=user,space=8+1+1,seeds=[b"state",user.key().as_ref()],bump)]
    pub state:Account<'info,Vault_State>,
    #[account(seeds=[b"vault",state.key().as_ref()],bump)]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>,



}
impl <'info>Initialize<'info>{
    pub fn initialize(&mut self,bumps:&InitializeBumps)->Result<()>{
        self.state.vault_bump=bumps.vault;
        self.state.state_bump=bumps.state;

        Ok(())

    }
   

}
#[account]
pub struct Vault_State{
    pub vault_bump:u8,
    pub state_bump:u8

}
#[derive(Accounts)]
pub struct Payment<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(seeds=[b"state",user.key().as_ref()],bump=vault_state.state_bump)]
    pub state:Account<'info,Vault_State>,
    pub vault_state:Account<'info,Vault_State>,
    #[account(
        mut,
        seeds=[b"vault",state.key().as_ref()],
        bump=vault_state.vault_bump
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>

}
impl <'info>Payment<'info>{
    pub fn deposit(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info()
        };
        let cpi_ctx=CpiContext::new(cpi_program,cpi_accounts);
        transfer(cpi_ctx,amount);
        Ok(())
    }
    pub fn withdraw(&mut self,amount:u64)->Result<()>{
        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info()
        };
        let user_key = self.user.key();
         
        let seeds =& [
            b"vault",
            user_key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        transfer(cpi_ctx,amount);
        Ok(())
    }
}
#[derive(Accounts)]
pub struct CloseAccount<'info>{
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state:Account<'info,Vault_State>,
    pub vault_state:Account<'info,Vault_State>,
    #[account(
        mut,
        seeds=[b"vault",state.key().as_ref()],
        bump=vault_state.vault_bump
        
    )]
    pub vault:SystemAccount<'info>,
    pub system_program:Program<'info,System>
}
impl<'info>CloseAccount<'info>{
    pub fn close(&mut self)->Result<()>{
        let cpi_program=self.system_program.to_account_info();
        let cpi_accounts=Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info()
        };
        let user_key = self.user.key();
         
        let seeds =& [
            b"vault",
            user_key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];
        let cpi_ctx=CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        transfer(cpi_ctx,self.vault.lamports())?;
        Ok(())
    }
}