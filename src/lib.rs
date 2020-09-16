#![warn(rust_2018_idioms)]

use std::io::Write;

use git2::{Repository, Tree};
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
    pub fn write_since<W: Write>(
        &self,
        tree: Tree<'_>,
        write_to: &mut W,
    ) -> Result<(), Error> {
        let diff = self.repo.diff_tree_to_workdir(Some(&tree), None)?;

        diff.foreach(
            &mut |_file, _progress| {
                true
            },
            None,
            None,
            Some(
                &mut |delta, hunk, line| {
                    // println!(
                    //     "d: {:?}, h: {:?}, l: {:?}",
                    //     delta,
                    //     hunk,
                    //     std::str::from_utf8(line.content()).unwrap(),
                    // );

                    if let Some(line_number) = line.new_lineno() {
                        if let Ok(l) = std::str::from_utf8(line.content()) {
                            if l.contains("TODO") {
                                if let Some(path) = delta.new_file().path() {
                                    write!(
                                        write_to,
                                        "{}:{}:{}",
                                        path.display(),
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

    pub fn master_tree(&self) -> Result<Tree<'_>, Error> {
        let master = self.repo.find_branch("master", git2::BranchType::Local)?;

        Ok(master.get().peel_to_tree()?)
    }
}
