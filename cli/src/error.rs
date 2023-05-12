use std::string::FromUtf8Error;


#[derive(Debug)]
pub enum Error {
	UnknownContainerEngine,
	CtrlCSetup,
	IO(std::io::Error),
	UTF8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for Error {
	fn from(error: std::io::Error) -> Self {
		Error::IO(error)
	}
}

impl From<FromUtf8Error> for Error {
	fn from(error: FromUtf8Error) -> Self {
		Error::UTF8(error)
	}
}

impl From<Error> for String {
	fn from(error: Error) -> Self {
		match error {
			Error::UnknownContainerEngine => "Unknown container engine".to_string(),
			Error::CtrlCSetup => "Error setting Ctrl-C handler".to_string(),
			Error::IO(error) => format!("IO error: {error}"),
			Error::UTF8(error) => format!("UTF8 error: {error}"),
		}
	}
}
