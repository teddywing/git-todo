#![warn(rust_2018_idioms)]

use std::env;
use std::process;

use getopts::Options;
use git2::Repository;

use git_todo::Todos;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln(&e);
            process::exit(exitcode::NOINPUT);
        },
    };

    let repo = match Repository::open(".") {
        Ok(r) => r,
        Err(e) => {
            eprintln(&format!("unable to open repository: {}", e));
            process::exit(exitcode::NOINPUT);
        },
    };

    let todos = Todos { repo: &repo };

    let tree = if matches.free.is_empty() {
        match todos.master_tree() {
            Ok(t) => t,
            Err(e) => {
                eprintln(&e);
                process::exit(exitcode::USAGE);
            },
        }
    } else {
        // TODO: error if more than one ref given
        let refname = &matches.free[0];

        let object = match repo.revparse_single(&refname) {
            Ok(object) => object,
            Err(e) => {
                eprintln(&e);
                process::exit(exitcode::USAGE);
            },
        };

        match object.peel_to_tree() {
            Ok(t) => t,
            Err(e) => {
                eprintln(&e);
                process::exit(exitcode::USAGE);
            },
        }
    };

    match todos.write_since(tree, &mut std::io::stdout()) {
        Err(e) => {
            eprintln(&e);
            process::exit(exitcode::UNAVAILABLE);
        },
        _ => (),
    };
}

/// Print to standard error with a program-specific prefix.
fn eprintln<D: std::fmt::Display>(error: &D) {
    eprintln!("error: {}", error);
}
