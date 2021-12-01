# css-color

Parse color strings from [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/). All legacy sRGB forms are supported:
* RGB hexadecimal notation
* Named colors including `transparent`
* `rgb()` and `rgba()` functions
* `hsl()` and `hsla()` functions
* `hwb()` function

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
css-color = "0.2.4"
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
