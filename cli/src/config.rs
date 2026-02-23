// Config types for chappie apply workflow.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// Directory containing rendered markdown files (relative to config file).
    pub output_dir: PathBuf,

    /// Named environments (e.g., "local").
    pub environments: HashMap<String, Environment>,
}

#[derive(Debug, Deserialize)]
pub struct Environment {
    /// Root path for this environment.
    ///
    /// Use `"."` for project-relative paths, or `"~"` for user home directory
    /// (`%USERPROFILE%` on Windows, `$HOME` on Unix). Absolute paths also work.
    pub root: PathBuf,

    /// Named targets within this environment (e.g., "claude", "roo").
    pub targets: HashMap<String, Target>,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    /// File copy mappings: source (from output_dir) → dest (relative to root).
    pub mappings: Vec<Mapping>,
}

#[derive(Debug, Deserialize)]
pub struct Mapping {
    /// Source filename inside output_dir.
    pub source: PathBuf,

    /// Destination path relative to the environment root.
    pub dest: PathBuf,
}

impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let config: Config =
            toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;
        Ok(config)
    }

    /// Returns the list of known target names for a given environment.
    pub fn target_names(&self, environment: &str) -> Option<Vec<&str>> {
        self.environments
            .get(environment)
            .map(|env| env.targets.keys().map(String::as_str).collect())
    }
}

#[cfg(test)]
mod tests {
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
}
