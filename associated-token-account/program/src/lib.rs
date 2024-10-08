//! Convention for associating token accounts with a user wallet
#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod entrypoint;
pub mod processor;

// Export current SDK types for downstream users building with a different SDK version
pub use huione_program;
use huione_program::{
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    sysvar,
};

huione_program::declare_id!("HuiATA1111111111111111111111111111111111111");

pub(crate) fn get_associated_token_address_and_bump_seed(
    wallet_address: &Pubkey,
    hpl_token_mint_address: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    get_associated_token_address_and_bump_seed_internal(
        wallet_address,
        hpl_token_mint_address,
        program_id,
        &hpl_token::id(),
    )
}

/// Derives the associated token account address for the given wallet address and token mint
pub fn get_associated_token_address(
    wallet_address: &Pubkey,
    hpl_token_mint_address: &Pubkey,
) -> Pubkey {
    get_associated_token_address_and_bump_seed(wallet_address, hpl_token_mint_address, &id()).0
}

fn get_associated_token_address_and_bump_seed_internal(
    wallet_address: &Pubkey,
    hpl_token_mint_address: &Pubkey,
    program_id: &Pubkey,
    token_program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            &wallet_address.to_bytes(),
            &token_program_id.to_bytes(),
            &hpl_token_mint_address.to_bytes(),
        ],
        program_id,
    )
}

/// Create an associated token account for the given wallet address and token mint
///
/// Accounts expected by this instruction:
///
///   0. `[writeable,signer]` Funding account (must be a system account)
///   1. `[writeable]` Associated token account address to be created
///   2. `[]` Wallet address for the new associated token account
///   3. `[]` The token mint for the new associated token account
///   4. `[]` System program
///   5. `[]` HPL Token program
///   6. `[]` Rent sysvar
///
pub fn create_associated_token_account(
    funding_address: &Pubkey,
    wallet_address: &Pubkey,
    hpl_token_mint_address: &Pubkey,
) -> Instruction {
    let associated_account_address =
        get_associated_token_address(wallet_address, hpl_token_mint_address);

    Instruction {
        program_id: id(),
        accounts: vec![
            AccountMeta::new(*funding_address, true),
            AccountMeta::new(associated_account_address, false),
            AccountMeta::new_readonly(*wallet_address, false),
            AccountMeta::new_readonly(*hpl_token_mint_address, false),
            AccountMeta::new_readonly(huione_program::system_program::id(), false),
            AccountMeta::new_readonly(hpl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: vec![],
    }
}
