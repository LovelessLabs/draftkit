//! Package manager detection and command generation.
//!
//! Detection priority:
//! 1. Existing lockfile in directory
//! 2. User config preference
//! 3. Installed tooling (prefer bun > pnpm > yarn > npm by speed)
//! 4. Fallback: npm

use std::process::Command;

use camino::Utf8Path;
use serde::{Deserialize, Serialize};

/// JavaScript package manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    #[default]
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

impl PackageManager {
    /// Detect the appropriate package manager for a directory.
    ///
    /// Priority order:
    /// 1. Existing lockfile in directory (bun.lockb → pnpm-lock.yaml → yarn.lock → package-lock.json)
    /// 2. User config preference (if provided)
    /// 3. Installed tooling (prefer bun > pnpm > yarn > npm by speed)
    /// 4. Fallback: npm
    #[must_use]
    pub fn detect(dir: &Utf8Path, config_preference: Option<Self>) -> Self {
        // 1. Check for existing lockfiles (highest priority to preserve consistency)
        if let Some(pm) = Self::detect_from_lockfile(dir) {
            return pm;
        }

        // 2. User config preference
        if let Some(preferred) = config_preference {
            return preferred;
        }

        // 3. Installed tooling (prefer faster options)
        Self::detect_installed()
    }

    /// Detect package manager from lockfile presence.
    #[must_use]
    fn detect_from_lockfile(dir: &Utf8Path) -> Option<Self> {
        let dir = dir.as_std_path();

        // Order by preference (bun is fastest, npm is most compatible)
        if dir.join("bun.lockb").exists() || dir.join("bun.lock").exists() {
            return Some(Self::Bun);
        }
        if dir.join("pnpm-lock.yaml").exists() {
            return Some(Self::Pnpm);
        }
        if dir.join("yarn.lock").exists() {
            return Some(Self::Yarn);
        }
        if dir.join("package-lock.json").exists() {
            return Some(Self::Npm);
        }

        None
    }

    /// Detect installed package managers, preferring faster ones.
    #[must_use]
    fn detect_installed() -> Self {
        // Try each in order of speed preference
        if Self::is_installed("bun") {
            return Self::Bun;
        }
        if Self::is_installed("pnpm") {
            return Self::Pnpm;
        }
        if Self::is_installed("yarn") {
            return Self::Yarn;
        }

        // npm is always the fallback (installed with Node.js)
        Self::Npm
    }

    /// Check if a command is available in PATH.
    fn is_installed(cmd: &str) -> bool {
        Command::new(cmd)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
    }

    /// Get the CLI command name.
    #[must_use]
    pub const fn command(&self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::Yarn => "yarn",
            Self::Bun => "bun",
        }
    }

    /// Get the install command (for package.json dependencies).
    #[must_use]
    pub const fn install_cmd(&self) -> &'static [&'static str] {
        match self {
            Self::Npm => &["npm", "install"],
            Self::Pnpm => &["pnpm", "install"],
            Self::Yarn => &["yarn", "install"],
            Self::Bun => &["bun", "install"],
        }
    }

    /// Get the dev server command.
    #[must_use]
    pub const fn dev_cmd(&self) -> &'static [&'static str] {
        match self {
            Self::Npm => &["npm", "run", "dev"],
            Self::Pnpm => &["pnpm", "dev"],
            Self::Yarn => &["yarn", "dev"],
            Self::Bun => &["bun", "run", "dev"],
        }
    }

    /// Get the add dependency command.
    #[must_use]
    pub fn add_cmd(&self, packages: &[&str], dev: bool) -> Vec<String> {
        let mut cmd = vec![self.command().to_string()];

        match self {
            Self::Npm => {
                cmd.push("install".to_string());
                if dev {
                    cmd.push("--save-dev".to_string());
                }
            }
            Self::Pnpm => {
                cmd.push("add".to_string());
                if dev {
                    cmd.push("-D".to_string());
                }
            }
            Self::Yarn => {
                cmd.push("add".to_string());
                if dev {
                    cmd.push("-D".to_string());
                }
            }
            Self::Bun => {
                cmd.push("add".to_string());
                if dev {
                    cmd.push("-d".to_string());
                }
            }
        }

        cmd.extend(packages.iter().map(|s| (*s).to_string()));
        cmd
    }

    /// Get the lockfile name for this package manager.
    #[must_use]
    pub const fn lockfile(&self) -> &'static str {
        match self {
            Self::Npm => "package-lock.json",
            Self::Pnpm => "pnpm-lock.yaml",
            Self::Yarn => "yarn.lock",
            Self::Bun => "bun.lockb",
        }
    }

    /// Parse from string (case-insensitive).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "npm" => Some(Self::Npm),
            "pnpm" => Some(Self::Pnpm),
            "yarn" => Some(Self::Yarn),
            "bun" => Some(Self::Bun),
            _ => None,
        }
    }

    /// Get the string representation.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::Yarn => "yarn",
            Self::Bun => "bun",
        }
    }
}

impl std::fmt::Display for PackageManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for PackageManager {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s).ok_or_else(|| format!("Unknown package manager: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn parse_package_managers() {
        assert_eq!(PackageManager::parse("npm"), Some(PackageManager::Npm));
        assert_eq!(PackageManager::parse("NPM"), Some(PackageManager::Npm));
        assert_eq!(PackageManager::parse("pnpm"), Some(PackageManager::Pnpm));
        assert_eq!(PackageManager::parse("yarn"), Some(PackageManager::Yarn));
        assert_eq!(PackageManager::parse("bun"), Some(PackageManager::Bun));
        assert_eq!(PackageManager::parse("invalid"), None);
    }

    #[test]
    fn detect_from_npm_lockfile() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("package-lock.json"), "{}").unwrap();

        let path = Utf8Path::from_path(dir.path()).unwrap();
        let detected = PackageManager::detect(path, None);
        assert_eq!(detected, PackageManager::Npm);
    }

    #[test]
    fn detect_from_pnpm_lockfile() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("pnpm-lock.yaml"), "").unwrap();

        let path = Utf8Path::from_path(dir.path()).unwrap();
        let detected = PackageManager::detect(path, None);
        assert_eq!(detected, PackageManager::Pnpm);
    }

    #[test]
    fn detect_from_yarn_lockfile() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("yarn.lock"), "").unwrap();

        let path = Utf8Path::from_path(dir.path()).unwrap();
        let detected = PackageManager::detect(path, None);
        assert_eq!(detected, PackageManager::Yarn);
    }

    #[test]
    fn detect_from_bun_lockfile() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("bun.lockb"), "").unwrap();

        let path = Utf8Path::from_path(dir.path()).unwrap();
        let detected = PackageManager::detect(path, None);
        assert_eq!(detected, PackageManager::Bun);
    }

    #[test]
    fn lockfile_takes_precedence_over_config() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("yarn.lock"), "").unwrap();

        let path = Utf8Path::from_path(dir.path()).unwrap();
        // Even with pnpm preference, yarn lockfile wins
        let detected = PackageManager::detect(path, Some(PackageManager::Pnpm));
        assert_eq!(detected, PackageManager::Yarn);
    }

    #[test]
    fn config_preference_used_when_no_lockfile() {
        let dir = TempDir::new().unwrap();
        let path = Utf8Path::from_path(dir.path()).unwrap();

        let detected = PackageManager::detect(path, Some(PackageManager::Pnpm));
        assert_eq!(detected, PackageManager::Pnpm);
    }

    #[test]
    fn install_commands() {
        assert_eq!(PackageManager::Npm.install_cmd(), &["npm", "install"]);
        assert_eq!(PackageManager::Pnpm.install_cmd(), &["pnpm", "install"]);
        assert_eq!(PackageManager::Yarn.install_cmd(), &["yarn", "install"]);
        assert_eq!(PackageManager::Bun.install_cmd(), &["bun", "install"]);
    }

    #[test]
    fn dev_commands() {
        assert_eq!(PackageManager::Npm.dev_cmd(), &["npm", "run", "dev"]);
        assert_eq!(PackageManager::Pnpm.dev_cmd(), &["pnpm", "dev"]);
        assert_eq!(PackageManager::Yarn.dev_cmd(), &["yarn", "dev"]);
        assert_eq!(PackageManager::Bun.dev_cmd(), &["bun", "run", "dev"]);
    }

    #[test]
    fn add_commands() {
        let npm_add = PackageManager::Npm.add_cmd(&["react", "react-dom"], false);
        assert_eq!(npm_add, vec!["npm", "install", "react", "react-dom"]);

        let npm_add_dev = PackageManager::Npm.add_cmd(&["typescript"], true);
        assert_eq!(
            npm_add_dev,
            vec!["npm", "install", "--save-dev", "typescript"]
        );

        let pnpm_add = PackageManager::Pnpm.add_cmd(&["react"], false);
        assert_eq!(pnpm_add, vec!["pnpm", "add", "react"]);

        let pnpm_add_dev = PackageManager::Pnpm.add_cmd(&["typescript"], true);
        assert_eq!(pnpm_add_dev, vec!["pnpm", "add", "-D", "typescript"]);
    }

    #[test]
    fn serde_roundtrip() {
        let json = serde_json::to_string(&PackageManager::Pnpm).unwrap();
        assert_eq!(json, "\"pnpm\"");

        let parsed: PackageManager = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, PackageManager::Pnpm);
    }

    #[test]
    fn display_impl() {
        assert_eq!(format!("{}", PackageManager::Npm), "npm");
        assert_eq!(format!("{}", PackageManager::Bun), "bun");
    }

    #[test]
    fn from_str_impl() {
        assert_eq!(
            "npm".parse::<PackageManager>().unwrap(),
            PackageManager::Npm
        );
        assert!("invalid".parse::<PackageManager>().is_err());
    }
}
