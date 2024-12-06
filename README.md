<p align="center">
<img src='docs/logo.png' width='128'>
</p>

# Kompari

*Kompari* is a tool for reporting image differences. It is intended for use in snapshot testing.
It can be used as a stand-alone CLI tool or as a Rust crate.

<img src='docs/screenshot.png' width='100%'>


## CLI

### Installation

Basic installation, `PNG` format only

```commandline
$ cargo install kompari --features=cli
```

OR with all supported formats

```commandline
$ cargo install kompari --features=cli,all-formats
```

### Usage

```commandline
$ kompari <left/image_dir> <right/image_dir> report
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

