use anchor_lang::{prelude::*, solana_program::program_option::COption};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{BondingErrors, BondingState, K1, K2, OFFSET};



#[derive(Accounts)]
pub struct Curve<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,              
   
    #[account(
        init,
        payer = signer,
        space = 8 + BondingState::INIT_SPACE,
        seeds = [b"bonding_curve", signer.key().as_ref(), mint_account.key().as_ref()],
        bump
    )]
    pub bonding_pda: Account<'info,BondingState>,


    #[account(
        init,
        payer = signer,
        mint::decimals = 9,
        mint::authority = bonding_pda.key(),
        mint::freeze_authority = bonding_pda.key(),
    )]
    pub mint_account:Account<'info,Mint>,


    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_account,
        associated_token::authority = signer
    )]
    pub associated_token_account:Account<'info,TokenAccount>,

   /// CHECK: This PDA is only used to store SOL, no data is read or written
    #[account(
        init,
        payer = signer,
        space = 0,
        seeds = [b"vault_for_sol",bonding_pda.key().as_ref()],
        bump
    )]
    pub vault:UncheckedAccount<'info>,

    ///  CHECK: This PDA store the transation fee bitches!!
    #[account(
        init,
        payer = signer,
        space = 0,
        seeds = [b"fee_vault", bonding_pda.key().as_ref()],
        bump
    )]
    pub fee_vault: UncheckedAccount<'info>,


    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>

}


pub fn process_initialize_curve(ctx:Context<Curve>,supply:u64) -> Result<()> {


    let bonding_curve = &mut ctx.accounts.bonding_pda;

    require!(supply > 0 && supply <= 100_000_000 * 10_u64.pow(6),BondingErrors::InvalidInitialMintAmount);


    require!(ctx.accounts.mint_account.mint_authority == COption::Some(bonding_curve.key()) ,BondingErrors::InvalidMintAuthority);


    let signer_key = ctx.accounts.signer.key();
    let mint_key = ctx.accounts.mint_account.key();
    
    let seeds:&[&[u8]] = &[
        b"bonding_curve",
        signer_key.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.bonding_pda],
    ];
    
    let signer_seeds = &[seeds];
    


    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        MintTo {
            mint:ctx.accounts.mint_account.to_account_info(),
            to:ctx.accounts.associated_token_account.to_account_info(),
            authority:bonding_curve.to_account_info(),
        }, 
        signer_seeds);

    mint_to(cpi_context, supply )?;



    bonding_curve.creator = ctx.accounts.signer.key();
    bonding_curve.mint  = ctx.accounts.mint_account.key();
    bonding_curve.vault = ctx.accounts.vault.key();


    bonding_curve.total_supply = supply;
    bonding_curve.sol_reserves = 0;

    bonding_curve.k1 = K1 ;
    bonding_curve.k2 = K2;
    bonding_curve.offset = OFFSET;
    bonding_curve.fee_rate = 250;
    bonding_curve.fees_collected = 0;
    bonding_curve.bump = ctx.bumps.bonding_pda;

    msg!(
        "Bonding curve initialized! Creator minted {} tokens for free. Next mints require SOL payment.",
        supply
    );
    Ok(())
}


pub fn process_calulate_token_price(bonding_state:&BondingState,supply:u64) -> Result<u64> {

    let denominator = bonding_state.offset.checked_add(supply)
        .ok_or(BondingErrors::MathOverFlow)?;

    let division_result = bonding_state.k2.checked_div(denominator)
        .ok_or(BondingErrors::DivisionByZero)?;

    let price_per_token = bonding_state.k1.checked_add(division_result)
        .ok_or(BondingErrors::MathUnderflow)?;

    Ok(price_per_token)
}

