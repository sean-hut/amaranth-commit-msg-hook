# Use > instead of a tab as recipe prefixes.
.RECIPEPREFIX = >

###################
# Integration Tests
###################

all: rust git-diff-check

rust: format clippy build test

.PHONY: format
format:
> cargo fmt -- --check --files-with-diff

.PHONY: clippy
clippy:
> cargo clippy -- --deny clippy::all

.PHONY: build
build:
> cargo build

.PHONY: test
test:
> cargo test

.PHONY: git-diff-check
git-diff-check:
> git diff --check

.PHONY: clean
clean:
> cargo clean
