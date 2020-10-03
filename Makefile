# Copyright (c) 2020  Teddy Wing
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.


VERSION := $(shell egrep '^version = ' Cargo.toml | awk -F '"' '{ print $$2 }')
TOOLCHAIN := $(shell fgrep default_host_triple $(HOME)/.rustup/settings.toml | awk -F '"' '{ print $$2 }')

SOURCES := $(shell find . -name '*.rs')
DEPENDENCIES := Cargo.toml

MAN_PAGE := doc/git-todo.1

DEBUG_PRODUCT := target/debug/git-todo
RELEASE_PRODUCT := target/release/git-todo

DIST := $(abspath dist)
DIST_PRODUCT := $(DIST)/bin/git-todo
DIST_MAN_PAGE := $(DIST)/share/man/man1/git-todo.1

# Set STATIC=1 to build a static binary.
STATIC ?= 0

ifeq ($(STATIC), 1)
BUILD_VARS += PKG_CONFIG_LIBDIR=''
endif


.PHONY: doc
doc: $(MAN_PAGE)

$(MAN_PAGE): doc/git-todo.1.txt
	sed 's/`/*/g' $< > $@.transformed
	a2x --no-xmllint --format manpage $@.transformed
	rm $@.transformed


.PHONY: test
test: $(DEBUG_PRODUCT)
	prove -v -I./t

$(DEBUG_PRODUCT): $(SOURCES) $(DEPENDENCIES)
	cargo build


$(RELEASE_PRODUCT): $(SOURCES) $(DEPENDENCIES)
	$(BUILD_VARS) cargo build --release


.PHONY: dist
dist: $(DIST_PRODUCT) $(DIST_MAN_PAGE)

$(DIST):
	mkdir -p $@

$(DIST)/bin: $(DIST)
	mkdir -p $@

$(DIST)/share/man/man1: $(DIST)
	mkdir -p $@

$(DIST_PRODUCT): $(DIST)/bin $(RELEASE_PRODUCT)
	cp $(RELEASE_PRODUCT) $<

$(DIST_MAN_PAGE): $(DIST)/share/man/man1 $(MAN_PAGE)
	cp $(MAN_PAGE) $<


.PHONY: pkg
pkg: git-todo_$(VERSION)_$(TOOLCHAIN).tar.bz2

git-todo_$(VERSION)_$(TOOLCHAIN).tar.bz2: dist
	tar cjv -s /dist/git-todo_$(VERSION)_$(TOOLCHAIN)/ -f $@ dist
