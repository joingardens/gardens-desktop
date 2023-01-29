use crate::core::dictionaries::version_status_dictionary::VersionStatusDictionary;
use nix::unistd::Uid;

pub fn get_root_status() -> VersionStatusDictionary {
    if !Uid::effective().is_root() {
        return VersionStatusDictionary::RED;
    }
    return VersionStatusDictionary::GREEN;
}