# Changelog

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
