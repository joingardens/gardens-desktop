use std::process::Command;

use crate::core::{dictionaries::version_status_dictionary::VersionStatusDictionary, errors::{base_error::BaseError}};
    
    pub fn trim_whitespace_v2(s: &str) -> String {
        // second attempt: only allocate a string
        let mut result = String::with_capacity(s.len());
        s.split_whitespace().for_each(|w| {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(w);
        });
        result
    }
    pub trait VersionGetter {
        fn version() -> Result<String, BaseError>;
        fn version_status() -> VersionStatusDictionary;
    }
    
    pub struct DockerVersionGetter ();

    pub struct LinuxVersionGetter ();
    
    impl VersionGetter for DockerVersionGetter {
        fn version() -> Result<String, BaseError> {
            let command = "docker";
            let output = Command::new(command)
            .arg("--version")
            .output();
            let output_result = match output {
                Ok(command) => command,
                Err(error) => return Err(BaseError::error_executing_command(&error.to_string())),
            };
            let string_output = String::from_utf8((output_result.stdout).to_vec()).unwrap();
            let version = String::from(string_output.split_whitespace().nth(2).unwrap());
            return Ok(version);
        }

        fn version_status() -> VersionStatusDictionary {
            let verison = DockerVersionGetter::version();
            let verison_unwrapped = match verison {
                Ok(r) => r,
                Err(_err) => return VersionStatusDictionary::RED
            };
            println!("{}", verison_unwrapped);
            let main_version = verison_unwrapped.split(".").nth(0).unwrap().parse::<i32>().unwrap(); 
            if main_version > 16 {
                return VersionStatusDictionary::GREEN;
            }
            return  VersionStatusDictionary::RED;
        }
    }

    impl VersionGetter for LinuxVersionGetter {
        fn version() -> Result<String, BaseError> {
            let command = "lsb_release";
            if !(cfg!(unix)) {
                return Ok(String::from("NOT_LINUX 0.0.0")) ;
            }
            let output = Command::new(command)
            .arg("-a")
            .output();
            let output_unwrapped = match output {
                Ok(r) => r,
                Err(_err) => return Err(BaseError::error_executing_command(command))
            };
            let string_output = String::from_utf8((output_unwrapped.stdout).to_vec()).unwrap();
            print!("{}", string_output);
            let mut arr = string_output.split("\n");
            let release_string = arr.nth(1).unwrap().split(":").nth(1).unwrap();
            let result = trim_whitespace_v2(release_string);
            return Ok(result);
        }
        fn version_status() -> VersionStatusDictionary {
            let verison_either = LinuxVersionGetter::version();
            let verison = match verison_either {
                Ok(r) => r,
                Err(err) => return VersionStatusDictionary::RED
            };
            let mut tuple = verison.split_whitespace();
            let distribution = tuple.nth(0).unwrap();
            let mut version_number = tuple.nth(0).unwrap();
            let main_version = version_number.split(".").nth(0).unwrap();
            let secondary_version = version_number.split(".").nth(1).unwrap();
            println!("{} {}", main_version, secondary_version);
            if
                (distribution == "Ubuntu" && main_version == "18" && secondary_version == "04") 
                || 
                (distribution == "Ubuntu" && main_version == "22" && secondary_version == "04")
            {
                return VersionStatusDictionary::GREEN;
            } 
            if cfg!(unix) {
                return VersionStatusDictionary::YELLOW;
            }
            return VersionStatusDictionary::RED

        }
    }
    
    



