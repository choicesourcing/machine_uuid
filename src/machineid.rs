pub use self::machineid::get;
pub use self::machineid::get_via_windows_shell;
pub use self::machineid::transform_windows_output;

pub mod machineid {

    use std::process::Command;

    pub fn get() -> String
    {
        let mut uuid = String::from("");

        if cfg!(windows) {
            let output = get_via_windows_shell();
            uuid = transform_windows_output(output);
        } else if cfg!(unix) {
            println!("this is unix alike");
        }

        return uuid;

    }

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

    pub fn transform_windows_output(output: String) -> String {
        let parts: Vec<&str> = output.splitn(2, ' ').collect();
        return String::from(parts[1].trim());
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use regex::Regex;

    #[test]
    fn it_gets_uuid_for_windows() {
        let output = machineid::get_via_windows_shell();
        let result = transform_windows_output(output); 
        let re = Regex::new(r"\b[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}\b").unwrap();
        assert!(re.is_match(&result.to_lowercase()));
    }
}
