use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::BondingState;

#[derive(Accounts)]
pub struct MintMe<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"bonding_curve", bonding_pda.creator.as_ref(), mint.key().as_ref()],
        bump
    )]
    pub bonding_pda: Account<'info, BondingState>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = bonding_pda.creator,
    )]
    pub ata_mines: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}



pub fn mint_to_me(ctx:Context<MintMe>,mines:Pubkey) -> Result<()> {

        

    Ok(())
}