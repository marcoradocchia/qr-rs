// qr-rs: Encode URLs or text into QR codes.
// Copyright (C) 2022 Marco Radocchia
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see https://www.gnu.org/licenses/.

use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;
use lazy_static::lazy_static;
use qrcodegen::QrCodeEcc;
use regex::Regex;
use std::path::PathBuf;

const HELP_TEMPLATE: &'static str = "{name} v{version} by {author}

{about}
{usage-heading} {usage}

{all-args}
";

const ECC_VALUES: &'static [&'static str] = &["low", "medium", "quartile", "high"];

/// Parse hex code colors.
///
/// # Errors
///
/// Returns an error if an invalid hex color code was provided.
pub fn parse_hex_color(hex: &str) -> Result<String, String> {
    lazy_static! {
        static ref HEX_RE: Regex = Regex::new("^#([0-9A-Fa-f]{3}){1,2}$").unwrap();
    }

    match HEX_RE.is_match(hex) {
        true => Ok(hex.to_string()),
        false => Err(format!("{hex} is not a valid hex color code")),
    }
}

/// Parse QR error correction level (assumes ecl being one of ["low", "medium", "quartile", "high"]).
pub fn parse_error_correction_level(ecl: String) -> QrCodeEcc {
    match ecl.as_str() {
        "low" => QrCodeEcc::Low,
        "medium" => QrCodeEcc::Medium,
        "quartile" => QrCodeEcc::Quartile,
        "high" => QrCodeEcc::High,
        _ => unreachable!(),
    }
}

/// A CLI utility to encode URLs or text into QR codes in various formats and colors.
#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE,
)]
pub struct Args {
    /// Output file (supported file extensions: jpeg, jpg, png, svg); omit to print QR code to
    /// console.
    #[arg(short, long, value_parser)]
    pub output: Option<PathBuf>,

    /// Force output, i.e. overwrite without user confirmation.
    #[arg(short = 'F', long, requires = "output")]
    pub force: bool,

    /// Background color (hex code).
    #[arg(
        short,
        long,
        requires = "output",
        default_value = "#000",
        value_parser = parse_hex_color
    )]
    pub fg: String,

    /// Foreground color (hex code).
    #[arg(
        short,
        long,
        requires = "output",
        default_value = "#FFF",
        value_parser = parse_hex_color
    )]
    pub bg: String,

    /// Border size (expressed in unit blocks).
    #[arg(short = 'B', long, default_value_t = 1, value_parser)]
    pub border: u8,

    /// QR error orrection level.
    #[arg(
        short = 'l',
        long,
        default_value = "medium",
        value_parser = PossibleValuesParser::new(ECC_VALUES).map(parse_error_correction_level),
    )]
    pub error_correction_level: QrCodeEcc,

    /// Scale factor (raster image output only) [default: 25].
    #[arg(short, long, requires = "output", value_parser)]
    pub scale: Option<u8>,

    /// String to encode.
    #[arg(value_parser)]
    pub string: Option<String>,
}
