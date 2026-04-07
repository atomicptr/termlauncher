use termlauncher::Application;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    Application::new("printenv")
        .with_arg("APP_NAME")
        .with_env_var("APP_NAME", "termlauncher")
        .with_hold(true)
        .launch()
        .expect("expected to launch terminal with printenv")
        .wait()
        .expect("expected to wait for application to quit");
}
