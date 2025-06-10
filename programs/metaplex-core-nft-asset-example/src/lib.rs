use anchor_lang::prelude::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    instructions::CreateV2CpiBuilder,
    types::{
        Plugin, FreezeDelegate, PluginAuthority, PluginAuthorityPair,
        ExternalPluginAdapterInitInfo
    }
};

declare_id!("2KZp4Ph2tDqpqMKj3eUVP1VJAh7RzrzbYzvbbZW5f5Aa");

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    name: String,
    uri: String,
}

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        // Convert optional accounts to Option<&AccountInfo>
        let collection = ctx.accounts.collection.as_ref().map(|c| c.to_account_info());
        let authority = ctx.accounts.authority.as_ref().map(|a| a.to_account_info());
        let owner = ctx.accounts.owner.as_ref().map(|o| o.to_account_info());
        let update_authority = ctx.accounts.update_authority.as_ref().map(|u| u.to_account_info());
        
        // Create plugins
        let mut plugins: Vec<PluginAuthorityPair> = vec![];
        plugins.push(PluginAuthorityPair {
            plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: false }),
            authority: Some(PluginAuthority::UpdateAuthority),
        });
        
        // Create external plugin adapters
        let external_plugin_adapters: Vec<ExternalPluginAdapterInitInfo> = vec![];

        // Execute CPI to create Core NFT Asset
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(collection.as_ref())   // Use as_ref() for Option<&AccountInfo>
            .authority(authority.as_ref())     // Use as_ref() for Option<&AccountInfo>
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(owner.as_ref())             // Use as_ref() for Option<&AccountInfo>
            .update_authority(update_authority.as_ref()) // Use as_ref() for Option<&AccountInfo>
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(plugins)
            .external_plugin_adapters(external_plugin_adapters)
            .invoke()?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    /// CHECK: Optional collection account
    #[account(mut)]
    pub collection: Option<UncheckedAccount<'info>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Validated by MPL Core program
    pub owner: Option<UncheckedAccount<'info>>,
    /// CHECK: Validated by MPL Core program
    pub update_authority: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: Validated by address constraint
    pub mpl_core_program: UncheckedAccount<'info>,
}