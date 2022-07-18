use crate::utils::{parse_error_correction_level, parse_hex_color};
pub use clap::Parser;
use qrcodegen::QrCodeEcc;
use std::path::PathBuf;

/// Encode URLs or text into QR codes.
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
    pub string: String,
}
