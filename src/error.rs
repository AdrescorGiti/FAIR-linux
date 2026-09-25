use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FairError {
    Io(io::Error),
    Dns(String),
    PermissionDenied,
    HostsCorrupted(String),
    Service(String),
}

pub type Result<T> = std::result::Result<T, FairError>;

impl fmt::Display for FairError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FairError::Io(e) => write!(f, "ввод-вывод: {e}"),
            FairError::Dns(msg) => write!(f, "DNS: {msg}"),
            FairError::PermissionDenied => write!(f, "нужны права root (запустите через sudo)"),
            FairError::HostsCorrupted(msg) => write!(f, "/etc/hosts повреждён: {msg}"),
            FairError::Service(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for FairError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FairError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for FairError {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::PermissionDenied {
            FairError::PermissionDenied
        } else {
            FairError::Io(err)
        }
    }
}