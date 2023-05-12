use std::fmt::Display;

use crate::Error;

#[derive(Clone, Copy)]
pub enum ContainerEngine {
	Docker,
	Podman,
}

impl TryFrom<String> for ContainerEngine {
	type Error = Error;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		match value.as_str() {
			"docker" => Ok(ContainerEngine::Docker),
			"podman" => Ok(ContainerEngine::Podman),
			_ => Err(Error::UnknownContainerEngine),
		}
	}
}

impl Display for ContainerEngine {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ContainerEngine::Docker => write!(f, "docker"),
			ContainerEngine::Podman => write!(f, "podman"),
		}
	}
}
