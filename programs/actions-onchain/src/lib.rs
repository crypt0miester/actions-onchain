use anchor_lang::prelude::*;
use constants::*;
use state::{Action, ActionInstruction};
use errors::ActionsError;
use solana_program::{instruction::Instruction, program::invoke};
mod state;
mod constants;
mod errors;

declare_id!("EYj4oDQT9kwSTfwhDQwXhstE4MJ9fT1RjpwaGmqVY72f");

#[program]
pub mod actions_onchain {

    use super::*;

    pub fn create_action(
        ctx: Context<CreateAction>,
        icon_uri: String,
        title: String,
        description: String,
        label: String,
    ) -> Result<()> {
        let action = &mut ctx.accounts.action;
        action.init(
            ctx.accounts.creator.key(),
            icon_uri,
            title,
            description,
            label,
        )?;
        
        Ok(())
    }

    pub fn vote_on_validation(ctx: Context<VoteOnValidation>, is_positive: bool) -> Result<()> {
        let action = &mut ctx.accounts.action;
        
        if is_positive {
            action.positive_validations = action.positive_validations.checked_add(1).unwrap_or(u8::MAX);
        } else {
            action.negative_validations = action.negative_validations.checked_add(1).unwrap_or(u8::MAX);
        }
        
        Ok(())
    }

    pub fn add_instruction(
        ctx: Context<AddUpdateInstruction>,
        action_instruction: ActionInstruction,
    ) -> Result<()> {
        let instruction = &mut ctx.accounts.instruction;
        let action = &mut ctx.accounts.action;
        let incoming_instruction = action_instruction;
        
        instruction.init(incoming_instruction)?;
        action.instruction_index += 1;
        
        Ok(())
    }

    pub fn execute_transaction(
        ctx: Context<ExecuteTransaction>,
        data_modifications: Vec<(u8, usize, Vec<u8>)>, 
        key_modifications: Vec<(u8, usize, Pubkey)>,
        instructions_list: Vec<u8>,
    ) -> Result<()> {
        let action = &ctx.accounts.action;

        let mapped_remaining_accounts: Vec<AccountInfo> = instructions_list
            .iter()
            .map(|&i| {
                let index = usize::from(i);
                ctx.remaining_accounts[index].clone()
            })
            .collect();

        // iterator for remaining accounts
        let ix_iter = &mut mapped_remaining_accounts.iter();

        (1..=action.instruction_index).try_for_each(|i: u8| {
            // each ix block starts with the action_ix account
            let action_ix_account: &AccountInfo = next_account_info(ix_iter)?;

            // if the attached instruction doesn't belong to this program, throw error
            if action_ix_account.owner != ctx.program_id {
                return err!(ActionsError::InvalidInstructionAccount);
            }

            // deserialize the msIx
            let mut ix_account_data: &[u8] = &action_ix_account.try_borrow_mut_data()?;
            let action_ix = &mut ActionInstruction::try_deserialize(&mut ix_account_data)?;

            // get the instruction account pda - seeded from transaction account + the transaction accounts instruction index
            let (ix_pda, _) = Pubkey::find_program_address(
                &[ACTION_INSTRUCTION_PREFIX.as_bytes(), action.key().as_ref(), &[i]],
                ctx.program_id,
            );
            // check the instruction account key maches the derived pda
            if &ix_pda != action_ix_account.key {
                return err!(ActionsError::IxnPDAInvalid);
            }
            // get the instructions program account
            let ix_program_info: &AccountInfo = next_account_info(ix_iter)?;
            // check that it matches the submitted account
            if &action_ix.program_id != ix_program_info.key {
                return err!(ActionsError::IxnProgramInvalid);
            }
            // create the instruction to invoke from the saved ms ix account
            let ix: Instruction = action_ix.to_instruction();
            // the instruction account vec, with the program account first
            let mut ix_account_infos: Vec<AccountInfo> = vec![ix_program_info.clone()];

            // Apply data modifications
            for (index_of_ix, offset, new_data) in &data_modifications {
                if index_of_ix == &i {
                    if action_ix.data_modifier.contains(&offset) && offset + new_data.len() <= action_ix.data.len() {
                        let offset_set = *offset;
                        action_ix.data[offset_set..offset_set+new_data.len()].copy_from_slice(&new_data);
                    }
                } else {
                    return err!(ActionsError::FoundInvalidDataModifier);
                }
            }
        
            // Apply key modifications
            for (index_of_ix, index_of_pubkey, new_pubkey) in &key_modifications {
                if index_of_ix == &i {
                    if action_ix.key_modifier.contains(&index_of_pubkey) {
                        let index_set = *index_of_pubkey;
                        action_ix.keys[index_set].pubkey = *new_pubkey;
                    }
                } else {
                    return err!(ActionsError::FoundInvalidPubkeyModifier);
                }
            }

            let ix_keys = action_ix.keys.clone();
            // loop through the provided remaining accounts
            for account_index in 0..ix_keys.len() {
                let ix_account_info = next_account_info(ix_iter)?.clone();

                // check that the ix account keys match the submitted account keys
                if *ix_account_info.key != ix_keys[account_index].pubkey {
                    return err!(ActionsError::AccountInfoMissing);
                }

                ix_account_infos.push(ix_account_info.clone());
            }

            invoke(&ix, &ix_account_infos)?;
            Ok(())
        })?;


        
        Ok(())
    }
}
    
#[derive(Accounts)]
#[instruction(
    icon_uri: String,
    title: String,
    description: String,
    label: String,
)]
pub struct CreateAction<'info> {
    #[account(init, 
        payer = creator, 
        space = 8 + Action::get_action_size(&icon_uri, &title, &description, &label),
        seeds = [ACTION_PREFIX.as_bytes(), title.to_lowercase().as_bytes()],
        bump
    )]
    pub action: Account<'info, Action>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteOnValidation<'info> {
    #[account(mut)]
    pub action: Account<'info, Action>,
    pub voter: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(action_instruction: ActionInstruction)]
pub struct AddUpdateInstruction<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(init, 
        payer = creator, 
        space = 8 + ActionInstruction::get_max_size(action_instruction),
        seeds = [ACTION_INSTRUCTION_PREFIX.as_bytes(), action.key().as_ref(), &[action.instruction_index]],
        bump
    
    )]
    pub instruction: Account<'info, ActionInstruction>,
    #[account(mut, 
        has_one=creator,
        seeds = [ACTION_PREFIX.as_bytes(), action.title.to_lowercase().as_bytes()],
        bump
    )]
    pub action: Account<'info, Action>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [ACTION_PREFIX.as_bytes(), action.title.to_lowercase().as_bytes()],
        bump
    )]
    pub action: Account<'info, Action>,
}