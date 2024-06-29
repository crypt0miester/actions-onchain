use anchor_lang::prelude::*;
use constants::*;
use state::{Action, ActionInstruction};
mod state;
mod constants;

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