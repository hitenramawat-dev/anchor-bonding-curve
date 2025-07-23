use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::{BondingErrors, BondingState};

#[derive(Accounts)]
pub struct BuyToken<'info>{

    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(mut)]
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
        mut,
        seeds = [b"vault_for_sol",bonding_pda.key().as_ref()],
        bump
    )]
    pub vault:UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"bonding_curve", signer.key().as_ref(), mint_account.key().as_ref()],
        bump
    )]
    pub bonding_pda:Account<'info,BondingState>,

    ///  CHECK: This PDA store the transation fee bitches!!
    #[account(
        mut,
        seeds = [b"fee_vault", bonding_pda.key().as_ref()],
        bump
    )]
    pub fee_pda:UncheckedAccount<'info>,
    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>

}


pub fn process_buy_tokens(ctx:Context<BuyToken>,sol_amount:u64,min_tokens_out:u64) -> Result<()> {

    let bonding_account = &mut ctx.accounts.bonding_pda;
    let fee_amount = (sol_amount * bonding_account.fee_rate as u64) / 10_000;
    let sol_for_tokens  = sol_amount  - fee_amount;



    //calculate the token in exchange with sol
    let tokens_user_get = calculate_tokens_from_sol_amount(
        bonding_account,
        bonding_account.total_supply,
        sol_for_tokens
    )?;

    require!(min_tokens_out >= min_tokens_out,BondingErrors::SlippageExceededToken);


    // send feeAmount to fee_vault
    let cpi_context_0 = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        Transfer{
            from:ctx.accounts.signer.to_account_info(),
            to:ctx.accounts.fee_pda.to_account_info()
        }
    );

    transfer(cpi_context_0, fee_amount)?;



    // send remaining to vault
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        Transfer {
            from:ctx.accounts.signer.to_account_info(),
            to:ctx.accounts.vault.to_account_info(),
        }
    );

    transfer(cpi_context,sol_for_tokens)?;

    msg!("Buying with {} SOL (fee: {})", sol_amount, fee_amount);
    
    let mint_key = ctx.accounts.mint_account.key();

    let seeds:&[&[u8]] = &[
        b"bonding_curve",
        bonding_account.creator.as_ref(),
        mint_key.as_ref(),
        &[ctx.bumps.bonding_pda]
    ];

    let signer_seed = &[seeds];

    // give tokens to user
    let cpi_context_2 = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), MintTo{
            mint:ctx.accounts.mint_account.to_account_info(),
            to:ctx.accounts.associated_token_account.to_account_info(),
            authority: bonding_account.to_account_info()
        }, signer_seed);

        mint_to(cpi_context_2, tokens_user_get)?;


        msg!("Minting {} tokens to user", tokens_user_get);

    // user--> vault
    // mint --> user
    bonding_account.total_supply += tokens_user_get;
    bonding_account.sol_reserves += sol_for_tokens;
    bonding_account.fees_collected += fee_amount;

    Ok(())
}




/// Calculate how many tokens to mint given SOL amount
fn calculate_tokens_from_sol_amount(
    bonding_curve: &BondingState,
    current_supply: u64,
    sol_amount: u64,
) -> Result<u64> {
    // We need to solve: ∫[current_supply to new_supply] price(x) dx = sol_amount
    // This requires iterative solving since we can't analytically invert the integral
    
    // Binary search for the answer
    let mut low = 1u64;
    let mut high = 1_000_000_000 * 1_000_000u64; // 1B tokens max search range
    let target_sol = sol_amount;
    
    for _ in 0..50 { // Max 50 iterations
        let mid = (low + high) / 2;
        let new_supply = current_supply + mid;
        
        let calculated_sol = bonding_curve.calculate_integral(current_supply, new_supply)?;
        
        if calculated_sol == target_sol {
            return Ok(mid);
        } else if calculated_sol < target_sol {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
        
        // Close enough?
        if high <= low || (calculated_sol as i64 - target_sol as i64).abs() < 1000 {
            return Ok(mid);
        }
    }
    
    Ok((low + high) / 2)
}
