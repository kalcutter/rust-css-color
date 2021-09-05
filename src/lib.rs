#![cfg_attr(feature = "bench", feature(test))]

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::f32;
use std::str::{self, FromStr};

/// A color with RGBA components, represented as floats.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    /// The red component.
    pub red: f32,
    /// The green component.
    pub green: f32,
    /// The blue component.
    pub blue: f32,
    /// The alpha component.
    pub alpha: f32,
}

impl Rgba {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Rgba {
        Rgba {
            red,
            green,
            blue,
            alpha,
        }
    }

    fn from_rgb8(red: u8, green: u8, blue: u8) -> Rgba {
        Rgba::from_rgba8(red, green, blue, 255)
    }

    fn from_rgba8(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba {
        Rgba {
            red: red as f32 / 255.,
            green: green as f32 / 255.,
            blue: blue as f32 / 255.,
            alpha: alpha as f32 / 255.,
        }
    }
}

#[derive(Debug)]
pub struct ParseColorError;

impl FromStr for Rgba {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_css_color(s.as_bytes()).map_err(|_| ParseColorError)
    }
}

// https://www.w3.org/TR/css-color-4/
fn parse_css_color(input: &[u8]) -> Result<Rgba, ()> {
    if input.is_empty() {
        Err(())
    } else if let Ok(input) = expect_byte(input, b'#') {
        parse_hash(input)
    } else if let Ok(input) = expect_function(input, b"rgb") {
        parse_rgb(input)
    } else if let Ok(input) = expect_function(input, b"rgba") {
        parse_rgb(input)
    } else if let Ok(input) = expect_function(input, b"hsl") {
        parse_hsl(input)
    } else if let Ok(input) = expect_function(input, b"hsla") {
        parse_hsl(input)
    } else {
        parse_named(input)
    }
}

#[inline]
fn clamp_unit_f32(value: f32) -> f32 {
    value.max(0.).min(1.)
}

#[inline]
fn normalize_hue(value: f32) -> f32 {
    value - value.floor()
}

struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
}

impl From<Hsla> for Rgba {
    fn from(hsla: Hsla) -> Self {
        let t2 = if hsla.lightness <= 0.5 {
            hsla.lightness * (hsla.saturation + 1.)
        } else {
            hsla.lightness + hsla.saturation - hsla.lightness * hsla.saturation
        };
        let t1 = hsla.lightness * 2. - t2;
        let h6 = hsla.hue * 6.;

        fn hue_to_rgb(t1: f32, t2: f32, mut h6: f32) -> f32 {
            if h6 < 0. {
                h6 += 6.;
            } else if h6 >= 6. {
                h6 -= 6.;
            }

            if h6 < 1. {
                (t2 - t1) * h6 + t1
            } else if h6 < 3. {
                t2
            } else if h6 < 4. {
                (t2 - t1) * (4. - h6) + t1
            } else {
                t1
            }
        }
        Rgba {
            red: hue_to_rgb(t1, t2, h6 + 2.),
            green: hue_to_rgb(t1, t2, h6),
            blue: hue_to_rgb(t1, t2, h6 - 2.),
            alpha: hsla.alpha,
        }
    }
}

#[inline]
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

#[inline]
fn is_name_start(c: u8) -> bool {
    match c {
        b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
        c => !c.is_ascii() || c == b'\0',
    }
}

#[inline]
fn is_name(c: u8) -> bool {
    match c {
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'-' => true,
        c => !c.is_ascii() || c == b'\0',
    }
}

#[inline]
fn is_whitespace(c: u8) -> bool {
    c <= b' ' && (c == b' ' || c == b'\n' || c == b'\t' || c == b'\r' || c == b'\x0C')
}

#[inline]
fn digit(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(()),
    }
}

#[inline]
fn hexdigit(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(()),
    }
}

#[inline]
fn expect_byte(input: &[u8], b: u8) -> Result<&[u8], ()> {
    match input.get(0) {
        Some(c) if *c == b => Ok(&input[1..]),
        _ => Err(()),
    }
}

#[inline]
fn expect_function<'a>(input: &'a [u8], name: &[u8]) -> Result<&'a [u8], ()> {
    debug_assert!(is_ident_start(name));

    let n = name.len();
    if input.len() >= n + 1 && input[..n].eq_ignore_ascii_case(name) && input[n] == b'(' {
        Ok(&input[n + 1..])
    } else {
        Err(())
    }
}

#[inline]
fn expect_name<'a>(input: &'a [u8], name: &[u8]) -> Result<&'a [u8], ()> {
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

fn consume_number(mut input: &[u8]) -> Result<&[u8], ()> {
    fn skip_sign(input: &[u8]) -> &[u8] {
        match input.get(0) {
            Some(b'+') | Some(b'-') => &input[1..],
            _ => input,
        }
    }
    fn consume_integer(mut input: &[u8]) -> Result<&[u8], ()> {
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
            input = consume_integer(input)?;
        }
    }
    if let Some(b'.') = input.get(0) {
        input = consume_integer(&input[1..])?;
    }
    match input.get(0) {
        Some(b'E') | Some(b'e') => {
            input = skip_sign(&input[1..]);
            input = consume_integer(input)?;
        }
        _ => {}
    }
    Ok(input)
}

#[inline]
fn skip_whitespace(mut input: &[u8]) -> &[u8] {
    while input.len() > 0 && is_whitespace(input[0]) {
        input = &input[1..];
    }
    input
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
    let input = expect_byte(input, b'%')?;

    Ok((input, value / 100.))
}

enum NumberOrPercentage {
    Number(f32),
    Percentage(f32),
}
use self::NumberOrPercentage::*;

fn parse_number_or_percentage(input: &[u8]) -> Result<(&[u8], NumberOrPercentage), ()> {
    let (input, value) = parse_number(input)?;

    if let Ok(input) = expect_byte(input, b'%') {
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
    } else if let Ok(input) = expect_name(input, b"deg") {
        Ok((input, value / 360.))
    } else if let Ok(input) = expect_name(input, b"grad") {
        Ok((input, value / 400.))
    } else if let Ok(input) = expect_name(input, b"rad") {
        Ok((input, value / (2. * f32::consts::PI)))
    } else if let Ok(input) = expect_name(input, b"turn") {
        Ok((input, value))
    } else {
        Err(())
    }
}

/// Parse RGB hexadecimal colors.
fn parse_hash(input: &[u8]) -> Result<Rgba, ()> {
    match input.len() {
        8 => Ok(Rgba::from_rgba8(
            hexdigit(input[0])? * 16 + hexdigit(input[1])?,
            hexdigit(input[2])? * 16 + hexdigit(input[3])?,
            hexdigit(input[4])? * 16 + hexdigit(input[5])?,
            hexdigit(input[6])? * 16 + hexdigit(input[7])?,
        )),
        6 => Ok(Rgba::from_rgb8(
            hexdigit(input[0])? * 16 + hexdigit(input[1])?,
            hexdigit(input[2])? * 16 + hexdigit(input[3])?,
            hexdigit(input[4])? * 16 + hexdigit(input[5])?,
        )),
        4 => Ok(Rgba::from_rgba8(
            hexdigit(input[0])? * 17,
            hexdigit(input[1])? * 17,
            hexdigit(input[2])? * 17,
            hexdigit(input[3])? * 17,
        )),
        3 => Ok(Rgba::from_rgb8(
            hexdigit(input[0])? * 17,
            hexdigit(input[1])? * 17,
            hexdigit(input[2])? * 17,
        )),
        _ => Err(()),
    }
}

// hsl() = hsl( <hue> <percentage> <percentage> [ / <alpha-value> ]? )
//         hsl( <hue>, <percentage>, <percentage> [ , <alpha-value> ]? )
fn parse_hsl(input: &[u8]) -> Result<Rgba, ()> {
    let input = skip_whitespace(input);
    let (input, hue) = parse_hue(input)?;

    let input = skip_whitespace(input);
    let (input, uses_commas) = match input.get(0) {
        Some(b',') => (skip_whitespace(&input[1..]), true),
        _ => (input, false),
    };
    let (input, saturation) = parse_percentage(input)?;

    let mut input = skip_whitespace(input);
    if uses_commas {
        input = expect_byte(input, b',')?;
        input = skip_whitespace(input);
    }
    let (input, lightness) = parse_percentage(input)?;

    let input = skip_whitespace(input);
    let (input, has_alpha) = match input.get(0) {
        Some(b',') if uses_commas => (&input[1..], true),
        Some(b'/') if !uses_commas => (&input[1..], true),
        _ => (input, false),
    };
    let input = skip_whitespace(input);
    let (input, alpha) = if has_alpha {
        let (input, alpha) = parse_alpha_value(input)?;
        let input = skip_whitespace(input);
        (input, alpha)
    } else {
        (input, 1.)
    };

    if input != b")" {
        return Err(());
    }

    Ok(Rgba::from(Hsla {
        hue: normalize_hue(hue),
        saturation: clamp_unit_f32(saturation),
        lightness: clamp_unit_f32(lightness),
        alpha,
    }))
}

// rgb() = rgb( <percentage>{3} [ / <alpha-value> ]? )
//         rgb( <number>{3} [ / <alpha-value> ]? )
//         rgb( <percentage>#{3} [ , <alpha-value> ]? )
//         rgb( <number>#{3} [ , <alpha-value> ]? )
fn parse_rgb(input: &[u8]) -> Result<Rgba, ()> {
    let input = skip_whitespace(input);
    let (input, red) = parse_number_or_percentage(input)?;

    let input = skip_whitespace(input);
    let (input, uses_commas) = match input.get(0) {
        Some(b',') => (skip_whitespace(&input[1..]), true),
        _ => (input, false),
    };

    let (input, red, green, blue) = match red {
        Number(red) => {
            let (input, green) = parse_number(input)?;

            let mut input = skip_whitespace(input);
            if uses_commas {
                input = expect_byte(input, b',')?;
                input = skip_whitespace(input);
            }
            let (input, blue) = parse_number(input)?;

            (input, red / 255., green / 255., blue / 255.)
        }
        Percentage(red) => {
            let (input, green) = parse_percentage(input)?;

            let mut input = skip_whitespace(input);
            if uses_commas {
                input = expect_byte(input, b',')?;
                input = skip_whitespace(input);
            }
            let (input, blue) = parse_percentage(input)?;

            (input, red, green, blue)
        }
    };

    let input = skip_whitespace(input);
    let (input, has_alpha) = match input.get(0) {
        Some(b',') if uses_commas => (&input[1..], true),
        Some(b'/') if !uses_commas => (&input[1..], true),
        _ => (input, false),
    };
    let input = skip_whitespace(input);
    let (input, alpha) = if has_alpha {
        let (input, alpha) = parse_alpha_value(input)?;
        let input = skip_whitespace(input);
        (input, alpha)
    } else {
        (input, 1.)
    };

    if input != b")" {
        return Err(());
    }

    Ok(Rgba::new(
        clamp_unit_f32(red),
        clamp_unit_f32(green),
        clamp_unit_f32(blue),
        alpha,
    ))
}

fn parse_named(input: &[u8]) -> Result<Rgba, ()> {
    if input.len() <= NAMED_COLORS_MAX_LENGTH {
        let mut name = [b'\0'; NAMED_COLORS_MAX_LENGTH];
        let name = &mut name[..input.len()];
        for (i, c) in input.iter().enumerate() {
            name[i] = c.to_ascii_lowercase();
        }

        if let Some(&color) = NAMED_COLORS.get(name) {
            return Ok(color);
        }
    }
    Err(())
}

macro_rules! rgb {
    ($red: expr, $green: expr, $blue: expr) => {
        Rgba::from_rgb8($red, $green, $blue)
    };
}

lazy_static! {
    static ref NAMED_COLORS: HashMap<&'static [u8], Rgba> = {
        let mut m: HashMap<&'static [u8], _> = HashMap::new();

        m.insert(b"aliceblue", rgb!(240, 248, 255));
        m.insert(b"antiquewhite", rgb!(250, 235, 215));
        m.insert(b"aqua", rgb!(0, 255, 255));
        m.insert(b"aquamarine", rgb!(127, 255, 212));
        m.insert(b"azure", rgb!(240, 255, 255));
        m.insert(b"beige", rgb!(245, 245, 220));
        m.insert(b"bisque", rgb!(255, 228, 196));
        m.insert(b"black", rgb!(0, 0, 0));
        m.insert(b"blanchedalmond", rgb!(255, 235, 205));
        m.insert(b"blue", rgb!(0, 0, 255));
        m.insert(b"blueviolet", rgb!(138, 43, 226));
        m.insert(b"brown", rgb!(165, 42, 42));
        m.insert(b"burlywood", rgb!(222, 184, 135));
        m.insert(b"cadetblue", rgb!(95, 158, 160));
        m.insert(b"chartreuse", rgb!(127, 255, 0));
        m.insert(b"chocolate", rgb!(210, 105, 30));
        m.insert(b"coral", rgb!(255, 127, 80));
        m.insert(b"cornflowerblue", rgb!(100, 149, 237));
        m.insert(b"cornsilk", rgb!(255, 248, 220));
        m.insert(b"crimson", rgb!(220, 20, 60));
        m.insert(b"cyan", rgb!(0, 255, 255));
        m.insert(b"darkblue", rgb!(0, 0, 139));
        m.insert(b"darkcyan", rgb!(0, 139, 139));
        m.insert(b"darkgoldenrod", rgb!(184, 134, 11));
        m.insert(b"darkgray", rgb!(169, 169, 169));
        m.insert(b"darkgreen", rgb!(0, 100, 0));
        m.insert(b"darkgrey", rgb!(169, 169, 169));
        m.insert(b"darkkhaki", rgb!(189, 183, 107));
        m.insert(b"darkmagenta", rgb!(139, 0, 139));
        m.insert(b"darkolivegreen", rgb!(85, 107, 47));
        m.insert(b"darkorange", rgb!(255, 140, 0));
        m.insert(b"darkorchid", rgb!(153, 50, 204));
        m.insert(b"darkred", rgb!(139, 0, 0));
        m.insert(b"darksalmon", rgb!(233, 150, 122));
        m.insert(b"darkseagreen", rgb!(143, 188, 143));
        m.insert(b"darkslateblue", rgb!(72, 61, 139));
        m.insert(b"darkslategray", rgb!(47, 79, 79));
        m.insert(b"darkslategrey", rgb!(47, 79, 79));
        m.insert(b"darkturquoise", rgb!(0, 206, 209));
        m.insert(b"darkviolet", rgb!(148, 0, 211));
        m.insert(b"deeppink", rgb!(255, 20, 147));
        m.insert(b"deepskyblue", rgb!(0, 191, 255));
        m.insert(b"dimgray", rgb!(105, 105, 105));
        m.insert(b"dimgrey", rgb!(105, 105, 105));
        m.insert(b"dodgerblue", rgb!(30, 144, 255));
        m.insert(b"firebrick", rgb!(178, 34, 34));
        m.insert(b"floralwhite", rgb!(255, 250, 240));
        m.insert(b"forestgreen", rgb!(34, 139, 34));
        m.insert(b"fuchsia", rgb!(255, 0, 255));
        m.insert(b"gainsboro", rgb!(220, 220, 220));
        m.insert(b"ghostwhite", rgb!(248, 248, 255));
        m.insert(b"gold", rgb!(255, 215, 0));
        m.insert(b"goldenrod", rgb!(218, 165, 32));
        m.insert(b"gray", rgb!(128, 128, 128));
        m.insert(b"green", rgb!(0, 128, 0));
        m.insert(b"greenyellow", rgb!(173, 255, 47));
        m.insert(b"grey", rgb!(128, 128, 128));
        m.insert(b"honeydew", rgb!(240, 255, 240));
        m.insert(b"hotpink", rgb!(255, 105, 180));
        m.insert(b"indianred", rgb!(205, 92, 92));
        m.insert(b"indigo", rgb!(75, 0, 130));
        m.insert(b"ivory", rgb!(255, 255, 240));
        m.insert(b"khaki", rgb!(240, 230, 140));
        m.insert(b"lavender", rgb!(230, 230, 250));
        m.insert(b"lavenderblush", rgb!(255, 240, 245));
        m.insert(b"lawngreen", rgb!(124, 252, 0));
        m.insert(b"lemonchiffon", rgb!(255, 250, 205));
        m.insert(b"lightblue", rgb!(173, 216, 230));
        m.insert(b"lightcoral", rgb!(240, 128, 128));
        m.insert(b"lightcyan", rgb!(224, 255, 255));
        m.insert(b"lightgoldenrodyellow", rgb!(250, 250, 210));
        m.insert(b"lightgray", rgb!(211, 211, 211));
        m.insert(b"lightgreen", rgb!(144, 238, 144));
        m.insert(b"lightgrey", rgb!(211, 211, 211));
        m.insert(b"lightpink", rgb!(255, 182, 193));
        m.insert(b"lightsalmon", rgb!(255, 160, 122));
        m.insert(b"lightseagreen", rgb!(32, 178, 170));
        m.insert(b"lightskyblue", rgb!(135, 206, 250));
        m.insert(b"lightslategray", rgb!(119, 136, 153));
        m.insert(b"lightslategrey", rgb!(119, 136, 153));
        m.insert(b"lightsteelblue", rgb!(176, 196, 222));
        m.insert(b"lightyellow", rgb!(255, 255, 224));
        m.insert(b"lime", rgb!(0, 255, 0));
        m.insert(b"limegreen", rgb!(50, 205, 50));
        m.insert(b"linen", rgb!(250, 240, 230));
        m.insert(b"magenta", rgb!(255, 0, 255));
        m.insert(b"maroon", rgb!(128, 0, 0));
        m.insert(b"mediumaquamarine", rgb!(102, 205, 170));
        m.insert(b"mediumblue", rgb!(0, 0, 205));
        m.insert(b"mediumorchid", rgb!(186, 85, 211));
        m.insert(b"mediumpurple", rgb!(147, 112, 219));
        m.insert(b"mediumseagreen", rgb!(60, 179, 113));
        m.insert(b"mediumslateblue", rgb!(123, 104, 238));
        m.insert(b"mediumspringgreen", rgb!(0, 250, 154));
        m.insert(b"mediumturquoise", rgb!(72, 209, 204));
        m.insert(b"mediumvioletred", rgb!(199, 21, 133));
        m.insert(b"midnightblue", rgb!(25, 25, 112));
        m.insert(b"mintcream", rgb!(245, 255, 250));
        m.insert(b"mistyrose", rgb!(255, 228, 225));
        m.insert(b"moccasin", rgb!(255, 228, 181));
        m.insert(b"navajowhite", rgb!(255, 222, 173));
        m.insert(b"navy", rgb!(0, 0, 128));
        m.insert(b"oldlace", rgb!(253, 245, 230));
        m.insert(b"olive", rgb!(128, 128, 0));
        m.insert(b"olivedrab", rgb!(107, 142, 35));
        m.insert(b"orange", rgb!(255, 165, 0));
        m.insert(b"orangered", rgb!(255, 69, 0));
        m.insert(b"orchid", rgb!(218, 112, 214));
        m.insert(b"palegoldenrod", rgb!(238, 232, 170));
        m.insert(b"palegreen", rgb!(152, 251, 152));
        m.insert(b"paleturquoise", rgb!(175, 238, 238));
        m.insert(b"palevioletred", rgb!(219, 112, 147));
        m.insert(b"papayawhip", rgb!(255, 239, 213));
        m.insert(b"peachpuff", rgb!(255, 218, 185));
        m.insert(b"peru", rgb!(205, 133, 63));
        m.insert(b"pink", rgb!(255, 192, 203));
        m.insert(b"plum", rgb!(221, 160, 221));
        m.insert(b"powderblue", rgb!(176, 224, 230));
        m.insert(b"purple", rgb!(128, 0, 128));
        m.insert(b"rebeccapurple", rgb!(102, 51, 153));
        m.insert(b"red", rgb!(255, 0, 0));
        m.insert(b"rosybrown", rgb!(188, 143, 143));
        m.insert(b"royalblue", rgb!(65, 105, 225));
        m.insert(b"saddlebrown", rgb!(139, 69, 19));
        m.insert(b"salmon", rgb!(250, 128, 114));
        m.insert(b"sandybrown", rgb!(244, 164, 96));
        m.insert(b"seagreen", rgb!(46, 139, 87));
        m.insert(b"seashell", rgb!(255, 245, 238));
        m.insert(b"sienna", rgb!(160, 82, 45));
        m.insert(b"silver", rgb!(192, 192, 192));
        m.insert(b"skyblue", rgb!(135, 206, 235));
        m.insert(b"slateblue", rgb!(106, 90, 205));
        m.insert(b"slategray", rgb!(112, 128, 144));
        m.insert(b"slategrey", rgb!(112, 128, 144));
        m.insert(b"snow", rgb!(255, 250, 250));
        m.insert(b"springgreen", rgb!(0, 255, 127));
        m.insert(b"steelblue", rgb!(70, 130, 180));
        m.insert(b"tan", rgb!(210, 180, 140));
        m.insert(b"teal", rgb!(0, 128, 128));
        m.insert(b"thistle", rgb!(216, 191, 216));
        m.insert(b"tomato", rgb!(255, 99, 71));
        m.insert(b"turquoise", rgb!(64, 224, 208));
        m.insert(b"violet", rgb!(238, 130, 238));
        m.insert(b"wheat", rgb!(245, 222, 179));
        m.insert(b"white", rgb!(255, 255, 255));
        m.insert(b"whitesmoke", rgb!(245, 245, 245));
        m.insert(b"yellow", rgb!(255, 255, 0));
        m.insert(b"yellowgreen", rgb!(154, 205, 50));

        m.insert(b"transparent", Rgba::new(0.0, 0.0, 0.0, 0.0));

        m
    };
}
const NAMED_COLORS_MAX_LENGTH: usize = 20;

#[cfg(test)]
mod tests;
