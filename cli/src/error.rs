use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum Error {
	UnknownContainerEngine,
	CtrlCSetupError,
	IOError(std::io::Error),
    UTF8Error(std::string::FromUtf8Error),
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::IOError(error)
	}
}

impl From<FromUtf8Error> for Error {
	fn from(error: FromUtf8Error) -> Self {
		Error::UTF8Error(error)
	}
}

impl From<Error> for String {
	fn from(error: Error) -> Self {
		match error {
			Error::UnknownContainerEngine => "Unknown container engine".to_string(),
			Error::CtrlCSetupError => "Error setting Ctrl-C handler".to_string(),
			Error::IOError(error) => format!("IO error: {}", error),
            Error::UTF8Error(error) => format!("UTF8 error: {}", error),
		}
	}
}
