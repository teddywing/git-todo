SOURCES := $(shell find . -name '*.rs')
DEPENDENCIES := Cargo.toml
DEBUG_PRODUCT := target/debug/git-todo


.PHONY: test
test: $(DEBUG_PRODUCT)
	prove -v -I./t

$(DEBUG_PRODUCT): $(SOURCES) $(DEPENDENCIES)
	cargo build
