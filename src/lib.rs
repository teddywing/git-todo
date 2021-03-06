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

use std::io::Write;
use std::path::{Path, PathBuf};

use git2::{DiffOptions, Repository, Tree};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Git(#[from] git2::Error),
}


pub struct Todos<'a> {
    pub repo: &'a Repository,
}

impl Todos<'_> {
    /// Write TODO lines.
    ///
    /// Writes TODO lines since `tree` to `write_to`.
    ///
    /// # Panics
    ///
    /// Panics if writing to `write_to` fails.
    pub fn write_since<W: Write>(
        &self,
        tree: Tree<'_>,
        write_to: &mut W,
    ) -> Result<(), Error> {
        let mut diff_options = DiffOptions::new();
        diff_options
            .show_untracked_content(true)
            .recurse_untracked_dirs(true);

        let diff = self.repo.diff_tree_to_workdir_with_index(
            Some(&tree),
            Some(&mut diff_options),
        )?;

        diff.foreach(
            &mut |_file, _progress| {
                true
            },
            None,
            None,
            Some(
                &mut |delta, _hunk, line| {
                    if let Some(line_number) = line.new_lineno() {
                        if let Ok(l) = std::str::from_utf8(line.content()) {
                            if l.contains("TODO") {
                                if let Some(path) = delta.new_file().path() {
                                    write!(
                                        write_to,
                                        "{}:{}:{}",
                                        relative_path(self.repo, path).display(),
                                        line_number,
                                        l,
                                    ).expect("write error");
                                }
                            }
                        }
                    }

                    true
                }
            ),
        )?;

        Ok(())
    }

    /// Get a Git tree for the master branch.
    pub fn master_tree(&self) -> Result<Tree<'_>, Error> {
        let master = self.repo.find_branch("master", git2::BranchType::Local)?;

        Ok(master.get().peel_to_tree()?)
    }
}

/// Get `path` relative to the current directory.
///
/// Attempt to remove the current directory prefix from `path` to turn it into a
/// relative path from the current directory.
fn relative_path(repo: &Repository, path: &Path) -> PathBuf {
    let workdir = match repo.workdir() {
        Some(d) => d,
        None => return path.to_path_buf(),
    };

    let current_dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => return path.to_path_buf(),
    };

    let current_dir_relative = match current_dir.strip_prefix(workdir) {
        Ok(d) => d,
        Err(_) => return path.to_path_buf(),
    };

    match path.strip_prefix(current_dir_relative) {
        Ok(d) => d.to_path_buf(),
        Err(_) => path.to_path_buf(),
    }
}
