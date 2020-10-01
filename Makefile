.PHONY: test
test: target/debug/git-todo
	prove -v -I./t

target/debug/git-todo:
	cargo build
