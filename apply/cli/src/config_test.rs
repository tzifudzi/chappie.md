// Tests for config types and loading.

use super::*;
use std::io::Write;

#[test]
fn given_valid_toml_when_load_called_then_parses_all_fields() {
    // Given: a valid config TOML file
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    write!(
        tmp,
        r#"
output_dir = "_output"

[environments.local]
root = "."

[environments.local.targets.claude]
mappings = [
    {{ source = "coder.md", dest = "CLAUDE.md" }},
]
"#
    )
    .unwrap();

    // When: config is loaded
    let config = Config::load(tmp.path()).unwrap();

    // Expect: fields are parsed correctly
    assert_eq!(config.output_dir, PathBuf::from("_output"));
    assert!(config.environments.contains_key("local"));

    let env = &config.environments["local"];
    assert_eq!(env.root, PathBuf::from("."));
    assert!(env.targets.contains_key("claude"));

    let claude = &env.targets["claude"];
    assert_eq!(claude.mappings.len(), 1);
    assert_eq!(claude.mappings[0].source, PathBuf::from("coder.md"));
    assert_eq!(claude.mappings[0].dest, PathBuf::from("CLAUDE.md"));
}

#[test]
fn given_valid_config_when_target_names_called_then_returns_known_targets() {
    // Given: a config with two targets
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    write!(
        tmp,
        r#"
output_dir = "_output"

[environments.local]
root = "."

[environments.local.targets.claude]
mappings = [{{ source = "a.md", dest = "b.md" }}]

[environments.local.targets.roo]
mappings = [{{ source = "a.md", dest = "c.md" }}]
"#
    )
    .unwrap();

    // When: target_names is called
    let config = Config::load(tmp.path()).unwrap();
    let mut names = config.target_names("local").unwrap();
    names.sort();

    // Expect: both targets are returned
    assert_eq!(names, vec!["claude", "roo"]);
}

#[test]
fn given_unknown_environment_when_target_names_called_then_returns_none() {
    // Given: a config with only "local"
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    write!(
        tmp,
        r#"
output_dir = "_output"

[environments.local]
root = "."

[environments.local.targets.claude]
mappings = [{{ source = "a.md", dest = "b.md" }}]
"#
    )
    .unwrap();

    let config = Config::load(tmp.path()).unwrap();

    // Expect: unknown environment returns None
    assert!(config.target_names("prod").is_none());
}
