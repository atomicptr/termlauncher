#[derive(Debug)]
pub enum Error {
    /// The requested terminal emulator was not found on the system
    TerminalNotFound(String),
    /// No supported terminal is available
    NoSupportedTerminalAvailable,
    /// An I/O error occurred while spawning the process
    IOError(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TerminalNotFound(t) => write!(f, "Terminal emulator '{t}' not found"),
            Error::NoSupportedTerminalAvailable => {
                write!(f, "No supported terminal emulator could be found")
            }
            Error::IOError(error) => write!(f, "IO error: {error}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
