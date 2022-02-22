use serde::{Deserialize, Serialize};

use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LNError {
    pub err: String,
}

impl Display for LNError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for LNError {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LNErrorKind {
    HTTPError(LNError),
    StrikeError(LNError),
    JsonError(LNError),
}

impl Display for LNErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl From<reqwest::Error> for LNErrorKind {
    fn from(err: reqwest::Error) -> Self {
        LNErrorKind::HTTPError(LNError {
            err: err.to_string(),
        })
    }
}
