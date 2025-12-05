use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb(u8, u8, u8);

#[derive(Debug)]
enum RgbParseErr {
    IncorrectLength,
    //InvalidHexString,
    NoLeadingHash,
    RedChannelOutOfBounds,
    GreenChannelOutOfBounds,
    BlueChannelOutOfBounds,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    // TODO: implement trait
    fn b(&self) -> u8 {
        self.2
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn r(&self) -> u8 {
        self.0
    }
}

impl FromStr for Rgb {
    type Err = RgbParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 7 {
            return Err(RgbParseErr::IncorrectLength);
        };
        if let Some(hex) = s.strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16)
                .map_err(|_| RgbParseErr::RedChannelOutOfBounds)?;
            let g = u8::from_str_radix(&hex[2..4], 16)
                .map_err(|_| RgbParseErr::GreenChannelOutOfBounds)?;
            let b = u8::from_str_radix(&hex[4..6], 16)
                .map_err(|_| RgbParseErr::BlueChannelOutOfBounds)?;
            Ok(Rgb(r, g, b))
        } else {
            Err(RgbParseErr::NoLeadingHash)
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
