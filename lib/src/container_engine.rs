use std::{fmt::Display, process::Command};

use crate::SrtoolLibError;

/// Represents the container engine being used.
#[derive(Clone, Copy, PartialEq)]
pub enum ContainerEngine {
	/// Represents the Docker container engine.
	Docker,
	/// Represents the Podman container engine.
	Podman,
}

impl ContainerEngine {
	/// Check whether you have Podman and/or Docker installed. The default will be Podman if both are present.
	pub fn detect() -> Result<ContainerEngine, SrtoolLibError> {
		if let Ok(engine) = std::env::var("ENGINE") {
			return ContainerEngine::try_from(engine.as_str());
		}

		let podman_output: Option<std::process::Output> = Command::new("podman").arg("--version").output().ok();
		if let Some(podman) = podman_output {
			let podman = String::from_utf8_lossy(&podman.stdout);
			if podman.to_lowercase().contains("podman") {
				return Ok(ContainerEngine::Podman);
			} else if podman.contains("docker") {
				return Ok(ContainerEngine::Docker);
			}
		}

		let docker_output = Command::new("docker").arg("--version").output().ok();
		if let Some(docker) = docker_output {
			let docker = String::from_utf8_lossy(&docker.stdout);
			if docker.to_lowercase().contains("docker") {
				return Ok(ContainerEngine::Docker);
			} else if docker.contains("podman") {
				return Ok(ContainerEngine::Podman);
			}
		}

		Err(SrtoolLibError::UnknownContainerEngine(None))
	}
}

impl TryFrom<&str> for ContainerEngine {
	type Error = SrtoolLibError;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		match s.to_ascii_lowercase().as_str() {
			"auto" => Self::detect(),
			"podman" => Ok(ContainerEngine::Podman),
			"docker" => {
				println!("WARNING: You are using docker. We recommend using podman instead.");
				Ok(ContainerEngine::Docker)
			}
			_ => Err(SrtoolLibError::UnknownContainerEngine(Some(s.into()))),
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

#[cfg(test)]
mod tests {
	use super::*;
	use std::env;

	#[test]
	fn detect_works() {
		env::set_var("ENGINE", "docker");
		assert!(ContainerEngine::detect().unwrap() == ContainerEngine::Docker);

		env::set_var("ENGINE", "podman");
		assert!(ContainerEngine::detect().unwrap() == ContainerEngine::Podman);
		// Cleanup after test
		env::remove_var("ENGINE");
	}

	#[test]
	fn container_enginer_try_from_works() {
		assert!(ContainerEngine::try_from("docker").unwrap() == ContainerEngine::Docker);
		assert!(ContainerEngine::try_from("podman").unwrap() == ContainerEngine::Podman);
		assert!(matches!(
			ContainerEngine::try_from("invalid"),
			Err(SrtoolLibError::UnknownContainerEngine(Some(_)))
		));
	}

	#[test]
	fn container_enginer_display_works() {
		assert_eq!(ContainerEngine::Docker.to_string(), "docker");
		assert_eq!(ContainerEngine::Podman.to_string(), "podman");
	}
}
