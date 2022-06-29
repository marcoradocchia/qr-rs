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

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};
use lazy_static::lazy_static;
use qrcodegen::{QrCode, QrCodeEcc};
use regex::Regex;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

/// Parse hex code colors.
fn parse_hex_color(hex: &str) -> Result<String, String> {
    lazy_static! {
        static ref HEX_RE: Regex = Regex::new("^#([0-9A-Fa-f]{3}){1,2}$").unwrap();
    }

    match HEX_RE.is_match(hex) {
        true => Ok(hex.to_string()),
        false => Err(format!("{hex} is not a valid hex color code")),
    }
}

/// Encode URLs or text into QR codes.
#[derive(Parser, Debug)]
#[clap(
    author = "Marco Radocchia <marco.radocchia@outlook.com>",
    version,
    about,
    long_about = None
)]
struct Args {
    /// Output file.
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,

    /// Background color.
    #[clap(short, long, default_value = "#000000", value_parser = parse_hex_color)]
    fg: String,

    /// Foreground color.
    #[clap(short, long, default_value = "#FFFFFF", value_parser = parse_hex_color)]
    bg: String,

    /// Text to encode.
    #[clap(value_parser)]
    text: String,
}

/// Print colored error message only on Stderr stream.
fn print_err(body: &str) -> io::Result<()> {
    let writer = BufferWriter::stderr(ColorChoice::Auto);
    let mut buffer = writer.buffer();

    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true))?;
    write!(&mut buffer, "error: ")?;
    buffer.reset()?;
    writeln!(&mut buffer, "{body}")?;

    writer.print(&buffer)
}

// Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform.
fn gen_svg(qr: QrCode, border: i32, bg: &str, fg: &str) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    result.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    result.push_str(
        "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n"
    );
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap();
    result.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n",
        dimension
    ));
    result.push_str(&format!(
        "\t<rect width=\"100%\" height=\"100%\" fill=\"{fg}\"/>\n"
    ));
    result.push_str("\t<path d=\"");
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result.push(' ');
                }
                result.push_str(&format!("M{},{}h1v1h-1z", x + border, y + border));
            }
        }
    }
    result.push_str(&format!("\" fill=\"{bg}\"/>\n</svg>\n"));
    result
}

/// Create SVG file containing the QR code.
fn svg(qr: QrCode, output: &Path, bg: &str, fg: &str) -> io::Result<()> {
    // Create output file.
    let mut file = match fs::File::create(output) {
        Ok(file) => file,
        Err(_) => {
            print_err("unable to create SVG file")?;
            process::exit(1);
        }
    };

    // Write SVG to output file.
    if file.write_all(&gen_svg(qr, 1, fg, bg).as_bytes()).is_err() {
        print_err("unable to write SVG file")?;
        process::exit(1);
    }

    Ok(())
}

/// Create PNG file containing the QR code.
fn png(qr: QrCode, output: &Path, bg: &str, fg: &str) -> io::Result<()> {
    unimplemented!();
}

/// Print QR code to the console.
fn console(qr: QrCode) {
    const BORDER: i32 = 1;
    let min = -BORDER;
    let max = qr.size() + BORDER;
    for y in min..max {
        for x in min..max {
            let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
}

fn main() -> io::Result<()> {
    // Parse CLI arguments.
    let args = Args::parse();

    // Generate QR code.
    let qr = match QrCode::encode_text(&args.text, QrCodeEcc::Medium) {
        Ok(qr) => qr,
        Err(_) => {
            print_err("unable to generate QR code")?;
            process::exit(1);
        }
    };

    // Determine output type based on file extension.
    // Use SVG as default when no extension is provided.
    match args.output {
        Some(output) => {
            // Check if output file exists and if so ask for overwrite.
            if output.is_file()
                && !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Overwrite '{}'?", output.to_str().unwrap()))
                    .interact()?
            {
                process::exit(0);
            }

            match output.extension().map(|ext| ext.to_str().unwrap()) {
                Some("svg") => svg(qr, &output, &args.bg, &args.fg)?,
                Some("png") => png(qr, &output, &args.bg, &args.fg)?,
                _ => {
                    print_err("invalid file extension")?;
                    process::exit(1);
                }
            }
        }
        // When no output file is specified, print QR code to stdout.
        None => console(qr),
    };

    Ok(())
}
