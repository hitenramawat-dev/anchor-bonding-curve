use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

use crate::{BondingErrors, BondingState};



#[derive(Accounts)]
#[instruction(creator: Pubkey)]
pub struct SellToken<'info> {

    #[account(mut)]
    pub signer:Signer<'info>,
//signer
//mintAccount
//vault
//bondingPda
//ataAccountUser
    #[account(mut)]
    pub mint_account:Account<'info,Mint>,

     /// CHECK: This PDA is only used to store SOL, no data is read or written
     #[account(
        mut,
        seeds = [b"vault_for_sol",bonding_pda.key().as_ref()],
        bump
    )]
    pub vault:UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"bonding_curve", creator.as_ref(), mint_account.key().as_ref()],
        bump,
        constraint = bonding_pda.creator == creator @ BondingErrors::InvalidCreator
    )]
    pub bonding_pda:Account<'info,BondingState>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = signer,
    )]
    pub ata_account_user: Account<'info,TokenAccount>,

    ///  CHECK: This PDA store the transation fee bitches!!
    #[account(
        mut,
        seeds = [b"fee_vault", bonding_pda.key().as_ref()],
        bump
    )]
    pub fee_pda:UncheckedAccount<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
}



pub fn process_sell_tokens(ctx:Context<SellToken>,creator: Pubkey,token:u64,min_sol_out:u64) -> Result<()> {
    
    let bonding_account = &mut ctx.accounts.bonding_pda;
    let cretor_pubkey = creator;
    require!(token > 0,BondingErrors::InvalidTokenAmount);
    require!(bonding_account.total_supply >= token,BondingErrors::InsufficientSupply);

    
    // how much sols to return ??????????????? boi don't sell my token 😟😟    
    let current_supply = bonding_account.total_supply;
    let new_supply = current_supply - token;


    let sol_to_return = bonding_account.calculate_integral(new_supply, current_supply)?;


    let fee_amount = (sol_to_return * bonding_account.fee_rate as u64) / 10_000;
    let sol_after_fees = sol_to_return - fee_amount ; 


    require!(bonding_account.sol_reserves >= sol_after_fees,BondingErrors::InsufficientSOlSupply);

    require!(sol_after_fees >= min_sol_out,BondingErrors::SlippageExceededSol);


   // Burn  baby  Burn tokensssssssssssssssss
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        Burn{
            mint:ctx.accounts.mint_account.to_account_info(),
            from:ctx.accounts.ata_account_user.to_account_info(),
            authority:ctx.accounts.signer.to_account_info(),
        }
    );

    burn(cpi_context, token)?;



    //[b"vault_for_sol",bonding_pda.key().as_ref()]

    let bonding_key = bonding_account.key();

    let seeds:&[&[u8]] = &[
        b"vault_for_sol",
        bonding_key.as_ref(),
        &[ctx.bumps.vault]
    ];
// seeds = [b"vault_for_sol",bonding_pda.key().as_ref()],
    let signer_seeds = &[seeds];


    let cpi_context_2 = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(), 
        Transfer {
            from:ctx.accounts.vault.to_account_info(),
            to:ctx.accounts.signer.to_account_info()
        }, signer_seeds);

    
    transfer(cpi_context_2,sol_after_fees)?;


    Ok(())
}
