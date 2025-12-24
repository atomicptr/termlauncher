use termlauncher::{Application, CustomTerminal};

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    Application::new("btop")
        .with_class("dev.atomicptr.btop")
        .with_arg("--tty")
        .launch()
        .expect("expected to launch terminal with btop")
        .wait()
        .expect("expected to wait for application to quit");
}
