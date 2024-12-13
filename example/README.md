# Kompari Xtask example

This is an example that demonstrates how to integrate `kompari` as xtask into a test.

* `demolib` is an example library that we are testing with snapshot tests
* `xtask-kompari` is xtask that integrates kompari into the example project
* `tests` is a directory with snapshots and image for the current tests

## Demo

Try to modify the test in `demolib/src/lib.rs` to make it fail,
for example try to modify a parameter of the tested function as it
is shown in the comment of the test.

Run tests via `cargo test`. The test fails because snapshot is different.

Run `cargo xtask-kompari report` to generate a report
with differences. It will create a `report.html`.
