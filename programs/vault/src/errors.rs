use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds for withdrawal.")]
    InsufficientFunds,

    #[msg("The vault still has users; cannot close.")]
    VaultNotEmpty,
}
