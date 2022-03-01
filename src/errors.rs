use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseError {
    pub err: String,
    pub status: u16,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&format!("status: {} {}", &self.err, &self.status), f)
    }
}


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LNError {
    HTTPError(String),
    HTTPResponseError(ResponseError),
    StrikeError(String),
    JsonError(String),
}

impl Display for LNError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl From<reqwest::Error> for LNError {
    fn from(err: reqwest::Error) -> Self {
        LNError::HTTPError(err.to_string())
    }
}