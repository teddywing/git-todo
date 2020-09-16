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

    let repo = Repository::open(".").unwrap();

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

        let oid = match repo.refname_to_id(&refname) {
            Ok(oid) => oid,
            Err(e) => {
                eprintln(&e);
                process::exit(exitcode::USAGE);
            },
        };

        match repo.find_tree(oid) {
            Ok(t) => t,
            Err(e) => {
                eprintln(&e);
                process::exit(exitcode::USAGE);
            },
        }
    };

    todos.write_since(tree, &mut std::io::stdout()).unwrap();
}

/// Print to standard error with a program-specific prefix.
fn eprintln<D: std::fmt::Display>(error: &D) {
    eprintln!("error: {}", error);
}
