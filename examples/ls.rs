use termlauncher::Application;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    Application::new("ls")
        .with_arg("-la")
        .with_working_dir(&std::env::home_dir().expect("could not find home dir"))
        .with_class("dev.atomicptr.ls")
        .with_title("ls of home dir")
        .with_hold(true)
        .launch()
        .expect("expected to launch terminal with ls -la")
        .wait()
        .expect("expected to wait for application to quit");
}
