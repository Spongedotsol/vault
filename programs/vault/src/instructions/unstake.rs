use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_state", user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Unstake>, lamports: u64) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    let user_state = &mut ctx.accounts.user_state;

    // 檢查使用者存款
    if user_state.stake_amount < lamports {
        return Err(ErrorCode::InsufficientFunds.into());
    }

    // 減少使用者人數
    if user_state.stake_amount == lamports {
        vault_state.total_users -= 1;
    }

    // 減少 stake amount
    user_state.stake_amount -= lamports;

    let seeds = &[b"vault", ctx.accounts.vault_state.to_account_info().key.as_ref(), &[ctx.accounts.vault_state.vault_bump]];
    let vault_seeds = &[&seeds[..]];

    let accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        accounts,
        vault_seeds,
    );
    transfer(cpi_ctx, lamports)
}