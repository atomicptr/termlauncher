# termlauncher

Your app, the user's terminal, done.

This library provides a unified interface for spawning terminal applications in their own window using a terminal the
user has installed.

## Usage

```rust
use termlauncher::Application;

fn main() {
    Application::new("ls") // run the `ls` command
        .with_arg("-la") // with the flag `-la`
        .with_working_dir(&std::env::home_dir().expect("could not find home dir")) // in the home dir
        .with_title("ls of home dir") // set the terminal title
        .with_hold(true) // keep the window open
        .launch() // launch it in a supported terminal, returns std::process::Child
        .expect("expected to launch terminal with ls -la")
        .wait()
        .expect("expected to wait for application to quit");
}
```

Also, check out the examples!

## Motivation

I kinda have the problem a lot that I want to start a terminal application in a separate window and every time so far
I've reimplemented support for the same terminals and thought this would be a good fit for a library.

## License

MIT
