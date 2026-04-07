use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Child,
};

use log::debug;

use crate::{Error, Result, Terminal};

/// Configurates an application to be run
#[derive(Debug)]
pub struct Application {
    pub command: String,
    pub args: Vec<String>,
    pub title: Option<String>,
    pub class: Option<String>,
    pub working_dir: Option<PathBuf>,
    pub hold: bool,
    pub env_vars: HashMap<String, String>,
}

impl Application {
    /// Create a new application with the given command
    #[must_use]
    pub fn new(cmd: &str) -> Self {
        Self {
            command: cmd.to_string(),
            args: Vec::new(),
            title: None,
            class: None,
            working_dir: None,
            hold: false,
            env_vars: HashMap::new(),
        }
    }

    /// Add an argument to the command
    #[must_use]
    pub fn with_arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    /// Add multiple arguments to the command
    #[must_use]
    pub fn with_args(mut self, args: &Vec<&str>) -> Self {
        self.args.extend(args.iter().map(ToString::to_string));
        self
    }

    /// Set the title of the terminal
    #[must_use]
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set the class of the terminal (Linux only)
    #[must_use]
    pub fn with_class(mut self, class: &str) -> Self {
        self.class = Some(class.to_string());
        self
    }

    /// Set the working directory of the terminal
    #[must_use]
    pub fn with_working_dir(mut self, path: &Path) -> Self {
        self.working_dir = Some(path.into());
        self
    }

    /// Keep the terminal open after the command quits
    #[must_use]
    pub fn with_hold(mut self, keep_open: bool) -> Self {
        self.hold = keep_open;
        self
    }

    #[must_use]
    pub fn with_env_var(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }

    /// Launch the application in an available terminal
    ///
    /// # Errors
    /// `Error::NoSupportedTerminalAvailable`
    /// `Error::IOError`
    pub fn launch(&self) -> Result<Child> {
        if let Some(term) = Terminal::find_available() {
            return self.launch_with(&term);
        }

        Err(Error::NoSupportedTerminalAvailable)
    }

    /// Launch the application with a specific terminal
    ///
    /// # Errors
    /// `Error::TerminalNotFound`
    /// `Error::IOError`
    pub fn launch_with(&self, terminal: &Terminal) -> Result<Child> {
        let mut cmd = terminal.build_command(self)?;

        debug!("Launching: {cmd:?}");

        Ok(cmd.spawn()?)
    }
}
