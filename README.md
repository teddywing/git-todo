git-todo
========

Output TODO lines since `master` or a specified ref.


## Example

	$ git todo b0aa9334017e433225175d323732472c35f51aed
	src/lib.rs:38:                            if l.contains("TODO") {
	src/main.rs:38:        // TODO: error if more than one ref given


## Install
On Mac OS X, Git-Todo can be installed with Homebrew:

	$ brew install teddywing/formulae/git-todo

To compile from source or install on other platforms:

	$ cargo install --git https://github.com/teddywing/git-todo.git


## Uninstall

	$ cargo uninstall git-todo


## License
Copyright © 2020 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).
