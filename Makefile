SOURCES := $(shell find . -name '*.rs')
DEPENDENCIES := Cargo.toml
DEBUG_PRODUCT := target/debug/git-todo


.PHONY: doc
doc: doc/git-todo.1

doc/git-todo.1: doc/git-todo.1.txt
	sed 's/`/*/g' $< > $@.transformed
	a2x --no-xmllint --format manpage $@.transformed
	rm $@.transformed


.PHONY: test
test: $(DEBUG_PRODUCT)
	prove -v -I./t

$(DEBUG_PRODUCT): $(SOURCES) $(DEPENDENCIES)
	cargo build
