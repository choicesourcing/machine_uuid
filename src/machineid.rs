//! # Machine-UUID
//!
//! A library for retrieving UUID.

pub use self::machineid::get;
pub use self::machineid::get_via_linux_shell;
pub use self::machineid::get_via_windows_shell;
pub use self::machineid::transform_windows_output;

pub mod machineid {

    use std::process::Command;

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
    /// ```
    pub fn get() -> String {
        let mut uuid = String::from("");

        if cfg!(windows) {
            let output = get_via_windows_shell();
            uuid = transform_windows_output(output);
        } else if cfg!(unix) {
            uuid = get_via_linux_shell();
        }

        return uuid;
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
    pub fn get_via_windows_shell() -> String {
        let output = Command::new("cmd")
            .args(&["/C", "wmic csproduct get UUID"])
            .output()
            .expect("failed to execute process");

        let result = match String::from_utf8(output.stdout) {
            Ok(line) => line,
            Err(_) => {
                println!("Failed to retrieve UUID from windows shell.");
                String::from("")
            }
        };

        return result;
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
    pub fn get_via_linux_shell() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg("cat /etc/machine-id")
            .output()
            .expect("failed to execute process");

        let result = match String::from_utf8(output.stdout) {
            Ok(line) => line,
            Err(_) => {
                println!("Failed to retrieve UUID from linux shell.");
                String::from("")
            }
        };

        return result;
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
}

#[cfg(test)]
mod tests {

    // Tests are not agnostic to OS unfortunately.
    use super::*;
    use regex::Regex;

    #[test]
    fn test_if_gets_uuid() {
        if cfg!(windows) {
            let output = machineid::get_via_windows_shell();
            let result = transform_windows_output(output);
            let re =
                Regex::new(r"\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b")
                    .unwrap();
            assert!(re.is_match(&result.to_lowercase()));
        } else {
            let result = machineid::get_via_linux_shell();
            assert!(result.trim().len() == 32);
        }
    }
}
