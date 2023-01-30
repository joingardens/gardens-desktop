use crate::core::dictionaries::version_status_dictionary::VersionStatusDictionary;
use std::process::Command;
use nix::unistd::Uid;

pub fn get_root_status() -> VersionStatusDictionary {
    if !Uid::effective().is_root() {
        return VersionStatusDictionary::RED;
    }
    return VersionStatusDictionary::GREEN;
}

pub fn get_docker_permission_status() -> VersionStatusDictionary {
    let mut output = Command::new("stat")
    .arg("--format")
    .arg("%a")
    .arg("/var/run/docker.sock")
    .output()
    .expect("failed to execute process");
    let binding = String::from_utf8(output.stdout.to_vec()).unwrap();
    let chmod_string:Vec<&str> = binding
    .lines()
    .collect();
    println!("{}", format!("{:?}", chmod_string));
    let chmod_number = chmod_string[0].parse::<i32>().unwrap();
    if chmod_number < 666 {
        return VersionStatusDictionary::RED;
    }
    return  VersionStatusDictionary::GREEN;

}