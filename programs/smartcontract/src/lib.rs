use anchor_lang::prelude::*;

declare_id!("4DYP8c9XLTW88FrfPkJ1F1Ak4hcHLQy5kZXFo5QiJuAi");

#[program]
pub mod smartcontract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
