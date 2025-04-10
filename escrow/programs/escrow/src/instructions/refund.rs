use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        TokenAccount,
        TokenInterface,
        Mint,
        TransferChecked,
       transfer_checked
    }
};
use crate::state::Escrow;
#[derive(Accounts)]
pub struct Refund<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,
    #[account(
        mint::token_program=token_program,
    )]
    pub mint_a:InterfaceAccount<'info,Mint>,
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program
    )]
    pub maker_ata_a:InterfaceAccount<'info,TokenAccount>,
    #[account(
        mut,
        close=maker,
        has_one=mint_a,
        has_one=maker,
        seeds=[b"escrow",maker.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,

    )]
    pub escrow:Account<'info,Escrow>,
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program:Interface<'info,AssociatedToken>,
    pub system_program:Interface<'info,System>,
    
    pub token_program:Interface<'info,TokenInterface>,

}
impl <'info>Refund<'info>{
    pub fn refund_and_close_vault(&mut self)->Result<()>{
        let signer_seeds:[&[&[u8]];1]=[
            &[
                b"escrow",
                self.maker.to_account_info().key.as_ref(),
                self.escrow.seed.to_le_bytes().as_ref(),
                &[self.escrow.bump]
            ]
        ];
        
        let transfer_accounts=TransferChecked{
            from:self.vault.to_account_info(),
            mint:self.mint_a.to_account_info(),
            to:self.maker_ata_a.to_account_info(),
            authority:self.escrow.to_account_info(),
        
        };
        let cpi_ctx=CpiContext::new_with_signer(self.token_program.to_account_info(),transfer_accounts,&signer_seeds);
        transfer_checked(cpi_ctx,self.vault.amount,self.mint_a.decimals)?;
        let close_accounts=CloseAccount{
            account:self.vault.to_account_info(),
            destination:self.maker.to_account_info(),
            authority:self.escrow.to_account_info(),
        };
        let close_cpi_ctx=CpiContext::new_with_signer(self.token_program.to_account_info(),close_accounts,  &signer_seeds);

      Ok(())

    }
}