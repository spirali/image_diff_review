# Kompari Xtask example

This is an example that demonstrates how to integrate `kompari` as xtask into a test.

* `demolib` is an example library that we are testing with snapshot tests
* `xtask-kompari` is xtask that integrates kompari into the example project
* `tests` is a directory with snapshots and image for the current tests


## Demo

Try to modify the test in `demolib`:

E.g. changing the test in the following way:

```rust
    #[test]
    fn test_create_rectangle() {
        //      Change the value ----\
        //      here to e.g. 25      |
        //                           v
        let image = create_rectangle(10, 5, 50, 70, Rgb([255, 0, 0]));
        check_snapshot(image, "create_rectangle.png");
    }
```

Run tests via `cargo test`. The test fails because snapshot is different.

Run `cargo xtask-kompari report` to generate a report
with differences. It will create a `report.html`.
