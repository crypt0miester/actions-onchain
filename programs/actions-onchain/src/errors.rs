use anchor_lang::prelude::*;

#[error_code]
pub enum ActionsError {
    // 0x1770 - 6000
    #[msg("PublicKeyMismatch")]
    PublicKeyMismatch,
    // 0x1771 - 6001
    #[msg("Invalid Instruction Account")]
    InvalidInstructionAccount,

}
