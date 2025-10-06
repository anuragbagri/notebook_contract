use anchor_lang::prelude::*;
pub mod error;
use anchor_lang::{context, solana_program::clock};

use crate::error::NotesError;
declare_id!("HbyTDhGiswGqHmEr9dMBHgPRmPQERZirhkrtPyAGnorY");

#[program]

pub mod notes {

    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, title: String, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let clock = Clock::get()?;

        require!(title.len() <= 100, NotesError::TitleTooLong);
        require!(content.len() <= 1000, NotesError::ContentTooLong);

        require!(!title.trim().is_empty(), NotesError::TitleEmpty);
        require!(!content.trim().is_empty(), NotesError::ContentEmpty);

        note.author = ctx.accounts.author.key();
        note.title = title.clone();
        note.content = content.clone();
        note.created_at = clock.unix_timestamp;
        note.updated_at = clock.unix_timestamp;
        Ok(())
    }

    pub fn update_note(ctx: Context<UpdateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let clock = Clock::get()?;

        require!(
            note::author == ctx.accounts.author.key(),
            NotesError::UnAuthorized
        );
        require!(content.len() <= 1000, NotesError::ContentTooLong);
        require!(!content.trim().is_empty(), NotesError::ContentEmpty);

        note.content = content.clone();
        note.updated_at = clock.unix_timestamp;

        msg!("note {} updated", note.title);

        Ok(())
    }

    pub fn delete_note(ctx: Context<DeleteNote>) -> Result<()> {
        let note = &ctx.accounts.note;
        require!(
            note.author == ctx.accounts.author.key(),
            NotesError::UnAuthorized
        );

        msg!("Note {} deleted", note.title);
        Ok(())
    }
}
#[derive(Accounts)]
#[instruction(title : String)]
pub struct CreateNote<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        init,
        payer = author,
        space = 8 + Notes::INIT_SPACE,
        seeds = [b"notes", author.key().as_ref(), title.as_bytes()],
        bump,
    )]
    pub note: Account<'info, Notes>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateNote<'info> {
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        seeds = [b"notes" , author.key().as_ref() , note.title.as_bytes()],
        bump,
    )]
    pub note: Account<'info, Notes>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        seeds = [b"note" , author.key().as_ref() , note.title.as_bytes()],
        bump,
        close = author,
    )]
    pub note: Account<'info, Notes>,

    #[account(mut)]
    pub author: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Notes {
    pub author: Pubkey,
    #[max_len(100)]
    pub title: String,
    #[max_len(1000)]
    pub content: String,
    pub created_at: i64,
    pub updated_at: i64,
}
