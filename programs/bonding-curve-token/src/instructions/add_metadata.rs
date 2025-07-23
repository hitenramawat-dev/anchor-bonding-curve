use {
    crate::BondingState, anchor_lang::{accounts::signer, prelude::*}, anchor_spl::{
        metadata::{
            create_metadata_accounts_v3, mpl_token_metadata::types::{Creator, DataV2},
            CreateMetadataAccountsV3, Metadata,
        },
        token::{Mint, Token},
    }
};




#[derive(Accounts)]
pub struct AddMetadata<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"bonding_curve", payer.key().as_ref(), mint_account.key().as_ref()],
        bump
    )]
    pub bonding_pda:Account<'info,BondingState>,

    /// CHECK: The meta data account 
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn process_add_meta_data(
    ctx:Context<AddMetadata>, 
    token_name: String,
    token_symbol: String,
    token_uri: String,) 
    -> Result<()> 
    {
        msg!("Creating metadata account");

        let signer_key = ctx.accounts.payer.key();
        let mint_key = ctx.accounts.mint_account.key();
        
        let seeds:&[&[u8]] = &[
            b"bonding_curve",
            signer_key.as_ref(),
            mint_key.as_ref(),
            &[ctx.bumps.bonding_pda],
        ];
        
        let signer_seeds = &[seeds];

        let cpi_context  = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(), 
                CreateMetadataAccountsV3{
                    metadata:ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    mint_authority: ctx.accounts.bonding_pda.to_account_info(),
                    update_authority: ctx.accounts.bonding_pda.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                signer_seeds
            );
            
        create_metadata_accounts_v3(
            cpi_context,
            DataV2 { 
                name: token_name , 
                symbol: token_symbol, 
                uri: token_uri, 
                seller_fee_basis_points: 0, 
                creators: Some(vec![
                    Creator{
                        address:ctx.accounts.payer.key(),
                        verified:false,
                        share:100,
                    }
                ]) , 
                collection: None, 
                uses:None, 
            } ,
             true, 
             true,
             None)?;

        Ok(())
    }

