
use anchor_lang::prelude::*;

/// The Action 
#[account]
pub struct Action {
    pub creator: Pubkey,                // creator, used to seed pda
    pub icon_uri: String,               // action icon uri
    pub title: String,                  // action title
    pub description: String,            // action description
    pub label: String,                  // action label
    pub instruction_index: u8,          // index instructions to be executed
    pub positive_validations: u8,      // count of positive validations from trusted sources
    pub negative_validations: u8,      // count of negative validations from trusted sources
}


impl Action {

    /// Initializes the action account
    pub fn init(
        &mut self,
        creator: Pubkey,
        icon_uri: String,
        title: String,
        description: String,
        label: String,
        instruction_index: u8
    ) -> Result<()> {
        self.creator = creator;
        self.icon_uri = icon_uri;
        self.title = title;
        self.description = description;
        self.label = label;
        self.instruction_index = instruction_index;
        self.positive_validations = 0;
        self.negative_validations = 0;
        Ok(())
    }

    /// Calculates the size of the Action account
    pub fn get_action_size(icon_uri: &str, title: &str, description: &str, label: &str) -> usize {
        // The fixed size components
        let fixed_size: usize = 32 +    // creator pubkey
                                1 +     // status (enum)
                                1 +     // instruction_index
                                1 +     // positive_validations
                                1;      // negative_validations

        // Calculate the size of variable-length fields
        let variable_size: usize = 4 + icon_uri.len() +     // 4 bytes for length prefix + icon_uri bytes
                                   4 + title.len() +        // 4 bytes for length prefix + title bytes
                                   4 + description.len() +  // 4 bytes for length prefix + description bytes
                                   4 + label.len();         // 4 bytes for length prefix + label bytes

        fixed_size + variable_size
    }
}
