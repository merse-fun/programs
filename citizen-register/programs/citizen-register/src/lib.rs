use anchor_lang::prelude::*;

pub mod constant;
pub mod state;
pub mod error;

use crate::{constant::*, error::*, state::*};

declare_id!("2n6zPmbTaswX2hicYmF91w2H4ZMB9Wz8JJmtQqBCyRFE");

#[program]
mod citizen_register {
    use super::*;
    pub fn create_citizen(ctx: Context<CreateCitizen>, name: String) -> Result<()> {
        let citizen_account = &mut ctx.accounts.citizen_account;
        citizen_account.authority = ctx.accounts.signer.key();
        citizen_account.name = name;

        Ok(())
    }

    pub fn create_engineer(ctx: Context<CreateEngineer>, name: String) -> Result<()> {
        let engineer_account = &mut ctx.accounts.engineer_account;
        engineer_account.authority = ctx.accounts.signer.key();
        engineer_account.name = name;
        engineer_account.rank = EngineerRank::Intern;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCitizen<'info> {
    #[account(
        
        seeds = [CREATE_CITIZEN_TAG, signer.key().as_ref()],
        bump,
        payer = signer, 
        space = std::mem::size_of::<CitizenAccount>() + 8
    )]
    pub citizen_account: Box<Account<'info, CitizenAccount>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateEngineer<'info> {
    #[account(
        
        seeds = [CREATE_ENGINEER_TAG, signer.key().as_ref()],
        bump,
        payer = signer,
        space = std::mem::size_of::<EngineerAccount>() + 8,
    )]
    pub engineer_account: Box<Account<'info, EngineerAccount>>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
