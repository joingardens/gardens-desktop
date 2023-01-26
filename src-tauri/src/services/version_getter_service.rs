use std::process::Command;
use std::env;
use crate::core::dictionaries::version_status_dictionary::VersionStatusDictionary;
    
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
        fn version() -> String;
        fn version_status() -> VersionStatusDictionary;
    }
    
    pub struct DockerVersionGetter ();

    pub struct LinuxVersionGetter ();
    
    impl VersionGetter for DockerVersionGetter {
        fn version() -> String {
            let output = Command::new("docker")
            .arg("--version")
            .output()
            .expect("failed to execute process");
            let string_output = String::from_utf8((output.stdout).to_vec()).unwrap();
            let mut arr = string_output.split_whitespace();
            String::from(arr.nth(2).unwrap())
        }

        fn version_status() -> VersionStatusDictionary {
            let mut verison = DockerVersionGetter::version();
            let main_version = verison.split(".").nth(0).unwrap().parse::<i32>().unwrap(); 
            if main_version > 16 {
                return VersionStatusDictionary::GREEN;
            }
            return  VersionStatusDictionary::RED;
        }
    }

    impl VersionGetter for LinuxVersionGetter {
        fn version() -> String {
            if !(cfg!(unix)) {
                return String::from("NOT_LINUX 0.0.0");
            }
            let output = Command::new("lsb_release")
            .arg("-a")
            .output()
            .expect("failed to execute process");
            let string_output = String::from_utf8((output.stdout).to_vec()).unwrap();
            print!("{}", string_output);
            let mut arr = string_output.split("\n");
            let mut release_string = arr.nth(1).unwrap().split(":").nth(1).unwrap();
            let result = trim_whitespace_v2(release_string);
            result
        }
        fn version_status() -> VersionStatusDictionary {
            let verison = LinuxVersionGetter::version();
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
    
    



