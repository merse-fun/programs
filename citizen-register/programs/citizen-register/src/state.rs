use anchor_lang::prelude::*;

#[account]
// #[derive(Default)]
pub struct CitizenAccount {
    pub name: String,
    pub authority: Pubkey,
}

#[account]
// #[derive(Default)]
pub struct EngineerAccount {
    pub name: String,
    pub authority: Pubkey,
    pub rank: EngineerRank,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EngineerRank {
    Intern,
    Junior,
    Senior,
    General,
}
