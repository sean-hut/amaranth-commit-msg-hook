# Use > instead of a tab as recipe prefixes.
.RECIPEPREFIX = >

####################
# Internal variables
####################

rust-files = src/main.rs src/lib.rs src/body_checks.rs src/entire_commit_checks.rs src/summary_checks.rs

###################
# Integration Tests
###################

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
