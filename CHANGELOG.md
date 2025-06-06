# Changelog

## [0.3.0] - 2025-06-05

### Added

- Add `-F`/`--force` command line flag to force output, i.e. overwrite without
  user confirmation (e.g. assume `yes`).

### Changed

- Bump MSRV from `1.62.0` to `1.70.0`.
- Bump `clap` version from `3.2.12` to `4.5.39`.
- Bump `clap_mangen` version from `0.1.10` to `0.2.26`.
- Bump `clap_complete` version from `3.2.3` to `4.5.52`.
- Bump `directories` version from `4.0.1` to `6.0.0`.
- Bump `termcolor` version from `1.1.3` to `1.4.1`.
- Bump `dialoguer` version from `0.10.1` to `0.11.0`.
- Bump `regex` version from `1.6.0` to `1.11.1`.
- Bump `lazy_static` version from `1.4.0` to `1.5.0`.
- Bump `image` version from `0.24.2` to `0.25.6`.

### Removed

- Remove `atty` dependency.

## [0.2.0] - 2022-07-18

### Added

- Feature to read string to encode from standard input, which allows to pipe
  commands to `qr-rs` (closing issue #1).
- `border` CLI option to specify border size.
- `error-correction-level` CLI option to specify QR *error-correction-level* as
  one of the following values:
  - **low**;
  - **medium**;
  - **quartile**;
  - **high**.

## [0.1.0] - 2022-07-15

Initial release.
