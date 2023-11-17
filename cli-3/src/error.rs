use std::fmt::Display;

#[derive(Debug)]
pub struct CliError {
    pub(crate) inner: Error,
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.inner)
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Database { message, url } => {
                write!(f, "Failed to connect to database with url: {url}. {message}")
            }
            ErrorKind::Fatal { message } => f.write_str(message),
            ErrorKind::NoValuesSpecified { message } => f.write_str(message),
        }
    }
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Database { message: String, url: String },
    Fatal { message: String },
    NoValuesSpecified { message: String }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    pub(crate) fn fatal(message: String) -> Self {
        Self::new(ErrorKind::Fatal { message })
    }
}
