#![warn(rust_2018_idioms)]

use git2::Repository;

use git_todo::Todos;


fn main() {
    let repo = Repository::open(".").unwrap();

    let todos = Todos { repo: &repo };
    todos.write_since(todos.master_tree().unwrap(), &mut std::io::stdout()).unwrap();
}
