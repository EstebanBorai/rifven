<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667" height="120" width="120" />
  </div>
  <h1 align="center">rifven</h1>
  <h4 align="center">
    Venezuelan RIF implementation useful for creating and validating RIF numbers
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/rifven.svg)](https://crates.io/crates/rifven)
  [![Documentation](https://docs.rs/rifven/badge.svg)](https://docs.rs/rifven)
  ![Build](https://github.com/EstebanBorai/rifven/workflows/build/badge.svg)
  ![Lint](https://github.com/EstebanBorai/rifven/workflows/clippy/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/rifven/workflows/tests/badge.svg)

</div>

## What are RIF numbers?

RIF (Registro de Informacion Fiscal) in english _Fiscal Information Registry_ is a number
provided by a Venezuelan entity SAIME used to identify multiple entities for taxable purposes.

The RIF is composed by a kind which could be:

- C: Township or Communal Council
- E: Represents a foreigner natural person and stands for
"Extranjera" and "Extranjero"
- G: Represents a goverment entity and stands for
"Gubernamental"
- J: Used for a legal entity. Could be a natural person
or a corporate entity and stands for "Jurídico"
- P: Used on RIF numbers which belongs to passports
- V: Represents a person with venezuelan citizenship and stands
for "Venezolana" and "Venezolano"

An identifier number followed by a hyphen symbol and finally a checksum digit, as well followed
by a hyphen symbol.

<div align="center">
  <img src="https://raw.githubusercontent.com/EstebanBorai/rifven/main/docs/rif_parts.png" />
</div>

## Motivation

Implement a crate to help create instances of valid RIF numbers

## Usage

Creating a new `Rif` instance providing each of its parts values
such as `Kind` (J; V; P; G; C), identifier (tax payer ID), check number.

The following code, creates an instance of `Rif` for a RIF string which
looks like `J-07013380-5`:

```rust
use rifven::{Kind, Rif};

let rif = Rif::new(Kind::Legal, 07013380, 5).unwrap();

assert_eq!(rif.kind(), Kind::Legal);
assert_eq!(rif.identifier(), 7013380);
assert_eq!(rif.checksum_digit(), 5);
```

You can also create instances of `Rif` from its string representation

```rust
use rifven::{Kind, Rif};
use std::str::FromStr;

let myrif = Rif::from_str("J-07013380-5").unwrap();

assert_eq!(Rif::new(Kind::Legal, 07013380, 5).unwrap(), myrif);
```

## License

Licensed under the Apache 2.0 and MIT licenses

## Contributions

Every contribution to this project is welcome, feel free to either open a Pull Request
or issue
