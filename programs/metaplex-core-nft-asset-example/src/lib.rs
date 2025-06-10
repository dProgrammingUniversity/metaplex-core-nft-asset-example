use anchor_lang::prelude::*;

declare_id!("BjMEJTk1VQWDeDyTMaziXLYzujvVqgdvHEu4BYNQ56NR");

#[program]
pub mod metaplex_core_nft_asset_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
