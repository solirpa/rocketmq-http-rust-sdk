use std::fmt::{Debug};

/// Error type using by [`ClientError`].
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Failed to parse config")]
    Config,

    #[error("Failed to signature")]
    Signature,

    #[error("Unsupported method")]
    UnsupportedMethod,

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    Xml(#[from] quick_xml::DeError),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Unknown error")]
    Unknown,
}
