use std::error;
use std::fmt;

#[derive(Debug)]
pub enum WowError {
    InvalidProtocolMessage,
    Parse(serde_json::Error),
}

impl fmt::Display for WowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WowError::InvalidProtocolMessage => write!(f, "received protocol message is not valid"),
            WowError::Parse(..) => write!(
                f,
                "received protocol message cannot be parsed, connection might be interrupted"
            ),
        }
    }
}

impl error::Error for WowError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            WowError::InvalidProtocolMessage => None,
            WowError::Parse(e) => Some(e),
        }
    }
}

impl From<serde_json::Error> for WowError {
    fn from(err: serde_json::Error) -> WowError {
        WowError::Parse(err)
    }
}
