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

use std::fmt::{Display, Formatter};
use std::io::{IsTerminal, Write};
use std::{fmt, io};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub enum Warning {
    /// Occurs when scale argument is provided with non-raster image output format.
    UnexpectedScaleOpt,
}

pub enum Error {
    /// Occurs when unable to generate QR code.
    QrCodeErr(String),

    /// Occurs when user choses unsupported output file extension.
    InvalidOutputExt,

    /// Occurs when unable to generate SVG output file.
    SvgOutputErr(String),

    /// Occurs when unable to generate raster image output file.
    RasterOutputErr(String),
}

pub enum ErrorKind {
    Warning(Warning),
    Error(Error),
}

impl ErrorKind {
    /// Colorize warning|error output.
    pub fn colorize(&self) -> io::Result<()> {
        let color_choice = match io::stderr().is_terminal() {
            true => ColorChoice::Auto,
            false => ColorChoice::Never,
        };

        // Color based on ErrorKind variant:
        //  * Warning -> ("warning:", Yellow)
        //  * Error -> ("error:", Red)
        let (prefix, color) = match self {
            Self::Warning(_) => ("warning", Some(Color::Yellow)),
            Self::Error(_) => ("error", Some(Color::Red)),
        };

        let writer = BufferWriter::stderr(color_choice);
        let mut buffer = writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(color).set_bold(true))?;
        write!(&mut buffer, "{prefix}: ")?;
        buffer.reset()?;
        writeln!(&mut buffer, "{self}")?;

        writer.print(&buffer)
    }
}

impl Display for ErrorKind {
    /// Print colored error message, but ONLY on Stderr stream.
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning(warning) => match warning {
                Warning::UnexpectedScaleOpt => {
                    write!(f, "Ignoring `scale` CLI option with non-raster output format")
                },
            },
            Self::Error(error) => match error {
                Error::QrCodeErr(msg) => {
                    write!(f, "Failed to generate QR code: {msg}")
                },
                Error::InvalidOutputExt => {
                    write!(f, "Invalid output file extension, expected one of jpeg, jpg, png, svg")
                },
                Error::SvgOutputErr(msg) => {
                    write!(f, "Failed to write SVG output file: {msg}")
                },
                Error::RasterOutputErr(msg) => {
                    write!(f, "Failed to write raster image file: {msg}")
                },
            },
        }
    }
}
