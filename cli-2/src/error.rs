use std::backtrace::Backtrace;
use std::fmt;

pub struct CliError {
    pub(crate) inner: Error,
    pub(crate) verbose: bool,
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.verbose {
            write!(f, "{:#}", self.inner)?;
        } else {
            write!(f, "{}", self.inner)?;
        }

        Ok(())
    }
}

// TODO don't use println -> error!()
#[derive(Debug)]
pub struct Error {
    backtrace: Backtrace,
    kind: ErrorKind,
}

#[derive(Debug)]
pub(crate) enum ErrorKind {
    Deserialization {
        string: String, // don't know size during compilation
        format: &'static str,
        source: Box<dyn std::error::Error>,
    },
    Fatal {
        message: String,
    },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Deserialization { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Fatal { message } => {
                write!(f, "Fatal error: {message}")?;
            }
            ErrorKind::Deserialization {
                string,
                format,
                source,
            } => {
                write!(
                    f,
                    "Failed to deserialize {format} string: {source}\
                    \n{format} content:\n{string}",
                )?;
            }
        }

        if f.alternate() {
            write!(f, "\n\nBacktrace:\n{}", self.backtrace)?;
        }

        Ok(())
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self {
            backtrace: Backtrace::capture(),
            kind,
        }
    }
    pub fn fatal(message: String) -> Self {
        Self::new(ErrorKind::Fatal { message })
    }
}
