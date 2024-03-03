use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum EscrowErrorCode {
    #[msg("Acceptor is not initialized")]
    AcceptorNotSet,
    #[msg("User didn't initialize the bet")]
    NoInitializer,
    #[msg("Invalid role")]
    InvalidRole,
}
