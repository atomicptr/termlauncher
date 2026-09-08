use std::env;
use std::{path::PathBuf, process::Command};

use log::debug;
use which::which;

use crate::Application;
use crate::error::{Error, Result};
use crate::terminals::custom::{self, CustomTerminal};
use crate::terminals::{alacritty, foot, ghostty, kitty, st, wezterm};

#[derive(Debug, Clone)]
pub enum Terminal {
    Alacritty,
    Foot,
    Ghostty,
    Kitty,
    ST,
    WezTerm,
    Custom(CustomTerminal),
}

impl Terminal {
    const ALL: [Terminal; 6] = [
        Self::Alacritty,
        Self::Foot,
        Self::Ghostty,
        Self::Kitty,
        Self::ST,
        Self::WezTerm,
    ];

    /// Build a `std::process::Command` for the given application
    ///
    /// # Errors
    /// `Error:TerminalNotFound`
    pub fn build_command(&self, app: &Application) -> Result<Command> {
        let mut cmd = Command::new(self.executable_path()?);

        match self {
            Terminal::Alacritty => alacritty::build(&mut cmd, app),
            Terminal::Foot => foot::build(&mut cmd, app),
            Terminal::Ghostty => ghostty::build(&mut cmd, app),
            Terminal::Kitty => kitty::build(&mut cmd, app),
            Terminal::ST => st::build(&mut cmd, app),
            Terminal::WezTerm => wezterm::build(&mut cmd, app),
            Terminal::Custom(term) => custom::build(term, &mut cmd, app),
        }

        cmd.envs(app.env_vars.clone());

        Ok(cmd)
    }

    fn executable_name(&self) -> String {
        match self {
            Self::Alacritty => "alacritty".to_string(),
            Self::Foot => "foot".to_string(),
            Self::Ghostty => "ghostty".to_string(),
            Self::Kitty => "kitty".to_string(),
            Self::ST => "st".to_string(),
            Self::WezTerm => "wezterm".to_string(),
            Self::Custom(term) => term.executable.to_lowercase(),
        }
    }

    fn executable_path(&self) -> Result<PathBuf> {
        let name = self.executable_name();

        if let Ok(path) = which(&name) {
            return Ok(path);
        }

        #[cfg(target_os = "macos")]
        if let Some(path) = find_macos_executable_in_app_bundle(&name) {
            return Ok(path);
        }

        Err(Error::TerminalNotFound(name))
    }

    #[must_use]
    pub fn is_available(&self) -> bool {
        self.executable_path().is_ok()
    }

    #[must_use]
    pub fn find_by_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "alacritty" => Some(Terminal::Alacritty),
            "foot" => Some(Terminal::Foot),
            "ghostty" => Some(Terminal::Ghostty),
            "kitty" => Some(Terminal::Kitty),
            "st" => Some(Terminal::ST),
            "wezterm" => Some(Terminal::WezTerm),
            _ => None,
        }
    }

    #[must_use]
    pub fn find_available() -> Option<Self> {
        // if TERMLAUNCHER is set and we support the terminal use that
        if let Ok(name) = env::var("TERMLAUNCHER")
            && let Some(term) = Self::find_by_name(&name)
        {
            debug!("Found terminal: {term:?} via env var TERMLAUNCHER");
            return Some(term);
        }

        // if TERMCMD is set and we support the terminal use that
        if let Ok(name) = env::var("TERMCMD")
            && let Some(term) = Self::find_by_name(&name)
        {
            debug!("Found terminal: {term:?} via env var TERMCMD");
            return Some(term);
        }

        for term in &Self::ALL {
            if term.is_available() {
                debug!("Found terminal: {term:?}");
                return Some(term.clone());
            }
        }

        None
    }
}

#[cfg(target_os = "macos")]
fn find_macos_executable_in_app_bundle(name: &str) -> Option<PathBuf> {
    // support /Applications
    let mut roots = vec![PathBuf::from("/Applications")];

    // also support for /Users/name/Applications
    if let Some(home) = env::home_dir() {
        roots.push(home.join("Applications"));
    }

    // for all application roots
    for root in roots {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            // if its not an "*.app" continue
            if path.extension().is_none_or(|e| e != "app") {
                continue;
            }

            let candidate = path.join("Contents/MacOS").join(name);

            // if the .app bundle contains the file we're looking for
            if candidate.is_file() {
                debug!("Found macOS bundle executable: {}", candidate.display());
                return Some(candidate);
            }
        }
    }

    None
}
