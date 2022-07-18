use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};
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
        let color_choice = match atty::is(atty::Stream::Stderr) {
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
        write!(&mut buffer, "{}: ", prefix)?;
        buffer.reset()?;
        writeln!(&mut buffer, "{}", self)?;

        writer.print(&buffer)
    }
}

impl Display for ErrorKind {
    /// Print colored error message, but ONLY on Stderr stream.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Warning(warning) => match warning {
                Warning::UnexpectedScaleOpt => write!(
                    f,
                    "ignoring `scale` CLI option with non-raster output format"
                ),
            },
            Self::Error(error) => match error {
                Error::QrCodeErr(msg) => write!(f, "unable to generate QR code: {msg}"),
                Error::InvalidOutputExt => write!(f, "invalid output file extension"),
                Error::SvgOutputErr(msg) => write!(f, "unable to write SVG output file: {msg}"),
                Error::RasterOutputErr(msg) => {
                    write!(f, "unable to write raster image file: {msg}")
                }
            },
        }
    }
}
