use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("terminal IO error")]
    TerminalError(#[from] std::io::Error),
    #[error("terminal interface error")]
    InterfaceError(#[from] tty_interface::Error),
}
