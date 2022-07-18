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

mod args;
mod error;
mod utils;

use args::{Args, Parser};
use dialoguer::{theme::ColorfulTheme, Confirm};
use error::{Error, ErrorKind, Warning};
use image::{ImageBuffer, RgbImage};
use qrcodegen::QrCode;
use std::{fmt::Write as _, fs, io::Write, path::Path, process};
use utils::hex_to_rgb;

// A Qr code TODO
struct Qr {
    data: QrCode,
    border: i32,
}

impl Qr {
    // Construct `Qr` instance.
    fn new(data: QrCode, border: u8) -> Self {
        Self {
            data,
            border: border.into()
        }
    }
}

/// Trait defining methods to output QR code to various formats.
trait QrOutput {
    /// Create SVG file containing the QR code.
    fn svg(&self, output: &Path, bg: &str, fg: &str) -> Result<(), ErrorKind>;

    /// Create raster image (png|jpg|jpeg) file containing the QR code.
    fn rst(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind>;

    /// Print QR code to the console.
    fn console(&self);
}

impl QrOutput for Qr {
    /// Create SVG file containing the QR code.
    fn svg(&self, output: &Path, bg: &str, fg: &str) -> Result<(), ErrorKind> {
        // Create output file.
        let mut file = match fs::File::create(output) {
            Ok(file) => file,
            Err(err) => return Err(ErrorKind::Error(Error::SvgOutputErr(err.to_string()))),
        };

        // Generate a string of SVG code for an image depicting the given QR Code.
        // The string always uses Unix newlines (\n), regardless of the platform.
        let mut svg_str = String::new();

        let size = self.data.size();
        let dimension = size.checked_add(self.border * 2).unwrap();

        writeln!(
            svg_str,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
                <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \
                \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\
                <svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" \
                viewBox=\"0 0 {dimension} {dimension}\" stroke=\"none\">\
                \t<rect width=\"100%\" height=\"100%\" fill=\"{bg}\"/>\t<path d=\""
        )
        .unwrap();

        // Write actual QR code information.
        for y in 0..size {
            for x in 0..size {
                if self.data.get_module(x, y) {
                    if x != 0 || y != 0 {
                        write!(svg_str, " ").unwrap();
                    }
                    write!(svg_str, "M{},{}h1v1h-1z", x + self.border, y + self.border).unwrap();
                }
            }
        }
        writeln!(svg_str, "\" fill=\"{fg}\"/>\n</svg>").unwrap();

        // Write SVG to output file.
        if let Err(err) = file.write_all(svg_str.as_bytes()) {
            return Err(ErrorKind::Error(Error::SvgOutputErr(err.to_string())));
        }

        Ok(())
    }

    /// Create raster image (png|jpg|jpeg) file containing the QR code.
    fn rst(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind> {
        // Convert colors to RGB values.
        let fg = hex_to_rgb(fg);
        let bg = hex_to_rgb(bg);

        let scaled_border = scale * self.border;
        // Size of the image including the borders.
        let img_size = self.data.size() * scale + (2 * scaled_border);

        // Create square image: image needs border on each side of the square.
        let mut img: RgbImage = ImageBuffer::new(img_size as u32, img_size as u32);

        // Write image pixels.
        for y in 0..img_size {
            for x in 0..img_size {
                let mut pixel = img.get_pixel_mut(x as u32, y as u32);

                if x <= scaled_border || y <= scaled_border {
                    pixel.0 = bg;
                    continue;
                }

                let x = (x - scaled_border) / scale;
                let y = (y - scaled_border) / scale;
                pixel.0 = if self.data.get_module(x, y) { fg } else { bg };
            }
        }

        // Save image.
        if let Err(err) = img.save(output) {
            return Err(ErrorKind::Error(Error::RasterOutputErr(err.to_string())));
        }

        Ok(())
    }

    fn console(&self) {
        for y in -self.border..self.data.size() + self.border {
            for x in -self.border..self.data.size() + self.border {
                print!("{0}{0}", if self.data.get_module(x, y) { '█' } else { ' ' });
            }
            println!();
        }
    }
}

/// Runs the program & catches errors.
fn run(args: &Args) -> Result<(), ErrorKind> {
    // Generate QR code.
    let qr = match QrCode::encode_text(&args.string, args.error_correction_level) {
        Ok(data) => Qr::new(data, args.border),
        Err(err) => return Err(ErrorKind::Error(Error::QrCodeErr(err.to_string()))),
    };

    // Print to stdout if args.output is None, otherwhise print to file.
    match &args.output {
        Some(output) => {
            // Check if output file exists and if so ask for overwrite.
            if output.is_file()
                && !Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!("Overwrite {:?}?", output))
                    .interact()
                    .expect("dialog interaction failed")
            {
                return Ok(());
            }

            // Determine output file type based on file extension.
            match output.extension().map(|ext| ext.to_str().unwrap()) {
                Some("svg") => {
                    qr.svg(output, &args.bg, &args.fg)?;
                    // Warn the user if scale CLI option was provided with non-raster image output
                    // format.
                    if args.scale.is_some() {
                        return Err(ErrorKind::Warning(Warning::UnexpectedScaleOpt));
                    }
                }
                Some("png" | "jpg" | "jpeg") => {
                    qr.rst(output, args.scale.unwrap_or(25).into(), &args.bg, &args.fg)?
                }
                _ => return Err(ErrorKind::Error(Error::InvalidOutputExt)),
            }
        }
        // When no output file is specified, print QR code to stdout.
        None => qr.console(),
    };

    Ok(())
}

/// Main function: calls run function and prints errors.
fn main() {
    if let Err(e) = run(&Args::parse()) {
        e.colorize().unwrap();
        process::exit(1);
    }
}
