#![warn(rust_2018_idioms)]

use std::io::Write;

use git2::{Repository, Tree};


pub struct Todos<'a> {
    pub repo: &'a Repository,
}

impl Todos<'_> {
    pub fn write_since<W: Write>(&self, tree: Tree<'_>, write_to: &mut W) {
        let diff = self.repo.diff_tree_to_workdir(Some(&tree), None).unwrap();

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
                        let l = std::str::from_utf8(line.content()).unwrap();

                        if l.contains("TODO") {
                            write!(
                                write_to,
                                "{}:{}:{}",
                                delta.new_file().path().unwrap().display(),
                                line_number,
                                l,
                            ).unwrap();
                        }
                    }

                    true
                }
            ),
        ).unwrap();
    }

    pub fn master_tree(&self) -> Tree<'_> {
        let master = self.repo.find_branch("master", git2::BranchType::Local).unwrap();

        master.get().peel_to_tree().unwrap()
    }
}
