This directory is for integration testing.

Integration testing only looks at public APIs of a library.

It aims to see if multiple modules in a library work correctly together 
because it is possible that unit testing of individual modules might pass
but multiple modules working together might not be give correct implementations.

Each file will be compiled as a separate crate.

However, `.rs` files in subdirectories of this `tests` directory are not treated as test files.

So that means, if there's some setup code that is commonly used in some tests,
they need to be put inside a subdirectory.

Thus, the old-style rust module files `common/mod.rs` is used
instead of `common.rs` as a workaround.

Integration tests for binary crates is not possible
because it's not possible to import them with `use`.
Only library crates can be imported with `use`.

That's why binary crates should be designed as minimally as possible,
and call APIs in an accompanying library crate instead.
And then integration tests should be performed on the library crate.
