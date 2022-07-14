<div align="center">
  <h1 align="center">qr-rs</h1>

  ![GitHub releases](https://img.shields.io/github/downloads/marcoradocchia/qr-rs/total?color=%23a9b665&logo=github)
  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/qr-rs?color=ea6962&logo=github)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/qr-rs?color=%23d8a657&logo=github)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/qr-rs?color=%2389b482&logo=github)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github)
  <!-- ![Crates.io downloads](https://img.shields.io/crates/d/qr-rs?label=crates.io%20downloads&logo=rust) -->
  <!-- ![Crates.io version](https://img.shields.io/crates/v/qr-rs?logo=rust&color=%23d8a657) -->
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/qr-rs?color=%23e78a4e)
</div>

Command Line Interface utility to encode URLs or more generally text into QR
codes in various file formats and colors.

## Index

- [Install](#install)
  * [Master branch](#master-branch)
  * [Latest release from crates.io](#latest-release-from-crates.io)
- [Uninstall](#uninstall)
- [Usage](#usage)
- [Changelog](#changelog)
- [License](#license)

## Install

The following installation instructions assume a **Rust toolchain** (`rustc >=1.62.0`) installed
on the system. In order to install such toolchain you can use `rusutp`: see
https://www.rust-lang.org/tools/install for further installation
instructions and notes.

### Master branch

To build and install from master branch run:
```sh
cargo install --git https://github.com/marcoradocchia/qr-rs --branch master
```

### Latest release from crates.io

To build and install the latest release from
[crates.io](https://crates.io/crates/qr-rs) run:
```
cargo install qr-rs
```

## Uninstall

To uninstall run:
```
cargo uninstall qr-rs
```

## Usage
```
qr-rs 0.1.0
Marco Radocchia <marco.radocchia@outlook.com>
Encode URLs or text into QR codes.

USAGE:
    qr [OPTIONS] <STRING>

ARGS:
    <STRING>    String to encode

OPTIONS:
    -b, --bg <BG>            Foreground color (hex code) [default: #FFF]
    -f, --fg <FG>            Background color (hex code) [default: #000]
    -h, --help               Print help information
    -o, --output <OUTPUT>    Output file (supported file extensions: jpeg, jpg, png, svg); omit to
                             print QR code to console
    -s, --scale <SCALE>      Scale factor (raster image output only) [default: 25]
    -V, --version            Print version information
```

## Changelog

Complete [CHANGELOG](CHANGELOG.md).

## License

[GPLv3](LICENSE)
