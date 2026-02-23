// Tests for apply logic: file copying, tilde expansion, target filtering.

use super::*;
use crate::config::{Config, Environment, Mapping, Target};
use std::collections::HashMap;
use std::path::PathBuf;

fn make_config(targets: HashMap<String, Target>) -> Config {
    let mut environments = HashMap::new();
    environments.insert(
        "local".to_string(),
        Environment {
            root: PathBuf::from("."),
            targets,
        },
    );
    Config {
        output_dir: PathBuf::from("_output"),
        environments,
    }
}

#[test]
fn given_source_exists_when_apply_called_then_copies_file() {
    // Given: a temp dir with _output/agent.md and a target mapping
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("_output");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(output.join("agent.md"), "# Agent instructions").unwrap();

    let mut targets = HashMap::new();
    targets.insert(
        "claude".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("agent.md"),
                dest: PathBuf::from("CLAUDE.md"),
            }],
        },
    );
    let config = make_config(targets);

    // When: apply is called
    let result = apply(&config, dir.path(), "local", &[]);

    // Expect: destination file is created with matching content
    assert!(result.is_ok());
    let dest = dir.path().join("CLAUDE.md");
    assert!(dest.exists());
    assert_eq!(
        std::fs::read_to_string(dest).unwrap(),
        "# Agent instructions"
    );
}

#[test]
fn given_nested_dest_when_apply_called_then_creates_parent_dirs() {
    // Given: a source file and a dest path with subdirectories
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("_output");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(output.join("agent.md"), "content").unwrap();

    let mut targets = HashMap::new();
    targets.insert(
        "roo".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("agent.md"),
                dest: PathBuf::from(".roo/rules-code.md"),
            }],
        },
    );
    let config = make_config(targets);

    // When: apply is called
    let result = apply(&config, dir.path(), "local", &[]);

    // Expect: nested directory and file are created
    assert!(result.is_ok());
    let dest = dir.path().join(".roo/rules-code.md");
    assert!(dest.exists());
}

#[test]
fn given_missing_source_when_apply_called_then_returns_error() {
    // Given: no source file exists
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("_output");
    std::fs::create_dir_all(&output).unwrap();

    let mut targets = HashMap::new();
    targets.insert(
        "claude".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("missing.md"),
                dest: PathBuf::from("CLAUDE.md"),
            }],
        },
    );
    let config = make_config(targets);

    // When: apply is called
    let result = apply(&config, dir.path(), "local", &[]);

    // Expect: error mentioning the missing source
    assert!(result.is_err());
    let err = format!("{:#}", result.unwrap_err());
    assert!(err.contains("source file not found"), "got: {err}");
}

#[test]
fn given_unknown_target_when_apply_called_then_returns_error() {
    // Given: config with "claude" target only
    let mut targets = HashMap::new();
    targets.insert(
        "claude".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("a.md"),
                dest: PathBuf::from("b.md"),
            }],
        },
    );
    let config = make_config(targets);
    let dir = tempfile::tempdir().unwrap();

    // When: apply is called with an unknown target
    let result = apply(
        &config,
        dir.path(),
        "local",
        &["nope".to_string()],
    );

    // Expect: error about unknown target
    assert!(result.is_err());
    let err = format!("{:#}", result.unwrap_err());
    assert!(err.contains("unknown target"), "got: {err}");
}

#[test]
fn given_specific_targets_when_apply_called_then_only_applies_those() {
    // Given: two targets but only one requested
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("_output");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(output.join("agent.md"), "content").unwrap();

    let mut targets = HashMap::new();
    targets.insert(
        "claude".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("agent.md"),
                dest: PathBuf::from("CLAUDE.md"),
            }],
        },
    );
    targets.insert(
        "codex".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("agent.md"),
                dest: PathBuf::from("AGENTS.md"),
            }],
        },
    );
    let config = make_config(targets);

    // When: apply is called for only "claude"
    let result = apply(
        &config,
        dir.path(),
        "local",
        &["claude".to_string()],
    );

    // Expect: only CLAUDE.md exists, not AGENTS.md
    assert!(result.is_ok());
    assert!(dir.path().join("CLAUDE.md").exists());
    assert!(!dir.path().join("AGENTS.md").exists());
}

#[test]
fn given_tilde_root_when_apply_called_then_expands_to_home_dir() {
    // Given: a source file and config with root = "~"
    let dir = tempfile::tempdir().unwrap();
    let output = dir.path().join("_output");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(output.join("agent.md"), "global content").unwrap();

    let mut targets = HashMap::new();
    targets.insert(
        "claude".to_string(),
        Target {
            mappings: vec![Mapping {
                source: PathBuf::from("agent.md"),
                dest: PathBuf::from("test-chappie-apply/CLAUDE.md"),
            }],
        },
    );
    let mut environments = HashMap::new();
    environments.insert(
        "global".to_string(),
        Environment {
            root: PathBuf::from("~"),
            targets,
        },
    );
    let config = Config {
        output_dir: PathBuf::from("_output"),
        environments,
    };

    // When: apply is called for global environment
    let result = apply(&config, dir.path(), "global", &[]);

    // Expect: file is written under the home directory
    assert!(result.is_ok());
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .expect("HOME or USERPROFILE must be set");
    let dest = home.join("test-chappie-apply/CLAUDE.md");
    assert!(dest.exists(), "expected {} to exist", dest.display());
    assert_eq!(std::fs::read_to_string(&dest).unwrap(), "global content");

    // Cleanup
    let _ = std::fs::remove_dir_all(home.join("test-chappie-apply"));
}

#[test]
fn given_non_tilde_path_when_expand_tilde_called_then_returns_unchanged() {
    // Given: a relative path without ~
    let path = Path::new("some/relative/path");

    // When: expand_tilde is called
    let result = expand_tilde(path);

    // Expect: path is unchanged
    assert_eq!(result, PathBuf::from("some/relative/path"));
}

#[test]
fn given_tilde_path_when_expand_tilde_called_then_prepends_home() {
    // Given: a path starting with ~
    let path = Path::new("~/.claude/CLAUDE.md");

    // When: expand_tilde is called
    let result = expand_tilde(path);

    // Expect: ~ is replaced with the home directory
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .expect("HOME or USERPROFILE must be set");
    assert_eq!(result, home.join(".claude/CLAUDE.md"));
}
