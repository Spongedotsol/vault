use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod instructions;

use instructions::*;

// declare_id!("HoU7uBBQf1eqX2StdnCdgA7wuDZB3kyxU1EgpZ6aqPKF");
declare_id!("2ZoxjRSMEc67GbE3uzyNPxSM7tj72Fk2jhm5KEvHvYr9");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault::handler(ctx)
    }
    
    pub fn stake(ctx: Context<Stake>, lamports: u64) -> Result<()> {
        instructions::stake::handler(ctx, lamports)
    }

    pub fn unstake(ctx: Context<Unstake>, lamports: u64) -> Result<()> {
        instructions::unstake::handler(ctx, lamports)
    }

    pub fn close_vault(ctx: Context<CloseVault>) -> Result<()> {
        instructions::close_vault::handler(ctx)
    }
}

// use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

// declare_id!("HoU7uBBQf1eqX2StdnCdgA7wuDZB3kyxU1EgpZ6aqPKF");

// #[program]
// pub mod vault {
//     use super::*;
    
//     pub fn stake(ctx: Context<Vault>, lamports: u64) -> Result<()> {
//         ctx.accounts.stake(lamports)
//     }

//     pub fn unstake(ctx: Context<Vault>, lamports: u64) -> Result<()> {
//         ctx.accounts.unstake(lamports, &[ctx.bumps.vault])
//     }
// }

// #[derive(Accounts)]
// pub struct Vault<'info> {
//     #[account(mut)]
//     signer: Signer<'info>,
//     #[account(
//         mut,
//         seeds = [signer.key().as_ref()],
//         bump
//     )]
//     vault: SystemAccount<'info>,
//     system_program: Program<'info, System>
// }

// impl<'info> Vault<'info> {
//     pub fn stake(&self, lamports: u64) -> Result<()> {
//         let accounts = Transfer {
//             from: self.signer.to_account_info(),
//             to: self.vault.to_account_info()
//         };

//         let ctx = CpiContext::new(
//             self.system_program.to_account_info(), 
//             accounts
//         );

//         transfer(ctx, lamports)
//     }

//     pub fn unstake(&self, lamports: u64, bump: &[u8]) -> Result<()> {
//         let signer_seeds = [&[self.signer
//         .key.as_ref(), bump][..]];

//         let accounts = Transfer {
//             from: self.vault.to_account_info(),
//             to: self.signer.to_account_info()
//         };

//         let ctx = CpiContext::new_with_signer(
//             self.system_program.to_account_info(), 
//             accounts, 
//             &signer_seeds
//         );

//         transfer(ctx, lamports)
//     }
// }