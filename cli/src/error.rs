use std::string::FromUtf8Error;

use srtool_lib::SrtoolLibError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SrtoolError {
	#[error("Unknown container engine `{0:?}`")]
	UnknownContainerEngine(Option<String>),

	#[error("Error setting Ctrl-C handler")]
	CtrlCSetup,

	#[error("IO error: {0}")]
	IO(std::io::Error),

	#[error("UTF8 error: {0}")]
	UTF8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for SrtoolError {
	fn from(error: std::io::Error) -> Self {
		SrtoolError::IO(error)
	}
}

impl From<FromUtf8Error> for SrtoolError {
	fn from(error: FromUtf8Error) -> Self {
		SrtoolError::UTF8(error)
	}
}

impl From<SrtoolLibError> for SrtoolError {
	fn from(error: SrtoolLibError) -> Self {
		match error {
			SrtoolLibError::UnknownContainerEngine(opt) => SrtoolError::UnknownContainerEngine(opt),
			SrtoolLibError::CtrlCSetup => SrtoolError::CtrlCSetup,
			SrtoolLibError::IO(e) => SrtoolError::IO(e),
			SrtoolLibError::UTF8(e) => SrtoolError::UTF8(e),
			_ => unreachable!("Skipping SrtoolLibError::HttpRequest as it's not relevant"),
		}
	}
}
