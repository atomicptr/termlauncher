use std::process::Command;

use log::debug;
use which::which;

use crate::Application;

pub fn build(cmd: &mut Command, app: &Application) {
    if let Some(title) = &app.title {
        cmd.arg("-t");
        cmd.arg(title);
    }

    if cfg!(target_os = "linux")
        && let Some(class) = &app.class
    {
        cmd.arg("-c");
        cmd.arg(class);
    }

    if app.working_dir.is_some() {
        debug!("st does not support setting working directory");
    }

    if app.hold {
        debug!("st does not support holding the terminal open");
    }

    cmd.arg("-e");

    if let Ok(path) = which(&app.command) {
        cmd.arg(path);
    } else {
        cmd.arg(app.command.clone());
    }

    for arg in &app.args {
        cmd.arg(arg);
    }
}
