use thiserror::Error;

#[derive(Error, Debug)]
pub enum LPDPClientError {
    #[error("{0}")]
    UnreachableServer(String),
    #[error("{0}")]
    NotAcknowledged(String),
    #[error("{0}")]
    FailedWrite(String),
    #[error("{0}")]
    FailedRead(String),
    #[error("{0}")]
    SystemDetailsError(String),
    #[error("{0}")]
    FileReadError(String),
    #[error("{0}")]
    WriteTimeoutError(String),
    #[error("{0}")]
    ReadTimeoutError(String),
}

impl serde::Serialize for LPDPClientError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
