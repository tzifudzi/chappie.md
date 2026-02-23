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
#[path = "config_test.rs"]
mod tests;
