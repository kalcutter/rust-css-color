# css-color

[![crates.io-badge]][crates.io] [![docs-badge]][docs.rs] [![license-badge]][license]

[crates.io-badge]: https://img.shields.io/crates/v/css-color
[crates.io]: https://crates.io/crates/css-color
[docs-badge]: https://img.shields.io/docsrs/css-color
[docs.rs]: https://docs.rs/css-color
[license-badge]: https://img.shields.io/crates/l/css-color

Parse color strings from [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/). All legacy sRGB forms are supported:
* [RGB hexadecimal notations][hex].
* [`rgb()`][rgb] and [`rgba()`][rgb] functions.
* [`hsl()`][hsl] and [`hsla()`][hsl] functions.
* [`hwb()`][hwb] function.
* [Named colors][named-colors] including the [`transparent`][transparent] keyword.

The [`none`][none] keyword is supported. "Missing" color components behave identically to zero.

[hex]: https://www.w3.org/TR/css-color-4/#hex-notation
[rgb]: https://www.w3.org/TR/css-color-4/#rgb-functions
[hsl]: https://www.w3.org/TR/css-color-4/#the-hsl-notation
[hwb]: https://www.w3.org/TR/css-color-4/#the-hwb-notation
[named-colors]: https://www.w3.org/TR/css-color-4/#named-colors
[transparent]: https://www.w3.org/TR/css-color-4/#transparent-color
[none]: https://www.w3.org/TR/css-color-4/#missing

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
css-color = "0.2.8"
```

### Example

```rust
use css_color::Srgb;

fn main() {
    let lime = Srgb::new(0., 1., 0., 1.);
    assert_eq!(lime, "#0f0".parse().unwrap());
    assert_eq!(lime, "rgb(0 255 0)".parse().unwrap());
    assert_eq!(lime, "rgb(0% 100% 0%)".parse().unwrap());
    assert_eq!(lime, "hsl(120deg 100% 50% / 100%)".parse().unwrap());
    assert_eq!(lime, "hwb(120 0% 0% / 1)".parse().unwrap());
    assert_eq!(lime, "lime".parse().unwrap());
}
```

## Supported Rust Versions

The minimum supported Rust version is 1.55. Earlier versions may compile but parsing can reject certain valid numeric values.

## License

[license]: #license

This repository is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
