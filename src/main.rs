use std::env;
use std::process;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("missing argument");
        process::exit(1);
    }

    args.remove(0);
    args.remove(0);

    let status = Command::new("cargo")
        .arg("clean")
        .spawn()
        .expect("clean failed to execute")
        .wait()
        .expect("error while waiting for process");

    match status.code() {
        Some(0) => {}
        Some(code) => {
            eprintln!("`cargo clean` exited with status code {}", code);
            process::exit(code);
        }
        None => {
            eprintln!("`cargo clean` terminated by signal");
            process::exit(0);
        }
    }

    let status = Command::new("cargo")
        .arg("build")
        .args(&args)
        .spawn()
        .expect("process failed to execute")
        .wait()
        .expect("error while waiting for process");

    let mut command = vec![String::from("cargo"), String::from("build")];
    command.extend(args);

    let command = command.join(" ");

    match status.code() {
        Some(0) => {}
        Some(code) => {
            eprintln!("`{}` exited with status code {}", command, code);
            process::exit(code);
        }
        None => {
            eprintln!("`{}` terminated by signal", command);
            process::exit(0);
        }
    }
}
