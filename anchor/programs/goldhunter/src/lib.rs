use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod goldhunter {
    use super::*;

    /// Mint a cultural fragment cNFT when a user completes a POI challenge
    pub fn mint_fragment(
        ctx: Context<MintFragment>,
        poi_id: String,
        album_id: String,
        metadata_uri: String,
    ) -> Result<()> {
        let fragment = &mut ctx.accounts.fragment;
        fragment.owner = ctx.accounts.user.key();
        fragment.poi_id = poi_id;
        fragment.album_id = album_id;
        fragment.metadata_uri = metadata_uri;
        fragment.minted_at = Clock::get()?.unix_timestamp;
        fragment.redeemed = false;
        Ok(())
    }

    /// Mark a fragment as redeemed when the physical prize is claimed
    pub fn redeem_fragment(ctx: Context<RedeemFragment>) -> Result<()> {
        let fragment = &mut ctx.accounts.fragment;
        require!(!fragment.redeemed, GoldHunterError::AlreadyRedeemed);
        require!(
            fragment.owner == ctx.accounts.user.key(),
            GoldHunterError::Unauthorized
        );
        fragment.redeemed = true;
        fragment.redeemed_at = Some(Clock::get()?.unix_timestamp);
        Ok(())
    }
}

#[account]
pub struct Fragment {
    pub owner: Pubkey,
    pub poi_id: String,
    pub album_id: String,
    pub metadata_uri: String,
    pub minted_at: i64,
    pub redeemed: bool,
    pub redeemed_at: Option<i64>,
}

#[derive(Accounts)]
pub struct MintFragment<'info> {
    #[account(init, payer = user, space = 8 + 32 + 64 + 64 + 128 + 8 + 1 + 9)]
    pub fragment: Account<'info, Fragment>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RedeemFragment<'info> {
    #[account(mut)]
    pub fragment: Account<'info, Fragment>,
    pub user: Signer<'info>,
}

#[error_code]
pub enum GoldHunterError {
    #[msg("Fragment has already been redeemed.")]
    AlreadyRedeemed,
    #[msg("You are not the owner of this fragment.")]
    Unauthorized,
}
