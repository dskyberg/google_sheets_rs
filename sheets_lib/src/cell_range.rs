use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Error;

const MAX_COLS: u32 = 18_278;

/// Format an A1 column based on an index.
/// Obviously, this isn't really base26.
///
#[inline]
fn col_size_guard(col: u32) -> Result<u32> {
    if col > MAX_COLS {
        return Err(Error::A1ColIndex.into());
    }
    Ok(col)
}

#[allow(dead_code)]
fn from_a1_col(a1: &str) -> u32 {
    let a1 = a1.to_uppercase();
    let a1_bytes = a1.as_bytes();

    let mut result = 0;
    for (i, base) in a1_bytes.iter().enumerate() {
        let exp = a1_bytes.len() - i - 1;
        if exp > 0 {
            result += 26u32.pow(exp as u32) * (*base as u32 - 64);
        } else {
            result += *base as u32 - 64;
        }
    }
    result
}

pub(super) fn is_a1(value: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z]+[1-9][0-9]*$").unwrap();
    re.is_match(value)
}

pub(super) fn is_r1c1(value: &str) -> bool {
    let re = Regex::new(r"^[Rr][1-9][0-9]*[Cc][1-9][0-9]*$").unwrap();
    re.is_match(value)
}

#[allow(dead_code)]
pub(super) fn to_a1_col(col: u32) -> Result<String> {
    match col_size_guard(col)? <= 26 {
        true => Ok(format!(
            "{}",
            char::from_u32(64 + col).ok_or(Error::A1ColIndex)?
        )),
        false => match col % 26 == 0 {
            // If the modulo == 0,
            true => Ok(format!("{}Z", to_a1_col((col - 1) / 26)?)),
            false => Ok(format!(
                "{}{}",
                to_a1_col((col - 1) / 26)?,
                to_a1_col(col % 26)?
            )),
        },
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CellRange {
    pub row: u32,
    pub col: u32,
}

impl CellRange {
    pub fn from_row_col(row: u32, col: u32) -> Result<Self> {
        Ok(Self { row, col })
    }

    pub fn from_a1(a1: &str) -> Result<Self> {
        let re = Regex::new(r"^(?<col>[a-zA-Z]+)(?<row>[0-9]+)$").unwrap();
        let Some(caps) = re.captures(a1) else {
            return Err(Error::A1Address(a1.to_string()).into());
        };

        Ok(Self {
            row: caps["row"].parse::<u32>()?,
            col: from_a1_col(&caps["col"]),
        })
    }

    /// Create from a valid R1C1 address
    pub fn from_r1c1(r1c1: &str) -> Result<Self> {
        let re = Regex::new(r"^[Rr](?<row>[1-9][0-9]*)[Cc](?<col>[1-9][0-9]*)$").unwrap();
        let Some(caps) = re.captures(r1c1) else {
            return Err(Error::R1C1Address(r1c1.to_string()).into());
        };
        Ok(Self {
            row: caps["row"].parse::<u32>()?,
            col: caps["col"].parse::<u32>()?,
        })
    }

    pub fn to_a1(&self) -> String {
        format!("{}{}", to_a1_col(self.col).unwrap(), self.row)
    }
}

impl TryFrom<&str> for CellRange {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        if is_r1c1(value) {
            Self::from_r1c1(value)
        } else if is_a1(value) {
            Self::from_a1(value)
        } else {
            Err(Error::BadCellRange(value.to_string()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_r1c1() {
        assert!(is_r1c1("R1C1"));
        assert!(is_r1c1("r1c1"));
        assert!(!is_r1c1("R1C"));
    }

    #[test]
    fn test_is_a1() {
        assert!(is_a1("A1"));
        assert!(is_a1("A12"));
        assert!(is_a1("AZ1"));
        assert!(is_a1("AZ1"));
        assert!(!is_a1("A1:A2"));
        assert!(!is_a1("R1C1"));
    }

    #[test]
    fn test_cell_range_from_a1() {
        let cell = CellRange::from_a1("a1").expect("oops");
        assert_eq!(cell, CellRange { row: 1, col: 1 });
    }

    #[test]
    fn test_name() {
        assert_eq!(to_a1_col(26).expect("oops"), "Z");
        assert_eq!(to_a1_col(27).expect("oops"), "AA");
        assert_eq!(to_a1_col(52).expect("oops"), "AZ");
        assert_eq!(to_a1_col(53).expect("oops"), "BA");
        assert_eq!(to_a1_col(702).expect("oops"), "ZZ");
        assert_eq!(to_a1_col(703).expect("oops"), "AAA");
    }

    #[test]
    fn test_from_a1() {
        assert_eq!(from_a1_col("z"), 26);
        assert_eq!(from_a1_col("aa"), 27);
        assert_eq!(from_a1_col("az"), 52);
        assert_eq!(from_a1_col("ba"), 53);
        assert_eq!(from_a1_col("ba"), 53);
        assert_eq!(from_a1_col("Zz"), 702);
        assert_eq!(from_a1_col("aAa"), 703);
    }
}
