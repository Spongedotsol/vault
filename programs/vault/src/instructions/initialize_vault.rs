use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeVault<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    // 初始化 vault state
    #[account(
        init,
        payer = authority,
        space = 8 + 1 + 1 + 8 + 32, // discriminator + total_users + authority
        seeds = [b"vault_state", authority.key().as_ref()], // 關聯 authority PK
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    // 初始化 vault
    #[account(
        seeds = [b"vault", vault_state.key().as_ref()], // 關聯 vault state PK
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeVault>) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    vault_state.state_bump = ctx.bumps.vault_state;
    vault_state.vault_bump = ctx.bumps.vault;
    vault_state.total_users = 0;
    vault_state.authority = ctx.accounts.authority.key();
    Ok(())
}