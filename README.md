# colorhash256 [![crates.io](https://img.shields.io/crates/v/colorhash256.svg)](https://crates.io/crates/colorhash256) [![Build Status](https://img.shields.io/travis/myfreeweb/colorhash256.svg?style=flat)](https://travis-ci.org/myfreeweb/colorhash256) [![API Docs](https://img.shields.io/badge/api-docs-yellow.svg?style=flat)](https://myfreeweb.github.io/autodocs/colorhash256/colorhash256) [![unlicense](https://img.shields.io/badge/un-license-green.svg?style=flat)](http://unlicense.org)

A [Rust] library that's like [Chroma-Hash], but with ANSI terminal colors.

[Rust]: https://www.rust-lang.org
[Chroma-Hash]: https://github.com/mattt/Chroma-Hash/

## Usage

```rust
extern crate colorhash256;

let ansi = ::colorhash256::hash_as_ansi(b"Correct Horse Battery Staple");
let rgb = ::colorhash256::hash_as_rgb(b"Correct Horse Battery Staple");
```

## Contributing

Please feel free to submit pull requests!
Bugfixes and simple non-breaking improvements will be accepted without any questions :-)

By participating in this project you agree to follow the [Contributor Code of Conduct](http://contributor-covenant.org/version/1/2/0/).

[The list of contributors is available on GitHub](https://github.com/myfreeweb/colorhash256/graphs/contributors).

## License

This is free and unencumbered software released into the public domain.  
For more information, please refer to the `UNLICENSE` file or [unlicense.org](http://unlicense.org).
