use std::str::FromStr;
use std::string::ToString;

use crate::error::Error;

/// The RIF kind represents the kind of entity this RIF
/// belongs to.
///
/// Possible values are:
///
/// - C: Township or Communal Council
/// - E: Represents a foreigner natural person and stands for
/// "Extranjera" and "Extranjero"
/// - G: Represents a goverment entity and stands for
/// "Gubernamental"
/// - J: Used for a legal entity. Could be a natural person
/// or a corporate entity and stands for "Jurídico"
/// - P: Used on RIF numbers which belongs to passports
/// - V: Represents a person with venezuelan citizenship and stands
/// for "Venezolana" and "Venezolano"
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    /// E: Foreigner
    Foreigner,
    /// G: Government Entity
    Government,
    /// J: Legal Entity
    Legal,
    /// P: Passport
    Passport,
    /// C: Township or Communal Council
    Township,
    /// V: Venezuelan
    Venezuelan,
}

impl Kind {
    /// Retrieves the checksum digit for the current RIF kind
    pub fn checksum_digit(&self) -> u32 {
        match self {
            Kind::Venezuelan => 1,
            Kind::Foreigner => 2,
            Kind::Legal | Kind::Township => 3,
            Kind::Passport => 4,
            Kind::Government => 5,
        }
    }
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Kind::Township),
            "e" => Ok(Kind::Foreigner),
            "g" => Ok(Kind::Government),
            "j" => Ok(Kind::Legal),
            "p" => Ok(Kind::Passport),
            "v" => Ok(Kind::Venezuelan),
            _ => Err(Error::InvalidRifKind(s.to_string())),
        }
    }
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Kind::Foreigner => String::from("E"),
            Kind::Government => String::from("G"),
            Kind::Legal => String::from("J"),
            Kind::Passport => String::from("P"),
            Kind::Township => String::from("C"),
            Kind::Venezuelan => String::from("V"),
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn creates_a_kind_instance_from_str() {
        let kinds = vec![
            Kind::Foreigner,
            Kind::Government,
            Kind::Legal,
            Kind::Passport,
            Kind::Township,
            Kind::Venezuelan,
        ];

        let str_value = vec!["e", "g", "j", "p", "c", "v"];

        for (idx, kind_str) in str_value.into_iter().enumerate() {
            assert_eq!(kinds[idx], Kind::from_str(kind_str).unwrap());
        }
    }

    #[test]
    fn creates_a_string_from_a_kind() {
        let kinds = vec![
            Kind::Township,
            Kind::Foreigner,
            Kind::Government,
            Kind::Legal,
            Kind::Passport,
            Kind::Venezuelan,
        ];

        let string_value = vec![
            String::from("C"),
            String::from("E"),
            String::from("G"),
            String::from("J"),
            String::from("P"),
            String::from("V"),
        ];

        for (idx, kind) in kinds.into_iter().enumerate() {
            assert_eq!(string_value[idx].to_string(), kind.to_string());
        }
    }
}
