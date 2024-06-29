use anchor_lang::prelude::*;

#[error_code]
pub enum ActionsError {
    // 0x1770 - 6000
    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,
    // 0x1771 - 6001
    #[msg("Invalid Instruction Account")]
    InvalidInstructionAccount,
    // 0x1772 - 6002
    #[msg("Found An Invalid Data Modifier")]
    FoundInvalidDataModifier,
    // 0x1773 - 6003
    #[msg("Found An Invalid Pubkey Modifier")]
    FoundInvalidPubkeyModifier,
    // 0x1774 - 6004
    #[msg("An AccountInfo is Missing from Remaining Accounts")]
    AccountInfoMissing,
    // 0x1775 - 6005
    #[msg("Instruction PDA does not match from Remaining Accounts")]
    IxnPDAInvalid,
    // 0x1776 - 6006
    #[msg("Instruction Program ID missing from Remaining Accounts")]
    IxnProgramInvalid,
}
