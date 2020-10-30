# Use > instead of a tab as recipe prefixes.
.RECIPEPREFIX = >

####################
# Internal variables
####################

main = src/main.rs
lib = src/lib.rs
checks = src/checks/body.rs src/checks/entire.rs src/checks/summary.rs src/checks/footer.rs
rust-files = $(main) $(lib) $(checks)

###################
# Integration Tests
###################

all: rust-format rust-clippy-lints build test git-diff-check

rust-format: $(rust-files)
> cargo fmt -- --check --files-with-diff

rust-clippy-lints: $(rust-files)
> cargo clippy -- --deny clippy::all

build: $(rust-files)
> cargo build

test: $(rust-files)
> cargo test

.PHONY: git-diff-check
git-diff-check:
> git diff --check

.PHONY: clean
clean:
> cargo clean
