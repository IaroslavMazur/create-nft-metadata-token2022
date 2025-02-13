use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
        Metadata,
    },
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

declare_id!("BKyyKfz8MUVEsKN5ucE8KK2wepijTMkjxKhRVwM8CMVB");

#[program]
pub mod create_nft_metadata_token_2022 {
    use super::*;

    pub fn create_nft_with_metadata(ctx: Context<CreateNftWithMetadata>) -> Result<()> {
        // Mint the NFT
        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    to: ctx.accounts.nft_ata.to_account_info(),
                    authority: ctx.accounts.signer.to_account_info(),
                },
            ),
            1,
        )?;

        // Create the Metadata account for the NFT
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.nft_metadata.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    mint_authority: ctx.accounts.signer.to_account_info(),
                    update_authority: ctx.accounts.signer.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            DataV2 {
                name: "NFT_NAME".to_owned(),
                symbol: "NFT_SYMBOL".to_string(),
                uri: "NFT_URI".to_string(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateNftWithMetadata<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [b"nft_mint"],
        bump,
        mint::decimals = 0,
        mint::authority = signer,
        mint::freeze_authority = signer,
        mint::token_program = token_program,
    )]
    pub nft_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = nft_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub nft_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"metadata",
                 token_metadata_program.key().as_ref(),
                 nft_mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    /// CHECK: This account will be initialized by the Metaplex program
    pub nft_metadata: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
