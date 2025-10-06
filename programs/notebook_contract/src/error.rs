use anchor_lang::error_code;

#[error_code]
pub enum NotesError {
    #[msg("title should not be larger than 100 chars")]
    TitleTooLong,
    #[msg("content cannot be more than 1000 chars")]
    ContentTooLong,
    #[msg("title cannot be empty ")]
    TitleEmpty,
    #[msg("content cannot be empty")]
    ContentEmpty,
    #[msg("unauthorized")]
    UnAuthorized,
}
