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
    /// Output file.
    #[clap(short, long, value_parser)]
    pub output: Option<PathBuf>,

    /// Background color.
    #[clap(
        short,
        long,
        requires = "output",
        default_value = "#000",
        value_parser = parse_hex_color
    )]
    pub fg: String,

    /// Foreground color.
    #[clap(short,
        long,
        requires = "output",
        default_value = "#FFF",
        value_parser = parse_hex_color
    )]
    pub bg: String,

    /// Scale factor of raster output image format.
    #[clap(short, long, requires = "output", value_parser)]
    pub scale: Option<u32>,

    /// Text to encode.
    #[clap(value_parser)]
    pub text: String,
}
