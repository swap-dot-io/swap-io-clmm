use crate::error::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseSupportMintAssociated<'info> {
    /// Address of the admin
    #[account(
        mut,
        address = crate::admin::id() @ ErrorCode::NotApproved
    )]
    pub owner: Signer<'info>,
    
    /// The support mint account to be closed
    #[account(
        mut, 
        seeds = [
            SUPPORT_MINT_SEED.as_bytes(),
            support_mint_associated.mint.as_ref(),
        ],
        bump = support_mint_associated.bump,
        close = owner
    )]
    pub support_mint_associated: Account<'info, SupportMintAssociated>,
    
    pub system_program: Program<'info, System>,
}

pub fn close_support_mint_associated(_ctx: Context<CloseSupportMintAssociated>) -> Result<()> {
    Ok(())
}
