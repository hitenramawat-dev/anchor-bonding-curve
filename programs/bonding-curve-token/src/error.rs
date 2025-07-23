use anchor_lang::prelude::*;



#[error_code]
pub enum BondingErrors {
    #[msg("maths overflow")]
    MathOverFlow,
    #[msg("maths division by 0")]
    DivisionByZero,
    #[msg("maths underflow")]
    MathUnderflow,
    #[msg("you can't mint more than 100M")]
    InvalidInitialMintAmount,
    #[msg("Invalid range for integration")]
    InvalidRange,
    #[msg("invalid token amount")]
    InvalidTokenAmount,
    #[msg("Insufficient supply")]
    InsufficientSupply,
    #[msg("Don't have enough sol ")]
    InsufficientSOlSupply,
    #[msg("You aint the creator motherfucker tryna fuck with me ")]
    InvalidMintAuthority,
    #[msg("Slippage problem token")]
    SlippageExceededToken,
    #[msg("Slippage problem sol")]
    SlippageExceededSol,
    #[msg("tryna steal")]
    InvalidCreator,
}