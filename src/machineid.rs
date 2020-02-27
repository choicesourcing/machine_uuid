//! # Machine-UUID
//!
//! A library for retrieving UUID.

pub use self::machineid::get;
pub use self::machineid::get_via_windows_shell;
pub use self::machineid::get_via_linux_shell;
pub use self::machineid::transform_windows_output;

pub mod machineid {

    use std::process::Command;

    /// Retrieves UUID based on OS.
    ///
    /// # Examples
    ///
    /// ```
    /// let uuid = machineid::get();
    /// 
    /// // Based on OS, UUID format will differ
    /// // Windows
    /// // assert_eq!("140EF834-2DB3-0F7A-27B4-4CEDFB73167C", uuid);
    /// 
    /// // Based on OS, UUID format will differ
    /// // Linux
    /// assert_eq!("92cc698195f84d3b85f1cfb0a09e957f", uuid);
    /// 
    /// ```
    pub fn get() -> String
    {
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
    /// let untransformed = machineid::get_via_windows_shell();
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

        let result = match String::from_utf8(output.stdout) 
                    {
                        Ok(line) =>  line,
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
    /// let uuid_untrimmed = machineid::get_via_linux_shell();
    /// 
    /// // c:\ cat /etc/machine-id
    /// // 92cc698195f84d3b85f1cfb0a09e957f
    ///
    /// ```
    pub fn get_via_linux_shell() -> String {
        let output = Command::new("sh")
            .arg("-c")
            .arg("cat /etc/machine-id")
            .output()
            .expect("failed to execute process");

        let result = match String::from_utf8(output.stdout) 
        {
            Ok(line) =>  line,
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
    /// let raw = machineid::get_via_windows_shell();
    /// let just_uuid = machineid::transform_windows_output(raw);
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
    fn it_gets_uuid_for_linux() {
        let result = machineid::get_via_linux_shell();
        assert!(result.trim().len() == 32);
    }

    #[test]
    fn it_gets_uuid_for_windows() {
        let output = machineid::get_via_windows_shell();
        let result = transform_windows_output(output); 
        let re = Regex::new(r"\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b").unwrap();
        assert!(re.is_match(&result.to_lowercase()));
    }
}
