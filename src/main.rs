#![warn(rust_2018_idioms)]

extern crate git2;

use git2::Repository;


fn main() {
    let repo = Repository::open(".").unwrap();

    // let head = repo.head().unwrap().target().unwrap();
    let master = repo.find_branch("master", git2::BranchType::Local).unwrap();
    // let merge_base = repo.merge_base(head, master.get().target().unwrap()).unwrap();

    let tree = master.get().peel_to_tree().unwrap();
    let diff = repo.diff_tree_to_workdir(Some(&tree), None).unwrap();

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

                let l = std::str::from_utf8(line.content()).unwrap();

                if l.contains("TODO") {
                    println!(
                        "{}:{}:{}",
                        delta.new_file().path().unwrap().display(),
                        line.new_lineno().unwrap(),
                        l,
                    );
                }

                true
            }
        ),
    ).unwrap();
}
