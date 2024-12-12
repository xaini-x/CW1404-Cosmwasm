use anchor_lang::{prelude::*, solana_program::pubkey};
use anchor_spl::token::{
    self, initialize_mint, Approve, Burn, FreezeAccount, Mint, MintTo, Revoke, TokenAccount,
    Transfer
};

declare_id!("EzzxkLNd7fuPnq7aZL3pwqinyygtzCYoVm6m8LyMfWZk");

#[program]
pub mod vesting {
    use super::*;

    pub fn Create_vesting_account(ctx: Context<CreateVestingAccount>)-> Result<()> {

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]

pub struct CreateVestingAccount <'info>{
    #[account(mut)]
    pub signer :Signer<'info>,
    #[account(init , space = 8 + VestingAccount::I)]

}

#[account]
#[derive(InitSpace)]

pub struct VestingAccount{
pub owner : pubkey,
pub mint:pubkey,
pub treaury_token_account :pubkey,
#[max_len(50)]
pub company_name:String,
pub tresury_bump : u8,
pub bump : u8
}
