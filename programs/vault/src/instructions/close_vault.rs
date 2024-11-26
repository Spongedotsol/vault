use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct CloseVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // 管理 vault 的人

    #[account(
        mut,
        seeds = [b"vault_state", authority.key().as_ref()],
        bump,
        close = authority // rent 還給 authority
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseVault>) -> Result<()> {
    let vault_state = &ctx.accounts.vault_state;

    // 檢查是否還有用戶
    require!(
        vault_state.total_users == 0,
        ErrorCode::VaultNotEmpty
    );

    Ok(())
}

// close vault 轉移資金