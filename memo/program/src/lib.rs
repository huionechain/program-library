#![deny(missing_docs)]

//! A program that accepts a string of encoded characters and verifies that it parses,
//! while verifying and logging signers. Currently handles UTF-8 characters.

mod entrypoint;
pub mod processor;

// Export current sdk types for downstream users building with a different sdk version
pub use huione_program;
use huione_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// Legacy symbols from Memo v1
// pub mod v1 {
//     huione_program::declare_id!("8jjXD6tToXM9zJiLG44t1P4nhrf8zrVLy3T39CvBEgaa");
// }

huione_program::declare_id!("HuiMemo111111111111111111111111111111111111");

/// Build a memo instruction, possibly signed
///
/// Accounts expected by this instruction:
///
///   0. ..0+N. `[signer]` Expected signers; if zero provided, instruction will be processed as a
///     normal, unsigned spl-memo
///
pub fn build_memo(memo: &[u8], signer_pubkeys: &[&Pubkey]) -> Instruction {
    Instruction {
        program_id: id(),
        accounts: signer_pubkeys
            .iter()
            .map(|&pubkey| AccountMeta::new_readonly(*pubkey, true))
            .collect(),
        data: memo.to_vec(),
    }
}
