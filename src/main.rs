// Copyright (c) 2020  Teddy Wing
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


#![warn(rust_2018_idioms)]

use std::env;
use std::process;

use getopts::Options;
use git2::Repository;

use git_todo::Todos;


fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("V", "version", "show the program version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln(&e);
            process::exit(exitcode::NOINPUT);
        },
    };

    if matches.opt_present("h") {
        print_usage(&opts);
        process::exit(exitcode::USAGE);
    }

    if matches.opt_present("V") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        process::exit(exitcode::OK);
    }

    let repo = match Repository::open_from_env() {
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
    } else if matches.free.len() > 1 {
        eprintln(&"too many ref arguments");
        process::exit(exitcode::USAGE);
    } else {
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

/// Print command line usage.
fn print_usage(opts: &Options) {
    let brief = "usage: git todo [<commit>]";
    print!("{}", opts.usage(&brief));
}

/// Print to standard error with a program-specific prefix.
fn eprintln<D: std::fmt::Display>(error: &D) {
    eprintln!("error: {}", error);
}
