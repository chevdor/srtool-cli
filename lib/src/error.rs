use std::string::FromUtf8Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SrtoolLibError>;

#[derive(Error, Debug)]
pub enum SrtoolLibError {
	#[error("Unknown container engine `{0:?}`")]
	UnknownContainerEngine(Option<String>),

	#[error("Error setting Ctrl-C handler")]
	CtrlCSetup,

	#[error("IO error: {0}")]
	IO(std::io::Error),

	#[error("Http request error: {0}")]
	HttpRequest(Box<ureq::Error>),

	#[error("UTF8 error: {0}")]
	UTF8(std::string::FromUtf8Error),
}

impl From<ureq::Error> for SrtoolLibError {
	fn from(error: ureq::Error) -> Self {
		SrtoolLibError::HttpRequest(Box::new(error))
	}
}

impl From<std::io::Error> for SrtoolLibError {
	fn from(error: std::io::Error) -> Self {
		SrtoolLibError::IO(error)
	}
}

impl From<FromUtf8Error> for SrtoolLibError {
	fn from(error: FromUtf8Error) -> Self {
		SrtoolLibError::UTF8(error)
	}
}
