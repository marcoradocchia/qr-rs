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

pub use clap::Parser;
use lazy_static::lazy_static;
use qrcodegen::QrCodeEcc;
use regex::Regex;
use std::path::PathBuf;

/// Parse hex code colors.
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
pub fn parse_error_correction_level(ecl: &str) -> Result<QrCodeEcc, String> {
    Ok(match ecl {
        "low" => QrCodeEcc::Low,
        "medium" => QrCodeEcc::Medium,
        "quartile" => QrCodeEcc::Quartile,
        "high" => QrCodeEcc::High,
        _ => return Err("invalid error correction level".to_string()),
    })
}

/// A CLI utility to encode URLs or text into QR codes in various formats and colors.
#[derive(Parser, Debug)]
#[clap(
    author = "Marco Radocchia <marco.radocchia@outlook.com>",
    version,
    about,
    long_about = None
)]
pub struct Args {
    /// Output file (supported file extensions: jpeg, jpg, png, svg); omit to print QR code to
    /// console.
    #[clap(short, long, value_parser)]
    pub output: Option<PathBuf>,

    /// Background color (hex code).
    #[clap(
        short,
        long,
        requires = "output",
        default_value = "#000",
        value_parser = parse_hex_color
    )]
    pub fg: String,

    /// Foreground color (hex code).
    #[clap(
        short,
        long,
        requires = "output",
        default_value = "#FFF",
        value_parser = parse_hex_color
    )]
    pub bg: String,

    /// Border size (expressed in unit blocks).
    #[clap(short = 'B', long, default_value_t = 1, value_parser)]
    pub border: u8,

    /// QR error orrection level.
    #[clap(
        long,
        default_value = "medium",
        possible_values = ["low", "medium", "quartile", "high"],
        value_parser = parse_error_correction_level
    )]
    pub error_correction_level: QrCodeEcc,

    /// Scale factor (raster image output only) [default: 25].
    #[clap(short, long, requires = "output", value_parser)]
    pub scale: Option<u8>,

    /// String to encode.
    #[clap(value_parser)]
    pub string: Option<String>,
}
