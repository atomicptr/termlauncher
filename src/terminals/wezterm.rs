use std::process::Command;

use log::debug;
use which::which;

use crate::Application;

pub fn build(cmd: &mut Command, app: &Application) {
    if app.hold {
        cmd.arg("--config");
        cmd.arg("exit_behavior='Hold'");
    }

    cmd.arg("start");

    if app.title.is_some() {
        debug!("wezterm does not support setting title");
    }

    if cfg!(target_os = "linux")
        && let Some(class) = &app.class
    {
        cmd.arg("--class");
        cmd.arg(class);
    }

    if let Some(working_dir) = &app.working_dir {
        cmd.arg("--cwd");
        cmd.arg(working_dir);
    }

    cmd.arg("--");

    if let Ok(path) = which(&app.command) {
        cmd.arg(path);
    } else {
        cmd.arg(app.command.clone());
    }

    for arg in &app.args {
        cmd.arg(arg);
    }
}
