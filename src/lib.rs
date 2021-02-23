//! # rifve
//!
//! Venezuelan RIF implementation useful for creating and validating RIF numbers
//!
//! <div align="center">
//! 
//!   [![Crates.io](https://img.shields.io/crates/v/rifven.svg)](https://crates.io/crates/rifven)
//!   [![Documentation](https://docs.rs/rifven/badge.svg)](https://docs.rs/rifven)
//!   ![Build](https://github.com/EstebanBorai/rifven/workflows/build/badge.svg)
//!   ![Lint](https://github.com/EstebanBorai/rifven/workflows/clippy/fmt/badge.svg)
//!   ![Tests](https://github.com/EstebanBorai/rifven/workflows/tests/badge.svg)
//! 
//! </div>
//! 
//! ## What are RIF numbers?
//! 
//! RIF (Registro de Informacion Fiscal) in english _Fiscal Information Registry_ is a number
//! provided by a Venezuelan entity SAIME used to identify multiple entities for taxable purposes.
//! 
//! The RIF is composed by a kind which could be:
//! 
//! - C: Township or Communal Council
//! - E: Represents a foreigner natural person and stands for
//! "Extranjera" and "Extranjero"
//! - G: Represents a goverment entity and stands for
//! "Gubernamental"
//! - J: Used for a legal entity. Could be a natural person
//! or a corporate entity and stands for "Jurídico"
//! - P: Used on RIF numbers which belongs to passports
//! - V: Represents a person with venezuelan citizenship and stands
//! for "Venezolana" and "Venezolano"
//! 
//! An identifier number followed by a hyphen symbol and finally a checksum digit, as well followed
//! by a hyphen symbol.
//! 
//! <div align="center">
//!   <img src="https://raw.githubusercontent.com/EstebanBorai/rifven/main/docs/rif_parts.png" />
//! </div>
//! 
//! ## Motivation
//! 
//! Implement a crate to help create instances of valid RIF numbers
//! 
//! ## Usage
//! 
//! Creating a new `Rif` instance providing each of its parts values
//! such as `Kind` (J; V; P; G; C), identifier (tax payer ID), check number.
//!
//! The following code, creates an instance of `Rif` for a RIF string which
//! looks like `J-07013380-5`:
//!
//! ```
//! use rifven::{Kind, Rif};
//!
//! let rif = Rif::new(Kind::Legal, 07013380, 5).unwrap();
//!
//! assert_eq!(rif.kind(), Kind::Legal);
//! assert_eq!(rif.identifier(), 7013380);
//! assert_eq!(rif.checksum_digit(), 5);
//! ```
//!
//! You can also create instances of `Rif` from its string representation
//!
//! ```
//! use rifven::{Kind, Rif};
//! use std::str::FromStr;
//!
//! let myrif = Rif::from_str("J-07013380-5").unwrap();
//!
//! assert_eq!(Rif::new(Kind::Legal, 07013380, 5).unwrap(), myrif);
//! ```
//!
mod error;
mod kind;
mod rif;

pub use kind::*;
pub use rif::*;

#[allow(unused_imports)]
mod tests {
    use std::str::FromStr;

    use crate::error::*;
    use crate::kind::*;
    use crate::rif::*;

    #[test]
    fn calcs_the_checksum_digit() {
        let rif_identifier = vec![
            (Kind::Legal, 00019361),
            (Kind::Legal, 07013380),
            (Kind::Legal, 31286704),
            (Kind::Legal, 40512535),
            (Kind::Legal, 40119253),
            (Kind::Government, 20009997),
            (Kind::Government, 20000001),
            (Kind::Government, 20000002),
            (Kind::Government, 20000004),
            (Kind::Government, 20000044),
        ];

        let rif_checksum_digit = vec![4, 5, 3, 7, 0, 6, 5, 3, 0, 9];

        for (idx, (kind, identifier)) in rif_identifier.iter().enumerate() {
            assert_eq!(
                rif_checksum_digit[idx],
                Rif::calc_checksum_digit(kind, *identifier),
            );
        }
    }

    #[test]
    fn creates_rif_from_str() {
        let candidates = vec![
            Rif::new(Kind::Legal, 000019361, 4).unwrap(),
            Rif::new(Kind::Legal, 07013380, 5).unwrap(),
            Rif::new(Kind::Legal, 31286704, 3).unwrap(),
            Rif::new(Kind::Government, 20000044, 9).unwrap(),
            Rif::new(Kind::Government, 20000004, 0).unwrap(),
            Rif::new(Kind::Government, 20000002, 3).unwrap(),
        ];

        let expects = vec![
            Rif::from_str("J-00019361-4").unwrap(),
            Rif::from_str("J-07013380-5").unwrap(),
            Rif::from_str("J-31286704-3").unwrap(),
            Rif::from_str("G-20000044-9").unwrap(),
            Rif::from_str("G-20000004-0").unwrap(),
            Rif::from_str("G-20000002-3").unwrap(),
        ];

        for (idx, rif) in candidates.iter().enumerate() {
            assert_eq!(*rif, expects[idx]);
        }
    }

    #[test]
    fn checks_for_invalid_rifs_from_str() {
        let have = vec![
            Rif::from_str("J-00018461-4"),
            Rif::from_str("E-12312312-5"),
            Rif::from_str("M-00000001-3"),
            Rif::from_str("X-00029383-7"),
            Rif::from_str("V-AA348932-1"),
            Rif::from_str("G-X0000002-3"),
            Rif::from_str("G200000040"),
        ];

        let expected_error = vec![
          Error::UnexpectedCheckNum(5, 4),
          Error::UnexpectedCheckNum(6, 5),
          Error::InvalidRifKind(String::from("M")),
          Error::InvalidRifKind(String::from("X")),
          Error::InvalidRifIdentifier(String::from("invalid digit found in string")),
          Error::InvalidRifIdentifier(String::from("invalid digit found in string")),
          Error::InvalidRif(String::from("RIF must be splitted into 3 parts separated by dashes. Eg. J-123456789-1. Provided G200000040")),
      ];

        for (idx, rif) in have.into_iter().enumerate() {
            assert_eq!(rif.err().unwrap(), expected_error[idx]);
        }
    }
}
