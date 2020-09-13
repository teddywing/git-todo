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


use std::env;
use std::process;
use std::process::Command;

use exitcode;

use git_suggestion::{gseprintln, for_suggestion};
use git_suggestion::config::Config;
use git_suggestion::diff_options;


// TODO: one
fn main() {
    let args: Vec<_> = env::args().collect();

    let (args, diff_args) = diff_options::parse(&args);

    let config = match Config::get(
        &args,
        "usage: git sugdiff [options] <suggestion>...",
    ) {
        Ok(c) => c,
        Err(e) => {
            gseprintln!(e);

            process::exit(exitcode::CONFIG);
        },
    };

    // TODO two
    for_suggestion(
        &config,
        |suggestion| {
            let blob = match suggestion.blob() {
                Ok(b) => b,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
            };

            let mut child = match Command::new("git")
                .arg("--no-pager")
                .arg("diff")
                .args(&diff_args)
                .arg(format!("{}:{}", suggestion.commit(), suggestion.path()))
                .arg(blob.to_string())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
            };

            match child.wait() {
                Err(e) => {
                    gseprintln!(e);
                    process::exit(exitcode::UNAVAILABLE);
                },
                _ => (),
            };
        },
    );
}
