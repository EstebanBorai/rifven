use std::ops::IndexMut;
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::kind::Kind;

/// Length of a RIF identifier
const RIF_IDENTIFIER_LENGTH: usize = 9;

/// Venezuelan RIF identifier
///
/// # Anatomy
/// The following chart reviews the anatomy of a
/// RIF identifier.
///
///```ignore
///
/// X - XXXXXXXXX - X
/// ^   ^________   ^
/// A       B      C
///
/// ```
///
/// A: `Kind` represents either the citizenship or the
/// kind of entity behind the RIF identifier
///
/// B: The contributor/tax payer identifier. Must be 9
/// digits long
///
/// C: The check number for the RIF identifier
#[derive(Clone, Debug, PartialEq)]
pub struct Rif {
    pub(crate) checksum_digit: u8,
    pub(crate) identifier: u32,
    pub(crate) kind: Kind,
}

impl Rif {
    /// Creates a new `Rif` instance with the provided `kind`, `identifier`
    /// and the `checksum_digit`
    ///
    /// ```rust
    /// use rifven::{Kind, Rif};
    ///
    /// let rif = Rif::new(Kind::Legal, 07013380, 5).unwrap();
    ///
    /// assert_eq!(rif.kind(), Kind::Legal);
    /// assert_eq!(rif.identifier(), 7013380);
    /// assert_eq!(rif.checksum_digit(), 5);
    /// ```
    pub fn new(kind: Kind, identifier: u32, checksum_digit: u8) -> Result<Self> {
        let expected_checksum_digit = Rif::calc_checksum_digit(&kind, identifier);

        if expected_checksum_digit == checksum_digit {
            return Ok(Rif {
                checksum_digit,
                identifier,
                kind,
            });
        }

        Err(Error::UnexpectedCheckNum(
            expected_checksum_digit,
            checksum_digit,
        ))
    }

    /// Retrieves the `Kind` of the current `Rif`
    ///
    /// ```rust
    /// use rifven::{Kind, Rif};
    ///
    /// let rif = Rif::new(Kind::Legal, 07013380, 5).unwrap();
    ///
    /// assert_eq!(rif.kind(), Kind::Legal);
    /// ```
    pub fn kind(&self) -> Kind {
        self.kind.clone()
    }

    /// Retrieves the identifier of the current `Rif`
    ///
    /// ```rust
    /// use rifven::{Kind, Rif};
    ///
    /// let rif = Rif::new(Kind::Legal, 07013380, 5).unwrap();
    ///
    /// assert_eq!(rif.identifier(), 7013380);
    /// ```
    pub fn identifier(&self) -> u32 {
        self.identifier
    }

    /// Retrieves the check number of the current `Rif`
    ///
    /// ```rust
    /// use rifven::{Kind, Rif};
    ///
    /// let rif = Rif::new(Kind::Legal, 07013380, 5).unwrap();
    ///
    /// assert_eq!(rif.checksum_digit(), 5);
    /// ```
    pub fn checksum_digit(&self) -> u8 {
        self.checksum_digit
    }

    /// Validates the RIF provided as a string slice (`&str`).
    /// Returning a `Result` where the `Ok` variant contains a
    /// `Rif` instance built from the `rif` string slice and the
    /// `Err` variant contains the `Error` (`rifven::error::Error`)
    /// instance created during the validation
    ///
    /// The RIF provided must be separated using a hyphen (`-`) as
    /// follows `J-40512535-7` otherwise this function will return
    /// an `Err` variant containing the `Error::InvalidRif` error
    fn validate(rif: &str) -> Result<Rif> {
        let parts: Vec<&str> = rif.split("-").collect();

        if parts.len() != 3 {
            return Err(Error::InvalidRif(format!("RIF must be splitted into 3 parts separated by dashes. Eg. J-123456789-1. Provided {}", rif)));
        }

        let checksum_digit = parts
            .get(2)
            .unwrap()
            .parse::<u8>()
            .map_err(|_| Error::InvalidCheckNum(parts.get(2).unwrap().to_string()))?;
        let kind = Kind::from_str(parts.get(0).unwrap())?;
        let identifier = {
            let identifier = parts.get(1).unwrap();

            identifier
                .parse::<u32>()
                .map_err(|e| Error::InvalidRifIdentifier(e.to_string()))?
        };
        let expected_checksum_digit = Rif::calc_checksum_digit(&kind, identifier);

        if expected_checksum_digit != checksum_digit {
            return Err(Error::UnexpectedCheckNum(
                expected_checksum_digit,
                checksum_digit,
            ));
        }

        Ok(Rif {
            checksum_digit,
            identifier,
            kind,
        })
    }

    /// Calculates the **checksum_digit** for a provided RIF number
    pub fn calc_checksum_digit(kind: &Kind, identifier: u32) -> u8 {
        let mut digits: Vec<u32> = vec![0; RIF_IDENTIFIER_LENGTH];
        let mut sum_values: Vec<u32> = vec![0; RIF_IDENTIFIER_LENGTH];
        let mut identifier = identifier;

        for idx in 1..=RIF_IDENTIFIER_LENGTH {
            *digits.index_mut(RIF_IDENTIFIER_LENGTH - idx) = identifier % 10;
            identifier /= 10;
        }

        for (idx, digit) in digits.into_iter().enumerate() {
            let current_value = sum_values.index_mut(idx);

            match idx {
                0 => *current_value = kind.checksum_digit() * 4,
                1 | 7 => *current_value = digit * 3,
                2 | 8 => *current_value = digit * 2,
                3 => *current_value = digit * 7,
                4 => *current_value = digit * 6,
                5 => *current_value = digit * 5,
                6 => *current_value = digit * 4,
                _ => {}
            }
        }

        let sum_values_total: u32 = sum_values.iter().sum();
        let validator = sum_values_total / 11;
        let reminder = sum_values_total - (validator * 11);
        let checksum_digit = 11 - reminder;

        if checksum_digit > 9 {
            return 0;
        }

        checksum_digit as u8
    }
}

impl FromStr for Rif {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Rif::validate(s)?)
    }
}
