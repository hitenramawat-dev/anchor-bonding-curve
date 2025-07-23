use anchor_lang::prelude::*;

declare_id!("EpTZy8wg9YC6jerepiebigRXgBSGg3HNH6jBm4gbHj25");

pub mod instructions;
pub use instructions::*;


pub mod state;
pub use state::*;


pub mod constants;
pub use constants::*;


pub mod error;
pub use error::*;

#[program]
pub mod bonding_curve_token {
    use super::*;

    pub fn initialize_curve(ctx: Context<Curve>,supply:u64) -> Result<()> {
       process_initialize_curve(ctx, supply);
        Ok(())
    }

    pub fn sell_tokens(ctx: Context<SellToken>,creator: Pubkey,token:u64,min_sol_out:u64) -> Result<()> {
        process_sell_tokens(ctx,creator,token,min_sol_out);
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyToken>,sol_amount:u64,min_tokens_out:u64) -> Result<()> {
        process_buy_tokens(ctx, sol_amount,min_tokens_out);
        Ok(())
    }

    pub fn add_metadata(ctx: Context<AddMetadata>,
        token_name: String,
        token_symbol: String,
        token_uri: String,) 
        -> Result<()> {
            process_add_meta_data(ctx, token_name, token_symbol, token_uri);
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}
