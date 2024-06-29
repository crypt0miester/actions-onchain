use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

/// The state account for an instruction that is attached to an action.
/// Almost analagous to the native Instruction struct for solana, but with extra
/// fields for modifiers.
#[account]
pub struct ActionInstruction {
    pub program_id: Pubkey,             // program_id of instruction
    pub keys: Vec<ActionAccountMeta>,   // AccountMetas of instruction
    pub data: Vec<u8>,                  // data of instruction
    pub data_modifier: Vec<usize>,      // modifiers of pubkeys
pub key_modifier: Vec<usize>,           // modifiers of data
}

impl ActionInstruction {
    /// Calculates how much space will be needed to allocate to the instruction
    /// to be attached to the action.
    pub fn get_max_size(action_instruction: ActionInstruction) -> usize {
        let program_id_size = 32; // Size of Pubkey
        let keys_size = 4 + (action_instruction.keys.len() * std::mem::size_of::<ActionAccountMeta>()); // 4 bytes for vec length
        let data_size = 4 + action_instruction.data.len(); // 4 bytes for vec length
        let data_modifier_size = 4 + (action_instruction.data_modifier.len() * std::mem::size_of::<usize>()); // 4 bytes for vec length
        let key_modifier_size = 4 + (action_instruction.key_modifier.len() * std::mem::size_of::<usize>()); // 4 bytes for vec length

        program_id_size + keys_size + data_size + data_modifier_size + key_modifier_size
    }

    /// Initializes the instruction account
    pub fn init(&mut self, incoming_instruction: ActionInstruction) -> Result<()> {
        self.program_id = incoming_instruction.program_id;
        self.keys = incoming_instruction.keys;
        self.data = incoming_instruction.data;
        self.data_modifier = incoming_instruction.data_modifier;
        self.key_modifier = incoming_instruction.key_modifier;
        Ok(())
    }

    pub fn to_instruction(&self) -> Instruction {
        Instruction {
            program_id: self.program_id,
            accounts: self.keys
                .iter()
                .map(|account| AccountMeta {
                    pubkey: account.pubkey,
                    is_signer: account.is_signer,
                    is_writable: account.is_writable,
                })
                .collect(),
            data: self.data.clone(),
        }
    }
}

/// Wrapper for our internal ActionInstruction key serialization schema
/// MsAccount meta is identical to the AccountMeta struct, but defined
/// here for serialization purposes.
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct ActionAccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool
}
