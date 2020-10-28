# Use > instead of a tab as recipe prefixes.
.RECIPEPREFIX = >

####################
# Internal variables
####################

rust-files = src/main.rs src/lib.rs src/body_checks.rs src/entire_commit_checks.rs src/summary_checks.rs

build: $(rust-files)
> cargo build
