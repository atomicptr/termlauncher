use std::process::Command;

use which::which;

use crate::Application;

pub fn build(cmd: &mut Command, app: &Application) {
    if let Some(title) = &app.title {
        cmd.arg(format!("--title={title}"));
    }

    if cfg!(target_os = "linux")
        && let Some(class) = &app.class
    {
        cmd.arg(format!("--class={class}"));
    }

    if let Some(working_dir) = &app.working_dir {
        cmd.arg(format!("--working-directory={}", working_dir.display()));
    }

    if app.hold {
        cmd.arg("--wait-after-command=true");
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
