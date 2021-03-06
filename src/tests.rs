use super::{Rgba, NAMED_COLORS, NAMED_COLORS_MAX_LENGTH};
use std::str::FromStr;

#[cfg(feature = "bench")]
extern crate test;

#[test]
fn readme_examples() {
    let lime = Rgba::new(0., 1., 0., 1.);
    assert_eq!(lime, "#0f0".parse().unwrap());
    assert_eq!(lime, "rgb(0 255 0)".parse().unwrap());
    assert_eq!(lime, "rgb(0% 100% 0%)".parse().unwrap());
    assert_eq!(lime, "hsl(120deg 100% 50% / 100%)".parse().unwrap());
    assert_eq!(lime, "lime".parse().unwrap());
}

#[test]
fn check_named_colors() {
    assert!(
        NAMED_COLORS.keys().all(|k| k.to_ascii_lowercase() == *k),
        "NAMED_COLORS must be given in ASCII lowercase"
    );
    assert_eq!(
        NAMED_COLORS.keys().map(|k| k.len()).max().unwrap(),
        NAMED_COLORS_MAX_LENGTH
    );
}

#[test]
fn color4_spec_examples() {
    // EXAMPLE 1
    assert!(Rgba::from_str("lime").is_ok());
    assert!(Rgba::from_str("rgb(0 255 0)").is_ok());
    assert!(Rgba::from_str("rgb(0% 100% 0%)").is_ok());

    // EXAMPLE 2
    assert_eq!(
        Rgba::from_str("#00ff00").unwrap(),
        Rgba::from_str("rgb(0 255 0)").unwrap()
    );

    // EXAMPLE 3
    assert_eq!(
        Rgba::from_str("#0000ffcc").unwrap(),
        Rgba::from_str("rgb(0% 0% 100% / 80%)").unwrap()
    );

    // EXAMPLE 4
    assert_eq!(
        Rgba::from_str("#123").unwrap(),
        Rgba::from_str("#112233").unwrap()
    );

    // EXAMPLE 7
    let red = Rgba::from_str("red").unwrap();
    assert_eq!(red, Rgba::from_str("#f00").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(0deg 100% 50%)").unwrap());

    // EXAMPLE 8
    assert!(Rgba::from_str("hsl(120deg 100% 50%)").is_ok());
    assert!(Rgba::from_str("hsl(120deg 100% 25%)").is_ok());
    assert!(Rgba::from_str("hsl(120deg 100% 75%)").is_ok());
    assert!(Rgba::from_str("hsl(120deg 75% 85%)").is_ok());
}

#[test]
fn hash() {
    assert!(Rgba::from_str("#").is_err());
    assert!(Rgba::from_str("#f").is_err());
    assert!(Rgba::from_str("#ff").is_err());
    assert_eq!(
        Rgba::from_rgba8(255, 255, 255, 255),
        Rgba::from_str("#fff").unwrap()
    );
    assert!(Rgba::from_str("#ffg").is_err());
    assert_eq!(
        Rgba::from_rgba8(255, 255, 255, 255),
        Rgba::from_str("#ffff").unwrap()
    );
    assert!(Rgba::from_str("#fffg").is_err());
    assert!(Rgba::from_str("#fffff").is_err());
    assert_eq!(
        Rgba::from_rgba8(255, 255, 255, 255),
        Rgba::from_str("#ffffff").unwrap()
    );
    assert!(Rgba::from_str("#fffffg").is_err());
    assert!(Rgba::from_str("#fffffff").is_err());
    assert_eq!(
        Rgba::from_rgba8(255, 255, 255, 255),
        Rgba::from_str("#ffffffff").unwrap()
    );
    assert!(Rgba::from_str("#fffffffg").is_err());
    assert!(Rgba::from_str("#fffffffff").is_err());

    assert!(Rgba::from_str("#+").is_err());
    assert!(Rgba::from_str("#+0").is_err());
    assert!(Rgba::from_str("#+00").is_err());
    assert!(Rgba::from_str("#+000").is_err());
    assert!(Rgba::from_str("#+0000").is_err());
    assert!(Rgba::from_str("#+00000").is_err());
    assert!(Rgba::from_str("#+000000").is_err());
    assert!(Rgba::from_str("#+0000000").is_err());
    assert!(Rgba::from_str("#+00000000").is_err());

    assert!(Rgba::from_str("#-").is_err());
    assert!(Rgba::from_str("#-0").is_err());
    assert!(Rgba::from_str("#-00").is_err());
    assert!(Rgba::from_str("#-000").is_err());
    assert!(Rgba::from_str("#-0000").is_err());
    assert!(Rgba::from_str("#-00000").is_err());
    assert!(Rgba::from_str("#-000000").is_err());
    assert!(Rgba::from_str("#-0000000").is_err());
    assert!(Rgba::from_str("#-00000000").is_err());
}

#[test]
fn rgb() {
    let transparent = Rgba {
        red: 0.,
        green: 0.,
        blue: 0.,
        alpha: 0.,
    };
    assert_eq!(transparent, Rgba::from_str("rgb(0 0 0 / 0)").unwrap());
    assert_eq!(transparent, Rgba::from_str("rgb(0% 0% 0% / 0%)").unwrap());

    let rebeccapurple = Rgba {
        red: 102. / 255.,
        green: 51. / 255.,
        blue: 153. / 255.,
        alpha: 1.,
    };
    assert_eq!(rebeccapurple, Rgba::from_str("rgb(102 51 153)").unwrap());
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(102 51 153 / 1)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(102 51 153 / 100%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(102.0 51.0 153.0 / 100.0%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(102.0e0 51.0e0 153.0e0 / 100.0e0%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(102e0 51e0 153e0 / 100e0%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(10.2e1 5.1e1 15.3e1 / 10.0e1%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(1.02e2 .51e2 1.53e2 / 1.00e2%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(.102e3 .051e3 .153e3 / .100e3%)").unwrap()
    );

    assert_eq!(rebeccapurple, Rgba::from_str("rgb(40% 20% 60%)").unwrap());
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(40% 20% 60% / 100%)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(40% 20% 60% / 1)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(40.0% 20.0% 60.0% / 1.0)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(40.0E0% 20.0E0% 60.0E0% / 1.0E0)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(40E0% 20E0% 60E0% / 1E0)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(4.0E+1% 2.0E+1% 6.0E+1% / .1E+1)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(.40E+2% .20E+2% .60E+2% / .01E+2)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(.040E+3% .020E+3% .060E+3% / .001E+3)").unwrap()
    );
    assert_eq!(
        rebeccapurple,
        Rgba::from_str("rgb(400E-1% 200E-1% 600E-1% / 1)").unwrap()
    );

    assert_eq!(
        Rgba {
            red: 0.10,
            green: 0.20,
            blue: 0.30,
            alpha: 1.0
        },
        Rgba::from_str("rgb(10%,20%,30%)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 12. / 100.,
            green: 34. / 100.,
            blue: 56. / 100.,
            alpha: 1.0
        },
        Rgba::from_str("rgb(12%,34%,56%)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 12.3 / 100.,
            green: 45.6 / 100.,
            blue: 78.9 / 100.,
            alpha: 0.0
        },
        Rgba::from_str("rgba(12.3%,45.6%,78.9%,0.00)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 11.1111 / 100.,
            green: 22.2222 / 100.,
            blue: 33.3333 / 100.,
            alpha: 0.444444,
        },
        Rgba::from_str("rgba(11.1111%, 22.2222%, 33.3333%, 0.444444)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 44.4444 / 100.,
            green: 55.5555 / 100.,
            blue: 66.6666 / 100.,
            alpha: 0.777777
        },
        Rgba::from_str("rgba(44.4444%, 55.5555%, 66.6666%, 0.777777)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 77.7777 / 100.,
            green: 88.8888 / 100.,
            blue: 99.9999 / 100.,
            alpha: 0.000000
        },
        Rgba::from_str("rgba(77.7777%, 88.8888%, 99.9999%, 0.000000)").unwrap()
    );

    assert_eq!(
        Rgba {
            red: 12.3456789 / 100.,
            green: 1.,
            blue: 1.,
            alpha: 1.
        },
        Rgba::from_str("rgba(012.3456789%, 200%, 300%, 4.000000)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 98.7654321 / 100.,
            green: 0.,
            blue: 0.,
            alpha: 0.
        },
        Rgba::from_str("rgba(098.7654321%, -200%, -300%, -4.000000)").unwrap()
    );

    assert_eq!(
        Rgba::from_rgb8(128, 64, 32),
        Rgba::from_str(&format!("rgba(128,64,32,{})", "9".repeat(512))).unwrap()
    );

    assert!(Rgba::from_str("rgb(+0 +0 +0 )").is_ok());
    assert!(Rgba::from_str("rgb(+0 +0 +0%)").is_err());
    assert!(Rgba::from_str("rgb(+0 +0%+0 )").is_err());
    assert!(Rgba::from_str("rgb(+0 +0%+0%)").is_err());
    assert!(Rgba::from_str("rgb(+0%+0 +0 )").is_err());
    assert!(Rgba::from_str("rgb(+0%+0 +0%)").is_err());
    assert!(Rgba::from_str("rgb(+0%+0%+0 )").is_err());
    assert!(Rgba::from_str("rgb(+0%+0%+0%)").is_ok());

    assert!(Rgba::from_str("rgb(-0 -0 -0 )").is_ok());
    assert!(Rgba::from_str("rgb(-0 -0 -0%)").is_err());
    assert!(Rgba::from_str("rgb(-0 -0%-0 )").is_err());
    assert!(Rgba::from_str("rgb(-0 -0%-0%)").is_err());
    assert!(Rgba::from_str("rgb(-0%-0 -0 )").is_err());
    assert!(Rgba::from_str("rgb(-0%-0 -0%)").is_err());
    assert!(Rgba::from_str("rgb(-0%-0%-0 )").is_err());
    assert!(Rgba::from_str("rgb(-0%-0%-0%)").is_ok());

    assert!(Rgba::from_str("rgb(0 ,0 ,0 )").is_ok());
    assert!(Rgba::from_str("rgb(0 ,0 ,0%)").is_err());
    assert!(Rgba::from_str("rgb(0 ,0%,0 )").is_err());
    assert!(Rgba::from_str("rgb(0 ,0%,0%)").is_err());
    assert!(Rgba::from_str("rgb(0%,0 ,0 )").is_err());
    assert!(Rgba::from_str("rgb(0%,0 ,0%)").is_err());
    assert!(Rgba::from_str("rgb(0%,0%,0 )").is_err());
    assert!(Rgba::from_str("rgb(0%,0%,0%)").is_ok());

    assert!(Rgba::from_str("rgb( 0%   0%   0% )").is_ok());
    assert!(Rgba::from_str("rgb( 0%   0%   0% / 0% )").is_ok());
    assert!(Rgba::from_str("rgb( 0% , 0% , 0% )").is_ok());
    assert!(Rgba::from_str("rgb( 0% , 0% , 0% , 0% )").is_ok());
}

#[test]
fn hsl() {
    let red = Rgba::new(1., 0., 0., 0.);
    assert_eq!(red, Rgba::from_str("hsl(0 100% 50% / 0)").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(0deg 100% 50% / 0)").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(0grad 100% 50% / 0)").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(0rad 100% 50% / 0)").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(0turn 100% 50% / 0)").unwrap());

    assert_eq!(red, Rgba::from_str("hsl(360 100% 50% / 0)").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(360deg 100% 50% / 0)").unwrap());
    assert_eq!(red, Rgba::from_str("hsl(400grad 100% 50% / 0)").unwrap());
    assert_eq!(
        red,
        Rgba::from_str("hsl(6.283185307179586rad 100% 50% / 0)").unwrap()
    );
    assert_eq!(red, Rgba::from_str("hsl(1turn 100% 50% / 0)").unwrap());

    let green = Rgba::new(0., 1., 0., 0.);
    assert_eq!(green, Rgba::from_str("hsl(120 100% 50% / 0)").unwrap());
    assert_eq!(green, Rgba::from_str("hsl(120deg 100% 50% / 0)").unwrap());
    assert_eq!(
        green,
        Rgba::from_str("hsl(2.0943951023931953rad 100% 50% / 0)").unwrap()
    );
    assert_eq!(
        green,
        Rgba::from_str("hsl(0.3333333333333333turn 100% 50% / 0)").unwrap()
    );

    let blue = Rgba::new(0., 0., 1., 0.);
    assert_eq!(blue, Rgba::from_str("hsl(240 100% 50% / 0)").unwrap());
    assert_eq!(blue, Rgba::from_str("hsl(240deg 100% 50% / 0)").unwrap());
    assert_eq!(
        blue,
        Rgba::from_str("hsl(4.1887902047863905rad 100% 50% / 0)").unwrap()
    );
    assert_eq!(
        blue,
        Rgba::from_str("hsl(0.6666666666666666turn 100% 50% / 0)").unwrap()
    );

    assert!(Rgba::from_str("hsl(0 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0 0% 0% / 0%)").is_ok());
    assert!(Rgba::from_str("HSL(0 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0 0% 0% , 0)").is_err());
    assert!(Rgba::from_str("hsl(0,0%,0%,0)").is_ok());
    assert!(Rgba::from_str("hsl(0,0%,0%,0%)").is_ok());
    assert!(Rgba::from_str("hsl(0, 0%, 0%, 0)").is_ok());
    assert!(Rgba::from_str("hsl(0, 0%, 0%, 0%)").is_ok());

    assert!(Rgba::from_str(" hsl(0 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl (0 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0 0% 0% / 0) ").is_err());

    assert!(Rgba::from_str("hsl( 0   0%   0% )").is_ok());
    assert!(Rgba::from_str("hsl( 0   0%   0% / 0% )").is_ok());
    assert!(Rgba::from_str("hsl( 0 , 0% , 0% )").is_ok());
    assert!(Rgba::from_str("hsl( 0 , 0% , 0% , 0% )").is_ok());

    assert!(Rgba::from_str("hsla(0 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("HSLA(0 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsla(0 0% 0%)").is_ok());
    assert!(Rgba::from_str("hsla(0,0%,0%)").is_ok());
    assert!(Rgba::from_str("hsla(0,0%,0%,0)").is_ok());

    assert!(Rgba::from_str("hsla(0deg 0% 0%)").is_ok());
    assert!(Rgba::from_str("hsla(0deg 0% 0% / 0)").is_ok());

    assert!(Rgba::from_str("hsl(+0 +0 +0 )").is_err());
    assert!(Rgba::from_str("hsl(+0 +0 +0%)").is_err());
    assert!(Rgba::from_str("hsl(+0 +0%+0 )").is_err());
    assert!(Rgba::from_str("hsl(+0 +0%+0%)").is_ok());
    assert!(Rgba::from_str("hsl(+0%+0 +0 )").is_err());
    assert!(Rgba::from_str("hsl(+0%+0 +0%)").is_err());
    assert!(Rgba::from_str("hsl(+0%+0%+0 )").is_err());
    assert!(Rgba::from_str("hsl(+0%+0%+0%)").is_err());

    assert!(Rgba::from_str("hsl(-0 -0 -0 )").is_err());
    assert!(Rgba::from_str("hsl(-0 -0 -0%)").is_err());
    assert!(Rgba::from_str("hsl(-0 -0%-0 )").is_err());
    assert!(Rgba::from_str("hsl(-0 -0%-0%)").is_ok());
    assert!(Rgba::from_str("hsl(-0%-0 -0 )").is_err());
    assert!(Rgba::from_str("hsl(-0%-0 -0%)").is_err());
    assert!(Rgba::from_str("hsl(-0%-0%-0 )").is_err());
    assert!(Rgba::from_str("hsl(-0%-0%-0%)").is_err());

    assert!(Rgba::from_str("hsl(0 ,0 ,0 )").is_err());
    assert!(Rgba::from_str("hsl(0 ,0 ,0%)").is_err());
    assert!(Rgba::from_str("hsl(0 ,0%,0 )").is_err());
    assert!(Rgba::from_str("hsl(0 ,0%,0%)").is_ok());
    assert!(Rgba::from_str("hsl(0%,0 ,0 )").is_err());
    assert!(Rgba::from_str("hsl(0%,0 ,0%)").is_err());
    assert!(Rgba::from_str("hsl(0%,0%,0 )").is_err());
    assert!(Rgba::from_str("hsl(0%,0%,0%)").is_err());

    assert!(Rgba::from_str("hsl(0 0% 0%/0%)").is_ok());
    assert!(Rgba::from_str("hsl(0+0%+0%/+0%)").is_ok());
    assert!(Rgba::from_str("hsl(0-0%-0%/-0%)").is_ok());
    assert!(Rgba::from_str("hsl(0,0%,0%,0%)").is_ok());

    assert!(Rgba::from_str("hsl(0deg 0% 0%)").is_ok());
    assert!(Rgba::from_str("hsl(0deg+0%+0%)").is_ok());
    assert!(Rgba::from_str("hsl(0deg-0%-0%)").is_err());
    assert!(Rgba::from_str("hsl(0deg,0%,0%)").is_ok());

    assert!(Rgba::from_str("hsl(0deg 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0DEG 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0de 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0degg 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0 deg 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0grad 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0GRAD 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0gra 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0gradd 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0 grad 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0rad 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0RAD 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0ra 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0radd 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0 rad 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0turn 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0TURN 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0tur 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0turnn 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0 turn 0% 0% / 0)").is_err());

    assert!(Rgba::from_str("hsl(0deg\0 0% 0% / 0)").is_err());
    assert!(Rgba::from_str("hsl(0deg\t 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0deg\n 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0deg\r 0% 0% / 0)").is_ok());
    assert!(Rgba::from_str("hsl(0deg\x0C 0% 0% / 0)").is_ok());

    assert!(Rgba::from_str("hsl()").is_err());
    assert!(Rgba::from_str("hsl(0)").is_err());
    assert!(Rgba::from_str("hsl(0,0%)").is_err());
    assert!(Rgba::from_str("hsl(0 0%)").is_err());
    assert!(Rgba::from_str("hsl(,,)").is_err());
    assert!(Rgba::from_str("hsl(,%,%)").is_err());
    assert!(Rgba::from_str("hsl(0,%,%)").is_err());
    assert!(Rgba::from_str("hsl(0 % %)").is_err());
    assert!(Rgba::from_str("hsl(0,0%,0%,)").is_err());
    assert!(Rgba::from_str("hsl(0,0%,0%,0").is_err());
    assert!(Rgba::from_str("hsl(0 0% 0% / 0").is_err());
}

#[test]
fn named() {
    for (name, color) in named_colors() {
        assert_eq!(color, Rgba::from_str(&name).unwrap());
        assert_eq!(color, Rgba::from_str(&name.to_uppercase()).unwrap());
        {
            let mut name = name.clone();
            name.pop();
            assert!(Rgba::from_str(&name).is_err());
        }
        {
            let mut name = name.clone();
            name.push('a');
            assert!(Rgba::from_str(&name).is_err());
        }
    }
}

#[test]
fn numeric() {
    assert!(Rgba::from_str("rgb(6 36 216 / 100%)").is_ok());
    assert!(Rgba::from_str("rgb(6. 36 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36. 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216. / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100.%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100%.)").is_err());

    assert!(Rgba::from_str("rgb(6.e0 36 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36.e0 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216.e0 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100.e0%)").is_err());

    assert!(Rgba::from_str("rgb(6e 36 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36e 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216e / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100e%)").is_err());

    assert!(Rgba::from_str("rgb(6e+ 36 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36e+ 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216e+ / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100e+%)").is_err());

    assert!(Rgba::from_str("rgb(6e- 36 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36e- 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216e- / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100e-%)").is_err());

    assert!(Rgba::from_str("rgb(6e+0. 36 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36e+0. 216 / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216e+0. / 100%)").is_err());
    assert!(Rgba::from_str("rgb(6 36 216 / 100e+0.%)").is_err());

    assert_eq!(
        Rgba {
            red: 6. / 255.,
            green: 0.36 / 255.,
            blue: 216. / 255.,
            alpha: 1.0
        },
        Rgba::from_str("rgb(6e+0.36+216/100%)").unwrap()
    );

    let nines = "9".repeat(999);
    assert_eq!(
        Rgba {
            red: 100. / 255.,
            green: 100. / 255.,
            blue: 100. / 255.,
            alpha: 0.5445618932859895,
        },
        Rgba::from_str(&format!(
            "rgb({}e-997 {}e-997 {}e-997 / 5445618932859895362967233318697132813618813095743952975439298223406969961560047552942717636670910728746893019786283454139917900193169748259349067524939840552682198095012176093045431437495773903922425632551857520884625114624126588173520906670968542074438852601438992904761759703022688483745081090292688986958251711580854575674815074162979705098246243690189880319928315307816832576838178256307401454285988871020923752587330172447966674453785790265533466496640456213871241930958703059911787722565044368663670643970181259143319016472430928902201239474588139233890135329130660705762320235358869874608541509790266400643191187286648422874774910682648288516244021893172769161449825765517353755844373640588822904791244190695299838293263075467057383813882521706545084301049855505888186560731e-781)",
            nines, nines, nines,
        ))
        .unwrap()
    );

    assert!(Rgba::from_str("rgb(inf 0 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 inf 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 inf / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 0 / inf)").is_err());

    assert!(Rgba::from_str("rgb(INF 0 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 INF 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 INF / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 0 / INF)").is_err());

    assert!(Rgba::from_str("rgb(infinity 0 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 infinity 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 infinity / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 0 / infinity)").is_err());

    assert!(Rgba::from_str("rgb(INFINITY 0 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 INFINITY 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 INFINITY / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 0 / INFINITY)").is_err());

    assert!(Rgba::from_str("rgb(nan 0 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 nan 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 nan / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 0 / nan)").is_err());

    assert!(Rgba::from_str("rgb(NAN 0 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 NAN 0 / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 NAN / 0)").is_err());
    assert!(Rgba::from_str("rgb(0 0 0 / NAN)").is_err());
}

#[test]
fn overflow() {
    // https://github.com/w3c/web-platform-tests/blob/master/2dcontext/fill-and-stroke-styles/2d.fillStyle.parse.rgb-clamp-3.html
    assert_eq!(
        Rgba {
            red: 0.,
            green: 1.,
            blue: 0.,
            alpha: 1.,
        },
        Rgba::from_str("rgb(-2147483649, 4294967298, -18446744073709551619)").unwrap()
    );

    let nines = "9".repeat(999);
    assert_eq!(
        Rgba {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 1.
        },
        Rgba::from_str(&format!("rgb({}, {}, {}, {})", nines, nines, nines, nines)).unwrap()
    );
    assert_eq!(
        Rgba {
            red: 1.,
            green: 1.,
            blue: 1.,
            alpha: 1.
        },
        Rgba::from_str("rgb(1e+999 1e+999 1e+999 / 1e+999)").unwrap()
    );
    assert_eq!(
        Rgba {
            red: 100. / 255.,
            green: 100. / 255.,
            blue: 100. / 255.,
            alpha: 0.5445618932859895,
        },
        Rgba::from_str(&format!(
            "rgb({}e-997 {}e-997 {}e-997 / 5445618932859895362967233318697132813618813095743952975439298223406969961560047552942717636670910728746893019786283454139917900193169748259349067524939840552682198095012176093045431437495773903922425632551857520884625114624126588173520906670968542074438852601438992904761759703022688483745081090292688986958251711580854575674815074162979705098246243690189880319928315307816832576838178256307401454285988871020923752587330172447966674453785790265533466496640456213871241930958703059911787722565044368663670643970181259143319016472430928902201239474588139233890135329130660705762320235358869874608541509790266400643191187286648422874774910682648288516244021893172769161449825765517353755844373640588822904791244190695299838293263075467057383813882521706545084301049855505888186560731e-781)",
            nines, nines, nines,
        ))
        .unwrap()
    );
}

#[test]
fn css_parsing_tests_color3() {
    run_color_test(include_str!("tests/css-parsing-tests/color3.json"));
}

#[test]
fn css_parsing_tests_color3_hsl() {
    run_color_test(include_str!("tests/css-parsing-tests/color3_hsl.json"));
}

#[test]
fn css_parsing_tests_color3_keywords() {
    run_color_test(include_str!("tests/css-parsing-tests/color3_keywords.json"));
}

#[cfg(feature = "bench")]
#[bench]
fn bench_hash(b: &mut test::Bencher) {
    let input = "#00000000";
    b.iter(|| {
        let result = Rgba::from_str(test::black_box(&input));
        let _ = test::black_box(result);
    })
}

#[cfg(feature = "bench")]
#[bench]
fn bench_rgb(b: &mut test::Bencher) {
    let input = "rgb(100% 100% 100% / 1.0)";
    b.iter(|| {
        let result = Rgba::from_str(test::black_box(&input));
        let _ = test::black_box(result);
    })
}

#[cfg(feature = "bench")]
#[bench]
fn bench_rgb_exp(b: &mut test::Bencher) {
    let input = "rgb(1.0e2% 1.0e2% 1.0e2% / 1.0e0)";
    b.iter(|| {
        let result = Rgba::from_str(test::black_box(&input));
        let _ = test::black_box(result);
    })
}

#[cfg(feature = "bench")]
#[bench]
fn bench_hsl(b: &mut test::Bencher) {
    let input = "hsl(0deg 100% 50% / 1.0)";
    b.iter(|| {
        let result = Rgba::from_str(test::black_box(&input));
        let _ = test::black_box(result);
    })
}

#[cfg(feature = "bench")]
#[bench]
fn bench_named(b: &mut test::Bencher) {
    let input = "rebeccapurple";
    b.iter(|| {
        let result = Rgba::from_str(test::black_box(&input));
        let _ = test::black_box(result);
    })
}

#[cfg(feature = "bench")]
#[bench]
fn bench_named_all(b: &mut test::Bencher) {
    let named = named_colors();
    b.iter(|| {
        for (name, _) in &named {
            let result = Rgba::from_str(&name);
            let _ = test::black_box(result);
        }
    })
}

fn color_f32_to_u8(value: f32) -> u8 {
    (value * 255.).round().max(0.).min(255.) as u8
}

fn named_colors() -> Vec<(String, Rgba)> {
    let named_colors: Vec<serde_json::Value> =
        serde_json::from_str(include_str!("tests/named_colors.json")).unwrap();
    let mut result: Vec<_> = named_colors
        .chunks(2)
        .map(|named| {
            (
                named[0].as_str().unwrap().into(),
                Rgba::from_rgb8(
                    named[1][0].as_u64().unwrap() as u8,
                    named[1][1].as_u64().unwrap() as u8,
                    named[1][2].as_u64().unwrap() as u8,
                ),
            )
        })
        .collect();
    result.push(("transparent".to_string(), Rgba::from_rgba8(0, 0, 0, 0)));
    result
}

fn run_color_test(json: &str) {
    let json: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
    for (input, expected) in json
        .chunks(2)
        .map(|named| (named[0].as_str().unwrap(), &named[1]))
    {
        // Don't accept CSS comments or escapes, leading or trailing whitespace, or "currentcolor".
        if input.contains("/*")
            || input.contains("\\")
            || input != input.trim_matches(|c: char| c.is_ascii() && super::is_whitespace(c as u8))
            || input.eq_ignore_ascii_case("currentcolor")
        {
            assert!(Rgba::from_str(input).is_err());
            continue;
        }
        match expected {
            serde_json::Value::Null => {
                assert!(Rgba::from_str(input).is_err());
            }
            serde_json::Value::Array(components) => {
                let color = Rgba::from_str(input).unwrap();
                let components = (
                    components[0].as_u64().unwrap() as u8,
                    components[1].as_u64().unwrap() as u8,
                    components[2].as_u64().unwrap() as u8,
                    components[3].as_u64().unwrap() as u8,
                );
                assert_eq!(components.0, color_f32_to_u8(color.red));
                assert_eq!(components.1, color_f32_to_u8(color.green));
                assert_eq!(components.2, color_f32_to_u8(color.blue));
                assert_eq!(components.3, color_f32_to_u8(color.alpha));
            }
            _ => {
                panic!("Bad test case");
            }
        }
    }
}
