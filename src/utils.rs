use lazy_static::lazy_static;
use qrcodegen::QrCodeEcc;
use regex::Regex;

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

/// This conversion assumes the HEX string as valid color and returns corresponding RGB value.
pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let mut hex = hex.strip_prefix('#').unwrap().to_string();
    if hex.len() == 3 {
        let mut expanded = String::new();
        for c in hex.chars() {
            for _ in 0..2 {
                expanded.push(c)
            }
        }
        hex = expanded;
    }

    let mut rgb: [u8; 3] = [0, 0, 0];
    for (i, rgb_val) in rgb.iter_mut().enumerate() {
        let (f, s) = hex[2 * i..2 * (i + 1)].split_at(1);
        let f = u8::from_str_radix(f, 16).unwrap();
        let s = u8::from_str_radix(s, 16).unwrap();
        *rgb_val = f * 16 + s;
    }

    rgb
}
