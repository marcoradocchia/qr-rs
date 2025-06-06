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

/// Convert HEX color code to RGB values
///
/// # Note
///
/// This function assumes `hex` parameter being a valid HEX color code.
pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let mut hex = hex.strip_prefix('#').unwrap().to_string();
    if hex.len() == 3 {
        let mut expanded = String::new();
        for c in hex.chars() {
            for _ in 0..2 {
                expanded.push(c);
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
