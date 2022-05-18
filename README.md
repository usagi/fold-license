[![github]](https://github.com/usagi/fold-license)&ensp;[![crates-io]](https://crates.io/crates/fold-license)&ensp;[![docs-rs]](https://docs.rs/fold-license)<br>
[![Build Status](https://travis-ci.org/usagi/fold-license.svg?branch=master)](https://travis-ci.org/usagi/fold-license)

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K

# fold-license

Folding licenses from multiple dirs with `cargo`/Cargo.toml and `yarn`/package.json.

- [x] multiple repos dirs support with glob patterns.
  - eg.) `fold-license -i path/to/repos1 -i path/to/repos2 -i path/to/repos3`
  - eg.) `fold-license -i path1/**/*/ -i path2/foo/bar`
- Repos type:
  - [x] `cargo`/Cargo.toml repos of a Rust project.
  - [x] `yarn`/packages.json repos of a Node.js project.
- Output to:
  - [x] STDOUT
  - [x] File
- Output with:
  - [x] TOML format, non-pretty or pretty.
  - [x] JSON format, non-pretty or pretty.
  - [x] MSGPACK format, binary.
- [x] unification from a multiple licenses from a multiple source dirs.
- Run-time configuration from:
  - [x] Command line arguments.
  - [x] Prepared configuration file.

### Install

```sh
cargo install fold-licenses
```

## Usage

note: This package include both of bin and lib. In this section, I explain only the bin version. See also lib.rs and docs if you want to use lib version.

`fold-license --help`:

```sh
USAGE:
    fold-license [OPTIONS]

OPTIONS:
    -c, --conf <CONF>        Use configuration file if set the path. -c path/to/conf.toml
        --cargo              `cargo`/Cargo.toml, enabled
    -f, --format <FORMAT>    toml, json, msgpack. -e json
    -h, --help               Print help information
    -i, --in <IN>            input glob pattern(s). -i aaa -i bbb -i ccc ...
    -o, --out <OUT>          output to the path if set. else, output to stdout
    -p, --pretty             pretty output. **ONLY TO USE WITH A TEXT FORMAT**
    -s, --silent             silet a log messages
    -V, --version            Print version information
        --yarn               `yarn`/packages.json, enabled
```

1. eg.) `fold-license -i . -i path/to/somewhere -i 'target/**/pattern/*' -o target/output.toml -s -p --cargo --yarn`
2. eg.) `fold-license -c example/example-conf.toml` if you want to use `-c` and prepared configuration file:

```toml
# example-conf.toml; `fold-license -c example/example-conf.toml`
format = "toml"
in = [".", "path/to/somewhere", "target/**/pattern/*"]
out = "target/output.toml"
silent = true
pretty = true
cargo = true
yarn = true
```

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
