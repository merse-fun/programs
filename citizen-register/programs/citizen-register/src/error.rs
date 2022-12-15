use anchor_lang::prelude::*;

#[error_code]
pub enum CreateCitizenError {
    #[msg("Custom Error Message")]
    CustomErrorName,
}