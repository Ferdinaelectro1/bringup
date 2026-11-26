use std::assert_eq;

use crate::generate::load_versions;

#[test]
fn test_load_versions() {
    assert!(load_versions().is_ok());
}

#[test]
fn availabe_version_and_edition() {
    let versions = load_versions().expect("Error loading versions");
    assert_eq!(versions.package.edition.is_empty(),false);
    assert_eq!(versions.package.version.is_empty(),false);
}

#[test]
fn available_dependencies() {
    let versions = load_versions().expect("Error loading versions");
    assert_eq!(versions.dependencies.contains_key("embassy-executor-version"),true);
    assert_eq!(versions.dependencies.contains_key("embassy-time-version"),true);
    assert_eq!(versions.dependencies.contains_key("embassy-rp-version"),true);
}