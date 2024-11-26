use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub state_bump: u8,
    pub vault_bump: u8,
    pub total_users: u64, // 紀錄存款的總人數
    pub authority: Pubkey // 管理者
}

#[account]
pub struct UserState {
    pub user_bump: u8,
    pub stake_amount: u64, // 使用者存款的數量
}
