use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::*;

#[derive(Accounts)]
pub struct Stake<'info> {
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
        init_if_needed,
        payer = user,
        space = 8 + 1 + 64,
        seeds = [b"user_state", user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Stake>, lamports: u64) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    let user_state = &mut ctx.accounts.user_state;

    // 增加使用者人數
    if user_state.stake_amount == 0 {
        vault_state.total_users += 1;
    }

    user_state.stake_amount += lamports;
    user_state.user_bump = ctx.bumps.user_state;

    // transfer accounts
    let accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        accounts
    );
    transfer(cpi_ctx, lamports)
}