use crate::utils::parse_hex_color;
pub use clap::Parser;
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
    #[clap(short,
        long,
        requires = "output",
        default_value = "#FFF",
        value_parser = parse_hex_color
    )]
    pub bg: String,

    /// Scale factor (raster image output only) [default: 25].
    #[clap(short, long, requires = "output", value_parser)]
    pub scale: Option<u32>,

    /// String to encode.
    #[clap(value_parser)]
    pub string: String,
}
