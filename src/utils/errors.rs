use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};

#[derive(Debug)]
pub enum ErrorType {
    Reqwest(reqwest::Error),
    HeaderValueError(InvalidHeaderValue),
    HeaderNameError(InvalidHeaderName),
    SerdeJson(serde_json::Error),
    InvalidProxy,
}

#[derive(Debug)]
pub struct ErrorT {
    pub inner: ErrorType,
}

impl ErrorT {
    pub(crate) fn new(inner: ErrorType) -> Self {
        ErrorT { inner }
    }
}

impl From<reqwest::Error> for ErrorT {
    fn from(err: reqwest::Error) -> Self {
        ErrorT::new(ErrorType::Reqwest(err))
    }
}

impl From<InvalidHeaderValue> for ErrorT {
    fn from(error: InvalidHeaderValue) -> Self {
        ErrorT::new(ErrorType::HeaderValueError(error))
    }
}

impl From<InvalidHeaderName> for ErrorT {
    fn from(error: InvalidHeaderName) -> Self {
        ErrorT::new(ErrorType::HeaderNameError(error))
    }
}

impl From<serde_json::Error> for ErrorT {
    fn from(value: serde_json::Error) -> Self {
        ErrorT::new(ErrorType::SerdeJson(value))
    }
}
