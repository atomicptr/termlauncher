use std::process::Command;

use which::which;

use crate::Application;

#[derive(Debug, Default, Clone)]
pub struct CustomTerminal {
    pub executable: String,
    pub arguments: Vec<String>,
    pub title_arg: Option<String>,
    pub class_arg: Option<String>,
    pub hold_arg: Option<String>,
    pub cwd_arg: Option<String>,
    pub run_arg: Option<String>,
}

fn apply_arg(cmd: &mut Command, arg: Option<&String>, value: &str) {
    if let Some(arg) = arg {
        if arg.ends_with('=') {
            cmd.arg(format!("{arg}{value}"));
        } else {
            cmd.arg("{arg}");
            cmd.arg(value);
        }
    }
}

fn apply_flag(cmd: &mut Command, flag: Option<&String>) {
    if let Some(flag) = flag {
        cmd.arg(flag);
    }
}

pub fn build(term: &CustomTerminal, cmd: &mut Command, app: &Application) {
    for arg in &term.arguments {
        cmd.arg(arg);
    }

    if let Some(title) = &app.title {
        apply_arg(cmd, term.title_arg.as_ref(), title);
    }

    if cfg!(target_os = "linux")
        && let Some(class) = &app.class
    {
        apply_arg(cmd, term.class_arg.as_ref(), class);
    }

    if let Some(working_dir) = &app.working_dir {
        apply_arg(cmd, term.cwd_arg.as_ref(), &working_dir.to_string_lossy());
    }

    if app.hold {
        apply_flag(cmd, term.hold_arg.as_ref());
    }

    let exe = if let Ok(path) = which(&app.command) {
        path.to_string_lossy().to_string()
    } else {
        app.command.clone()
    };

    if let Some(run_arg) = term.run_arg.as_ref() {
        apply_arg(cmd, Some(run_arg), &exe);
    } else {
        cmd.arg("--");
        cmd.arg(exe);
    }

    for arg in &app.args {
        cmd.arg(arg);
    }
}
