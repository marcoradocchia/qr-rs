use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub enum ErrorKind {
    /// Occurs when unable to generate QR code.
    QrCodeErr(String),

    /// Occurs when user choses unsupported output file extension.
    InvalidOutputExt,

    /// Occurs when unable to generate SVG output file.
    SvgOutputErr(String),

    /// Occurs when unable to generate raster image output file.
    RasterOutputErr,
}

impl ErrorKind {
    /// Colorize error output.
    pub fn colorize(&self) -> io::Result<()> {
        let writer = BufferWriter::stderr(ColorChoice::Auto);
        let mut buffer = writer.buffer();

        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        write!(&mut buffer, "error: ")?;
        buffer.reset()?;
        writeln!(&mut buffer, "{}", self)?;

        writer.print(&buffer)
    }
}

impl Display for ErrorKind {
    /// Print colored error message, but ONLY on Stderr stream.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::QrCodeErr(msg) => write!(f, "unable to generate QR code `{msg}`"),
            Self::InvalidOutputExt => write!(f, "invalid output file extension"),
            Self::SvgOutputErr(msg) => write!(f, "unable to write SVG output file `{msg}`"),
            Self::RasterOutputErr => write!(f, "unable to write raster image file"),
        }
    }
}
