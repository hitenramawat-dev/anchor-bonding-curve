use anchor_lang::prelude::*;

use crate::BondingErrors;




#[account]
#[derive(InitSpace)]
pub struct BondingState {
    pub creator:Pubkey,
    pub mint:Pubkey,
    pub vault:Pubkey,
    pub total_supply: u64,
    pub sol_reserves:u64,
    pub k1:u64,
    pub k2:u64,
    pub offset:u64,
    pub fee_rate:u16,
    pub fees_collected:u64,
    pub bump:u8,
    pub vault_bump: u8,
    pub fee_vault_bump: u8,
}


impl BondingState {
   pub  fn calculate_integral(&self, x1: u64, x2: u64) -> Result<u64> {
        require!(x2 > x1, BondingErrors::InvalidRange);
        
        // Convert to f64 for calculation, then back to u64
        let x1_f = x1 as f64 / 1_000_000.0; // Convert from microTokens to tokens
        let x2_f = x2 as f64 / 1_000_000.0;
        let a_f = self.k1 as f64;
        let b_f = self.k2 as f64;
        let c_f = self.offset as f64;

        // Calculate: a*(x2-x1) - b*ln((c+x2)/(c+x1))
        let linear_part = a_f * (x2_f - x1_f);
        let log_part = b_f * ((c_f + x2_f) / (c_f + x1_f)).ln();
        
        let result = linear_part - log_part;
        
        // Convert back to lamports
        Ok((result as u64).max(1)) // Ensure non-zero
    }
}

