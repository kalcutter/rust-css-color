#![allow(clippy::get_first)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::len_zero)]
#![cfg_attr(feature = "bench", feature(test))]

use std::f32;
use std::str::{self, FromStr};

const NONE: f32 = 0_f32;

#[doc(hidden)]
pub type Rgba = Srgb;

/// A color in the sRGB color space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Srgb {
    /// The red component.
    pub red: f32,
    /// The green component.
    pub green: f32,
    /// The blue component.
    pub blue: f32,
    /// The alpha component.
    pub alpha: f32,
}

impl Srgb {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Srgb {
        Srgb {
            red,
            green,
            blue,
            alpha,
        }
    }

    fn from_rgb8(red: u8, green: u8, blue: u8) -> Srgb {
        Srgb::from_rgba8(red, green, blue, 255)
    }

    fn from_rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Srgb {
        Srgb {
            red: red as f32 / 255.,
            green: green as f32 / 255.,
            blue: blue as f32 / 255.,
            alpha: alpha as f32 / 255.,
        }
    }
}

#[derive(Debug)]
pub struct ParseColorError;

impl FromStr for Srgb {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_css_color(s.as_bytes()).map_err(|_| ParseColorError)
    }
}

// https://www.w3.org/TR/css-color-4/
fn parse_css_color(input: &[u8]) -> Result<Srgb, ()> {
    if let Ok(input) = consume_byte(input, b'#') {
        parse_hex(input)
    } else if let Ok(input) = consume_function(input, b"rgb") {
        parse_rgb(input)
    } else if let Ok(input) = consume_function(input, b"rgba") {
        parse_rgb(input)
    } else if let Ok(input) = consume_function(input, b"hsl") {
        parse_hsl(input)
    } else if let Ok(input) = consume_function(input, b"hsla") {
        parse_hsl(input)
    } else if let Ok(input) = consume_function(input, b"hwb") {
        parse_hwb(input)
    } else {
        parse_named(input)
    }
}

fn clamp_unit_f32(value: f32) -> f32 {
    value.clamp(0., 1.)
}

fn normalize_hue(value: f32) -> f32 {
    value - value.floor()
}

struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
}

// https://www.w3.org/TR/css-color-4/#hsl-to-rgb
impl From<Hsla> for Srgb {
    fn from(hsla: Hsla) -> Self {
        let t2 = if hsla.lightness <= 0.5 {
            hsla.lightness * (hsla.saturation + 1.)
        } else {
            hsla.lightness + hsla.saturation - hsla.lightness * hsla.saturation
        };
        let t1 = hsla.lightness * 2. - t2;

        let hue_to_rgb = |h6: f32| -> f32 {
            if h6 < 1. {
                (t2 - t1) * h6 + t1
            } else if h6 < 3. {
                t2
            } else if h6 < 4. {
                (t2 - t1) * (4. - h6) + t1
            } else {
                t1
            }
        };
        let h6 = hsla.hue * 6.;
        let h6_red = if h6 + 2. < 6. { h6 + 2. } else { h6 - 4. };
        let h6_blue = if h6 - 2. >= 0. { h6 - 2. } else { h6 + 4. };
        Srgb {
            red: hue_to_rgb(h6_red),
            green: hue_to_rgb(h6),
            blue: hue_to_rgb(h6_blue),
            alpha: hsla.alpha,
        }
    }
}

struct Hwba {
    pub hue: f32,
    pub whiteness: f32,
    pub blackness: f32,
    pub alpha: f32,
}

// https://www.w3.org/TR/css-color-4/#hwb-to-rgb
impl From<Hwba> for Srgb {
    fn from(hwba: Hwba) -> Self {
        // If the sum of these two arguments is greater than 100%, then at computed-value time they
        // are further normalized to add up to 100%, with the same relative ratio.
        if hwba.whiteness + hwba.blackness >= 1. {
            let gray = hwba.whiteness / (hwba.whiteness + hwba.blackness);
            Srgb {
                red: gray,
                green: gray,
                blue: gray,
                alpha: hwba.alpha,
            }
        } else {
            fn hue_to_rgb(h6: f32) -> f32 {
                if h6 < 1. {
                    h6
                } else if h6 < 3. {
                    1.
                } else if h6 < 4. {
                    4. - h6
                } else {
                    0.
                }
            }
            let h6 = hwba.hue * 6.;
            let h6_red = if h6 + 2. < 6. { h6 + 2. } else { h6 - 4. };
            let h6_blue = if h6 - 2. >= 0. { h6 - 2. } else { h6 + 4. };
            let x = 1. - hwba.whiteness - hwba.blackness;
            Srgb {
                red: hue_to_rgb(h6_red) * x + hwba.whiteness,
                green: hue_to_rgb(h6) * x + hwba.whiteness,
                blue: hue_to_rgb(h6_blue) * x + hwba.whiteness,
                alpha: hwba.alpha,
            }
        }
    }
}

fn is_ident_start(input: &[u8]) -> bool {
    match input.get(0) {
        Some(b'-') => match input.get(1) {
            Some(b'-') => true,
            Some(c) => is_name_start(*c),
            _ => false,
        },
        Some(c) => is_name_start(*c),
        _ => false,
    }
}

fn is_name_start(c: u8) -> bool {
    match c {
        b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
        c => !c.is_ascii(),
    }
}

fn is_name(c: u8) -> bool {
    match c {
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' => true,
        c => !c.is_ascii(),
    }
}

fn is_whitespace(c: u8) -> bool {
    c <= b' ' && (c == b' ' || c == b'\n' || c == b'\t' || c == b'\r' || c == b'\x0C')
}

fn digit(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(()),
    }
}

fn hex_digit(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(()),
    }
}

fn skip_ws(mut input: &[u8]) -> &[u8] {
    while input.len() > 0 && is_whitespace(input[0]) {
        input = &input[1..];
    }
    input
}

fn consume_byte(input: &[u8], b: u8) -> Result<&[u8], ()> {
    match input.get(0) {
        Some(c) if *c == b => Ok(&input[1..]),
        _ => Err(()),
    }
}

/// Consumes a function-token matching the given identifier.
///
/// Any whitespace following the function-token is also consumed.
fn consume_function<'a>(input: &'a [u8], name: &[u8]) -> Result<&'a [u8], ()> {
    debug_assert!(is_ident_start(name));

    let n = name.len();
    if input.len() >= n + 1 && input[..n].eq_ignore_ascii_case(name) && input[n] == b'(' {
        Ok(skip_ws(&input[n + 1..]))
    } else {
        Err(())
    }
}

#[inline]
fn consume_name<'a>(input: &'a [u8], name: &[u8]) -> Result<&'a [u8], ()> {
    debug_assert!(is_ident_start(name));

    let n = name.len();
    if input.len() >= n
        && input[..n].eq_ignore_ascii_case(name)
        && input.get(n).filter(|c| is_name(**c)).is_none()
    {
        Ok(&input[n..])
    } else {
        Err(())
    }
}

fn consume_none(input: &[u8]) -> Result<&[u8], ()> {
    consume_name(input, b"none")
}

fn consume_number(mut input: &[u8]) -> Result<&[u8], ()> {
    fn skip_sign(input: &[u8]) -> &[u8] {
        match input.get(0) {
            Some(b'+') | Some(b'-') => &input[1..],
            _ => input,
        }
    }
    fn consume_digits(mut input: &[u8]) -> Result<&[u8], ()> {
        match input.get(0).map(|c| digit(*c)) {
            Some(Ok(_)) => {
                while let Some(Ok(_)) = input.get(0).map(|c| digit(*c)) {
                    input = &input[1..];
                }
                Ok(input)
            }
            _ => Err(()),
        }
    }

    input = skip_sign(input);
    match input.get(0) {
        Some(b'.') => {}
        _ => {
            input = consume_digits(input)?;
        }
    }
    if let Some(b'.') = input.get(0) {
        input = consume_digits(&input[1..])?;
    }
    match input.get(0) {
        Some(b'E') | Some(b'e') => {
            input = skip_sign(&input[1..]);
            input = consume_digits(input)?;
        }
        _ => {}
    }
    Ok(input)
}

fn parse_number(input: &[u8]) -> Result<(&[u8], f32), ()> {
    let pos = input.len() - consume_number(input)?.len();
    Ok((
        &input[pos..],
        str::from_utf8(&input[..pos])
            .unwrap()
            .parse()
            .map_err(|_| ())?,
    ))
}

// <percentage> = <number> %
fn parse_percentage(input: &[u8]) -> Result<(&[u8], f32), ()> {
    let (input, value) = parse_number(input)?;
    let input = consume_byte(input, b'%')?;

    Ok((input, value / 100.))
}

fn parse_number_percentage(input: &[u8]) -> Result<(&[u8], f32), ()> {
    let (input, value) = parse_number(input)?;

    if let Ok(input) = consume_byte(input, b'%') {
        Ok((input, value / 100.))
    } else {
        Ok((input, value / 100.))
    }
}

enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}
use self::NumberOrPercentage::*;

fn parse_number_or_percentage(input: &[u8]) -> Result<(&[u8], NumberOrPercentage), ()> {
    let (input, value) = parse_number(input)?;

    if let Ok(input) = consume_byte(input, b'%') {
        Ok((input, Percentage(value / 100.)))
    } else {
        Ok((input, Number(value)))
    }
}

// <alpha-value> = <number> | <percentage>
fn parse_alpha_value(input: &[u8]) -> Result<(&[u8], f32), ()> {
    let (input, alpha) = parse_number_or_percentage(input)?;
    let alpha = match alpha {
        Number(value) => value,
        Percentage(value) => value,
    };
    let alpha = clamp_unit_f32(alpha);

    Ok((input, alpha))
}

// <hue> = <number> | <angle>
fn parse_hue(input: &[u8]) -> Result<(&[u8], f32), ()> {
    let (input, value) = parse_number(input)?;

    if !is_ident_start(input) {
        Ok((input, value / 360.))
    } else if let Ok(input) = consume_name(input, b"deg") {
        Ok((input, value / 360.))
    } else if let Ok(input) = consume_name(input, b"grad") {
        Ok((input, value / 400.))
    } else if let Ok(input) = consume_name(input, b"rad") {
        Ok((input, value / (2. * f32::consts::PI)))
    } else if let Ok(input) = consume_name(input, b"turn") {
        Ok((input, value))
    } else {
        Err(())
    }
}

/// Parse sRGB hex colors.
fn parse_hex(input: &[u8]) -> Result<Srgb, ()> {
    match input.len() {
        8 => Ok(Srgb::from_rgba8(
            hex_digit(input[0])? * 16 + hex_digit(input[1])?,
            hex_digit(input[2])? * 16 + hex_digit(input[3])?,
            hex_digit(input[4])? * 16 + hex_digit(input[5])?,
            hex_digit(input[6])? * 16 + hex_digit(input[7])?,
        )),
        6 => Ok(Srgb::from_rgb8(
            hex_digit(input[0])? * 16 + hex_digit(input[1])?,
            hex_digit(input[2])? * 16 + hex_digit(input[3])?,
            hex_digit(input[4])? * 16 + hex_digit(input[5])?,
        )),
        4 => Ok(Srgb::from_rgba8(
            hex_digit(input[0])? * 17,
            hex_digit(input[1])? * 17,
            hex_digit(input[2])? * 17,
            hex_digit(input[3])? * 17,
        )),
        3 => Ok(Srgb::from_rgb8(
            hex_digit(input[0])? * 17,
            hex_digit(input[1])? * 17,
            hex_digit(input[2])? * 17,
        )),
        _ => Err(()),
    }
}

// hsl()  = [ <legacy-hsl-syntax>  | <modern-hsl-syntax>  ]
// hsla() = [ <legacy-hsla-syntax> | <modern-hsla-syntax> ]
// <legacy-hsl-syntax>  = hsl(  <hue>, <percentage>, <percentage>, <alpha-value>? )
// <legacy-hsla-syntax> = hsla( <hue>, <percentage>, <percentage>, <alpha-value>? )
// <modern-hsl-syntax>  = hsl(  [<hue> | none]
//                              [<percentage> | <number> | none]
//                              [<percentage> | <number> | none]
//                              [ / [<alpha-value> | none] ]? )
// <modern-hsla-syntax> = hsla( [<hue> | none]
//                              [<percentage> | <number> | none]
//                              [<percentage> | <number> | none]
//                              [ / [<alpha-value> | none] ]? )
fn parse_hsl(input: &[u8]) -> Result<Srgb, ()> {
    let (input, hue, legacy_syntax) = if let Ok((input, hue)) = parse_hue(input) {
        let input = skip_ws(input);
        match input.get(0) {
            Some(b',') => (skip_ws(&input[1..]), hue, true),
            _ => (input, hue, false),
        }
    } else {
        (skip_ws(consume_none(input)?), NONE, false)
    };

    let (input, saturation, lightness) = if legacy_syntax {
        let (mut input, saturation) = parse_percentage(input)?;
        input = skip_ws(input);
        input = skip_ws(consume_byte(input, b',')?);
        let (mut input, lightness) = parse_percentage(input)?;
        input = skip_ws(input);
        (input, saturation, lightness)
    } else {
        let (input, saturation) = if let Ok((input, saturation)) = parse_number_percentage(input) {
            (skip_ws(input), saturation)
        } else {
            (skip_ws(consume_none(input)?), NONE)
        };
        let (input, lightness) = if let Ok((input, lightness)) = parse_number_percentage(input) {
            (skip_ws(input), lightness)
        } else {
            (skip_ws(consume_none(input)?), NONE)
        };
        (input, saturation, lightness)
    };

    let (input, alpha) = match (input.get(0), legacy_syntax) {
        (Some(b'/'), false) | (Some(b','), true) => {
            let input = skip_ws(&input[1..]);
            if let Ok((input, alpha)) = parse_alpha_value(input) {
                (skip_ws(input), alpha)
            } else if !legacy_syntax {
                (skip_ws(consume_none(input)?), NONE)
            } else {
                return Err(());
            }
        }
        _ => (input, 1.),
    };

    if input != b")" {
        return Err(());
    }

    Ok(Srgb::from(Hsla {
        hue: normalize_hue(hue),
        saturation: clamp_unit_f32(saturation),
        lightness: clamp_unit_f32(lightness),
        alpha,
    }))
}

// hwb() = hwb( [<hue> | none]
//              [<percentage> | <number> | none]
//              [<percentage> | <number> | none]
//              [ / [<alpha-value> | none] ]? )
fn parse_hwb(input: &[u8]) -> Result<Srgb, ()> {
    let (input, hue) = if let Ok((input, hue)) = parse_hue(input) {
        (skip_ws(input), hue)
    } else {
        (skip_ws(consume_none(input)?), NONE)
    };

    let (input, whiteness) = if let Ok((input, whiteness)) = parse_number_percentage(input) {
        (skip_ws(input), whiteness)
    } else {
        (skip_ws(consume_none(input)?), NONE)
    };

    let (input, blackness) = if let Ok((input, blackness)) = parse_number_percentage(input) {
        (skip_ws(input), blackness)
    } else {
        (skip_ws(consume_none(input)?), NONE)
    };

    let (input, alpha) = match input.get(0) {
        Some(b'/') => {
            let input = skip_ws(&input[1..]);
            if let Ok((input, alpha)) = parse_alpha_value(input) {
                (skip_ws(input), alpha)
            } else {
                (skip_ws(consume_none(input)?), NONE)
            }
        }
        _ => (input, 1.),
    };

    if input != b")" {
        return Err(());
    }

    Ok(Srgb::from(Hwba {
        hue: normalize_hue(hue),
        whiteness: clamp_unit_f32(whiteness),
        blackness: clamp_unit_f32(blackness),
        alpha,
    }))
}

// rgb()  = [ <legacy-rgb-syntax>  | <modern-rgb-syntax>  ]
// rgba() = [ <legacy-rgba-syntax> | <modern-rgba-syntax> ]
// <legacy-rgb-syntax>  = rgb(  <percentage>#{3} , <alpha-value>? ) |
//                        rgb(  <number>#{3}     , <alpha-value>? )
// <legacy-rgba-syntax> = rgba( <percentage>#{3} , <alpha-value>? ) |
//                        rgba( <number>#{3}     , <alpha-value>? )
// <modern-rgb-syntax>  = rgb(  [ <number> | <percentage> | none]{3} [ / [<alpha-value> | none] ]? )
// <modern-rgba-syntax> = rgba( [ <number> | <percentage> | none]{3} [ / [<alpha-value> | none] ]? )
fn parse_rgb(input: &[u8]) -> Result<Srgb, ()> {
    let (input, red, legacy_syntax) = if let Ok((input, red)) = parse_number_or_percentage(input) {
        let input = skip_ws(input);
        match input.get(0) {
            Some(b',') => (skip_ws(&input[1..]), Some(red), true),
            _ => (input, Some(red), false),
        }
    } else {
        (skip_ws(consume_none(input)?), None, false)
    };

    let (input, red, green, blue) = if legacy_syntax {
        match red.unwrap() {
            Number(red) => {
                let (mut input, green) = parse_number(input)?;
                input = skip_ws(input);
                input = skip_ws(consume_byte(input, b',')?);
                let (mut input, blue) = parse_number(input)?;
                input = skip_ws(input);
                (input, red / 255., green / 255., blue / 255.)
            }
            Percentage(red) => {
                let (mut input, green) = parse_percentage(input)?;
                input = skip_ws(input);
                input = skip_ws(consume_byte(input, b',')?);
                let (mut input, blue) = parse_percentage(input)?;
                input = skip_ws(input);
                (input, red, green, blue)
            }
        }
    } else {
        let red = match red {
            Some(Number(red)) => red / 255.,
            Some(Percentage(red)) => red,
            None => NONE,
        };
        let (input, green) = if let Ok((input, green)) = parse_number_or_percentage(input) {
            let green = match green {
                Number(green) => green / 255.,
                Percentage(green) => green,
            };
            (skip_ws(input), green)
        } else {
            (skip_ws(consume_none(input)?), NONE)
        };
        let (input, blue) = if let Ok((input, blue)) = parse_number_or_percentage(input) {
            let blue = match blue {
                Number(blue) => blue / 255.,
                Percentage(blue) => blue,
            };
            (skip_ws(input), blue)
        } else {
            (skip_ws(consume_none(input)?), NONE)
        };
        (input, red, green, blue)
    };

    let (input, alpha) = match (input.get(0), legacy_syntax) {
        (Some(b'/'), false) | (Some(b','), true) => {
            let input = skip_ws(&input[1..]);
            if let Ok((input, alpha)) = parse_alpha_value(input) {
                (skip_ws(input), alpha)
            } else if !legacy_syntax {
                (skip_ws(consume_none(input)?), NONE)
            } else {
                return Err(());
            }
        }
        _ => (input, 1.),
    };

    if input != b")" {
        return Err(());
    }

    Ok(Srgb::new(
        clamp_unit_f32(red),
        clamp_unit_f32(green),
        clamp_unit_f32(blue),
        alpha,
    ))
}

macro_rules! rgb {
    ($red: expr, $green: expr, $blue: expr) => {
        Srgb::from_rgb8($red, $green, $blue)
    };
}

fn parse_named(input: &[u8]) -> Result<Srgb, ()> {
    const NAMED_MAX_LEN: usize = 20;
    if input.len() > NAMED_MAX_LEN {
        return Err(());
    }
    let mut name = [b'\0'; NAMED_MAX_LEN];
    let name = &mut name[..input.len()];
    for (i, c) in input.iter().enumerate() {
        name[i] = c.to_ascii_lowercase();
    }
    Ok(match &*name {
        b"aliceblue" => rgb!(240, 248, 255),
        b"antiquewhite" => rgb!(250, 235, 215),
        b"aqua" => rgb!(0, 255, 255),
        b"aquamarine" => rgb!(127, 255, 212),
        b"azure" => rgb!(240, 255, 255),
        b"beige" => rgb!(245, 245, 220),
        b"bisque" => rgb!(255, 228, 196),
        b"black" => rgb!(0, 0, 0),
        b"blanchedalmond" => rgb!(255, 235, 205),
        b"blue" => rgb!(0, 0, 255),
        b"blueviolet" => rgb!(138, 43, 226),
        b"brown" => rgb!(165, 42, 42),
        b"burlywood" => rgb!(222, 184, 135),
        b"cadetblue" => rgb!(95, 158, 160),
        b"chartreuse" => rgb!(127, 255, 0),
        b"chocolate" => rgb!(210, 105, 30),
        b"coral" => rgb!(255, 127, 80),
        b"cornflowerblue" => rgb!(100, 149, 237),
        b"cornsilk" => rgb!(255, 248, 220),
        b"crimson" => rgb!(220, 20, 60),
        b"cyan" => rgb!(0, 255, 255),
        b"darkblue" => rgb!(0, 0, 139),
        b"darkcyan" => rgb!(0, 139, 139),
        b"darkgoldenrod" => rgb!(184, 134, 11),
        b"darkgray" => rgb!(169, 169, 169),
        b"darkgreen" => rgb!(0, 100, 0),
        b"darkgrey" => rgb!(169, 169, 169),
        b"darkkhaki" => rgb!(189, 183, 107),
        b"darkmagenta" => rgb!(139, 0, 139),
        b"darkolivegreen" => rgb!(85, 107, 47),
        b"darkorange" => rgb!(255, 140, 0),
        b"darkorchid" => rgb!(153, 50, 204),
        b"darkred" => rgb!(139, 0, 0),
        b"darksalmon" => rgb!(233, 150, 122),
        b"darkseagreen" => rgb!(143, 188, 143),
        b"darkslateblue" => rgb!(72, 61, 139),
        b"darkslategray" => rgb!(47, 79, 79),
        b"darkslategrey" => rgb!(47, 79, 79),
        b"darkturquoise" => rgb!(0, 206, 209),
        b"darkviolet" => rgb!(148, 0, 211),
        b"deeppink" => rgb!(255, 20, 147),
        b"deepskyblue" => rgb!(0, 191, 255),
        b"dimgray" => rgb!(105, 105, 105),
        b"dimgrey" => rgb!(105, 105, 105),
        b"dodgerblue" => rgb!(30, 144, 255),
        b"firebrick" => rgb!(178, 34, 34),
        b"floralwhite" => rgb!(255, 250, 240),
        b"forestgreen" => rgb!(34, 139, 34),
        b"fuchsia" => rgb!(255, 0, 255),
        b"gainsboro" => rgb!(220, 220, 220),
        b"ghostwhite" => rgb!(248, 248, 255),
        b"gold" => rgb!(255, 215, 0),
        b"goldenrod" => rgb!(218, 165, 32),
        b"gray" => rgb!(128, 128, 128),
        b"green" => rgb!(0, 128, 0),
        b"greenyellow" => rgb!(173, 255, 47),
        b"grey" => rgb!(128, 128, 128),
        b"honeydew" => rgb!(240, 255, 240),
        b"hotpink" => rgb!(255, 105, 180),
        b"indianred" => rgb!(205, 92, 92),
        b"indigo" => rgb!(75, 0, 130),
        b"ivory" => rgb!(255, 255, 240),
        b"khaki" => rgb!(240, 230, 140),
        b"lavender" => rgb!(230, 230, 250),
        b"lavenderblush" => rgb!(255, 240, 245),
        b"lawngreen" => rgb!(124, 252, 0),
        b"lemonchiffon" => rgb!(255, 250, 205),
        b"lightblue" => rgb!(173, 216, 230),
        b"lightcoral" => rgb!(240, 128, 128),
        b"lightcyan" => rgb!(224, 255, 255),
        b"lightgoldenrodyellow" => rgb!(250, 250, 210),
        b"lightgray" => rgb!(211, 211, 211),
        b"lightgreen" => rgb!(144, 238, 144),
        b"lightgrey" => rgb!(211, 211, 211),
        b"lightpink" => rgb!(255, 182, 193),
        b"lightsalmon" => rgb!(255, 160, 122),
        b"lightseagreen" => rgb!(32, 178, 170),
        b"lightskyblue" => rgb!(135, 206, 250),
        b"lightslategray" => rgb!(119, 136, 153),
        b"lightslategrey" => rgb!(119, 136, 153),
        b"lightsteelblue" => rgb!(176, 196, 222),
        b"lightyellow" => rgb!(255, 255, 224),
        b"lime" => rgb!(0, 255, 0),
        b"limegreen" => rgb!(50, 205, 50),
        b"linen" => rgb!(250, 240, 230),
        b"magenta" => rgb!(255, 0, 255),
        b"maroon" => rgb!(128, 0, 0),
        b"mediumaquamarine" => rgb!(102, 205, 170),
        b"mediumblue" => rgb!(0, 0, 205),
        b"mediumorchid" => rgb!(186, 85, 211),
        b"mediumpurple" => rgb!(147, 112, 219),
        b"mediumseagreen" => rgb!(60, 179, 113),
        b"mediumslateblue" => rgb!(123, 104, 238),
        b"mediumspringgreen" => rgb!(0, 250, 154),
        b"mediumturquoise" => rgb!(72, 209, 204),
        b"mediumvioletred" => rgb!(199, 21, 133),
        b"midnightblue" => rgb!(25, 25, 112),
        b"mintcream" => rgb!(245, 255, 250),
        b"mistyrose" => rgb!(255, 228, 225),
        b"moccasin" => rgb!(255, 228, 181),
        b"navajowhite" => rgb!(255, 222, 173),
        b"navy" => rgb!(0, 0, 128),
        b"oldlace" => rgb!(253, 245, 230),
        b"olive" => rgb!(128, 128, 0),
        b"olivedrab" => rgb!(107, 142, 35),
        b"orange" => rgb!(255, 165, 0),
        b"orangered" => rgb!(255, 69, 0),
        b"orchid" => rgb!(218, 112, 214),
        b"palegoldenrod" => rgb!(238, 232, 170),
        b"palegreen" => rgb!(152, 251, 152),
        b"paleturquoise" => rgb!(175, 238, 238),
        b"palevioletred" => rgb!(219, 112, 147),
        b"papayawhip" => rgb!(255, 239, 213),
        b"peachpuff" => rgb!(255, 218, 185),
        b"peru" => rgb!(205, 133, 63),
        b"pink" => rgb!(255, 192, 203),
        b"plum" => rgb!(221, 160, 221),
        b"powderblue" => rgb!(176, 224, 230),
        b"purple" => rgb!(128, 0, 128),
        b"rebeccapurple" => rgb!(102, 51, 153),
        b"red" => rgb!(255, 0, 0),
        b"rosybrown" => rgb!(188, 143, 143),
        b"royalblue" => rgb!(65, 105, 225),
        b"saddlebrown" => rgb!(139, 69, 19),
        b"salmon" => rgb!(250, 128, 114),
        b"sandybrown" => rgb!(244, 164, 96),
        b"seagreen" => rgb!(46, 139, 87),
        b"seashell" => rgb!(255, 245, 238),
        b"sienna" => rgb!(160, 82, 45),
        b"silver" => rgb!(192, 192, 192),
        b"skyblue" => rgb!(135, 206, 235),
        b"slateblue" => rgb!(106, 90, 205),
        b"slategray" => rgb!(112, 128, 144),
        b"slategrey" => rgb!(112, 128, 144),
        b"snow" => rgb!(255, 250, 250),
        b"springgreen" => rgb!(0, 255, 127),
        b"steelblue" => rgb!(70, 130, 180),
        b"tan" => rgb!(210, 180, 140),
        b"teal" => rgb!(0, 128, 128),
        b"thistle" => rgb!(216, 191, 216),
        b"tomato" => rgb!(255, 99, 71),
        b"turquoise" => rgb!(64, 224, 208),
        b"violet" => rgb!(238, 130, 238),
        b"wheat" => rgb!(245, 222, 179),
        b"white" => rgb!(255, 255, 255),
        b"whitesmoke" => rgb!(245, 245, 245),
        b"yellow" => rgb!(255, 255, 0),
        b"yellowgreen" => rgb!(154, 205, 50),
        b"transparent" => Srgb::new(0., 0., 0., 0.),
        _ => return Err(()),
    })
}

#[cfg(test)]
mod tests;
