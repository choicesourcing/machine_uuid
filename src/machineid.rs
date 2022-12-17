//! # Machine-UUID
//!
//! A library for retrieving UUID.

pub use self::machineid::get;
pub use self::machineid::get_via_linux_shell;
pub use self::machineid::get_via_macos_shell;
pub use self::machineid::get_via_windows_shell;
pub use self::machineid::transform_windows_output;
pub use self::machineid::MachineIdError;

pub mod machineid {

    use std::{error::Error, ffi::OsStr, fmt, process::Command, string::FromUtf8Error};

    #[derive(Debug)]
    pub enum MachineIdError {
        ExecuteProcessError(std::io::Error),
        ParseError(FromUtf8Error),
    }

    impl Error for MachineIdError {}

    impl fmt::Display for MachineIdError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                MachineIdError::ExecuteProcessError(err) => {
                    write!(f, "Failed to execute process: {err}")
                }
                MachineIdError::ParseError(err) => {
                    write!(f, "Failed to retrieve UUID from shell: {err}")
                }
            }
        }
    }

    /// Retrieves UUID based on OS.
    ///
    /// # Examples
    ///
    /// ```
    /// let uuid = machine_uuid::get();
    ///
    /// // Based on OS, UUID format will differ
    /// // Windows
    /// #[cfg(any(windows))]
    /// assert_eq!("140EF834-2DB3-0F7A-27B4-4CEDFB73167C", uuid);
    ///
    /// // Based on OS, UUID format will differ
    /// #[cfg(any(linux))]
    /// assert_eq!("92cc698195f84d3b85f1cfb0a09e957f\n", uuid);
    ///
    /// // Based on OS, UUID format will differ
    /// #[cfg(any(macos))]
    /// assert_eq!("F7FA2B78-F7D4-5B1B-A4E3-BACB1BBD95A1\n", uuid);
    ///
    /// ```
    pub fn get() -> Result<String, MachineIdError> {
        let mut uuid = String::from("");

        if cfg!(windows) {
            let output = get_via_windows_shell()?;
            uuid = transform_windows_output(output);
        } else if cfg!(target_os = "macos") {
            uuid = get_via_macos_shell()?;
        } else if cfg!(unix) {
            uuid = get_via_linux_shell()?;
        }

        Ok(uuid)
    }

    /// Retrieves raw format of UUID retrieval.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(any(windows))]
    /// let untransformed = machine_uuid::get_via_windows_shell();
    ///
    /// // c:\ wmic csproduct get UUID
    /// // UUID
    /// // 140EF834-2DB3-0F7A-27B4-4CEDFB73167C
    ///
    /// ```
    pub fn get_via_windows_shell() -> Result<String, MachineIdError> {
        get_via_shell("cmd", ["/C", "wmic csproduct get UUID"])
    }

    /// Retrieves raw format of UUID retrieval.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(target_os = "macos")]
    /// let uuid_untrimmed = machine_uuid::get_via_macos_shell();
    ///
    /// // # ioreg -d2 -c IOPlatformExpertDevice | awk -F\" '/IOPlatformUUID/{print $(NF-1)}'
    /// // F7FA2B78-F7D4-5B1B-A4E3-BACB1BBD95A1
    ///
    /// ```
    pub fn get_via_macos_shell() -> Result<String, MachineIdError> {
        get_via_shell(
            "sh",
            [
                "-c",
                r#"ioreg -d2 -c IOPlatformExpertDevice | awk -F\" '/IOPlatformUUID/{print $(NF-1)}'"#,
            ],
        )
    }

    /// Retrieves raw format of UUID retrieval.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(any(unix))]
    /// let uuid_untrimmed = machine_uuid::get_via_linux_shell();
    ///
    /// // # cat /etc/machine-id
    /// // 92cc698195f84d3b85f1cfb0a09e957f
    ///
    /// ```
    pub fn get_via_linux_shell() -> Result<String, MachineIdError> {
        get_via_shell("sh", ["-c", "cat /etc/machine-id"])
    }

    /// Transforms Windows raw format to only contain the UUID.
    ///
    /// # Examples
    ///  
    /// ```
    /// // Output
    /// // UUID  *transforms removes*
    /// // 140EF834-2DB3-0F7A-27B4-4CEDFB73167C *transform returns*
    ///
    /// //let raw = machine_uuid::get_via_windows_shell();
    /// //let just_uuid = machine_uuid::transform_windows_output(raw);
    ///
    /// ```
    pub fn transform_windows_output(output: String) -> String {
        let parts: Vec<&str> = output.splitn(2, ' ').collect();
        return String::from(parts[1].trim());
    }

    /// Retrieves raw string from shell.
    fn get_via_shell<S, I>(shell: S, args: I) -> Result<String, MachineIdError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new(shell)
            .args(args)
            .output()
            .map_err(MachineIdError::ExecuteProcessError)
            .and_then(|output| String::from_utf8(output.stdout).map_err(MachineIdError::ParseError))
    }
}

#[cfg(test)]
mod tests {

    // Tests are not agnostic to OS unfortunately.
    use super::*;
    use regex::Regex;

    #[test]
    fn test_if_gets_uuid() {
        if cfg!(windows) {
            let output = machineid::get_via_windows_shell().unwrap();
            let result = transform_windows_output(output);
            let re =
                Regex::new(r"\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b")
                    .unwrap();
            assert!(re.is_match(&result.to_lowercase()));
        } else if cfg!(target_os = "macos") {
            let result = machineid::get_via_macos_shell().unwrap();
            let re =
                Regex::new(r"\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b")
                    .unwrap();
            assert!(re.is_match(&result.to_lowercase()));
        } else {
            let result = machineid::get_via_linux_shell().unwrap();
            assert!(result.trim().len() == 32);
        }
    }
}
