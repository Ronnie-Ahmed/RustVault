use thiserror::Error;

#[derive(Debug,Error)]
pub enum VaultError{
    #[error("unknown command '{0}'")]
    UnknownCommand(String),

    #[error("wrong number of arguments for '{0}' command")]
    WrongArgCount(String),

    #[error("invalid TTL/expiry value: '{0}' (expected a positive integer, seconds)")]
    InvalidTtl(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}