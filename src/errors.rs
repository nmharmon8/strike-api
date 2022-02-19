use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LNError {
    pub err: String,
}

impl Display for LNError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for LNError {}

#[derive(Debug, Clone, PartialEq)]
pub enum LNErrorKind {
    HTTPError(LNError),
    StrikeError(LNError),
    QRCodeError(LNError),
    JsonError(LNError),
}

impl From<qrcode_generator::QRCodeError> for LNErrorKind {
    fn from(err: qrcode_generator::QRCodeError) -> Self {
        LNErrorKind::QRCodeError(LNError {
            err: err.to_string(),
        })
    }
}

impl From<reqwest::Error> for LNErrorKind {
    fn from(err: reqwest::Error) -> Self {
        LNErrorKind::HTTPError(LNError {
            err: err.to_string(),
        })
    }
}
