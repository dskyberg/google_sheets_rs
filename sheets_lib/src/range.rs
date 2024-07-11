use std::fmt::Display;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::Error;

use super::{is_r1c1, to_a1_col, CellRange};

/// Can contain only letters, numbers, and underscores.
/// Can't start with a number, or the words "true" or "false."
/// Can't contain any spaces or punctuation.
/// Must be 1–250 characters.
/// Can't be in either A1 or R1C1 syntax. For example, you might get an error
/// if you give your range a name like "A1:B2" or "R1C1:R2C2."
fn is_valid_range_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 250 {
        return false;
    }
    if name.starts_with("true") || name.starts_with("false") {
        return false;
    }
    if is_r1c1(name) {
        return false;
    }

    let re = Regex::new(r"^[a-zA-Z][a-zA-Z_0-9]*$").unwrap();
    re.is_match(name)
}

/// A Range is defined as either an `<sheet name>!<A1 Start>:<A1 End>` or a named range
/// Where (for non named ranges):
/// - Sheet name is optional
/// - A1 End is optional, for instance for a single cell
/// - A1 can be a single column, referencing an entire column
/// - Start and end can be a single row, referencing an entire row
///
/// Examples:
/// - `MyNamedRange` - A named range
/// - `Sheet1!A1` - The single cell at row 1, col 1 in Sheet1
/// - `Sheet1!A1:C10` - The 10x3 grid from Sheet1 between row 1, col 1 and row 10, col 3
/// - `Sheet1!B` - The entire col 2 in Sheet1 as a 1xN grid (where N is the number of rows in the sheet)
/// - `Sheet1!A:B` - All cells in columns 1 and 2 as an Nx2 grid (where N is the number of rows in the sheet)
/// - `Sheet1!10` - The entire row 10 in Sheet1 as a 1xN grid (where N is the number of cols in the sheet)
/// - `Sheet1!1:3 - All cells in rows 1 - 3 as a 3xN grid (where N is the number of cols in the sheet)
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Range {
    name: Option<String>,
    sheet: Option<String>,
    start_row: Option<u32>,
    start_col: Option<u32>,
    end_row: Option<u32>,
    end_col: Option<u32>,
    use_a1_notation: bool,
}

impl Default for Range {
    fn default() -> Self {
        Self::new()
    }
}

impl Range {
    pub fn new() -> Self {
        Self {
            name: None,
            sheet: None,
            start_row: None,
            start_col: None,
            end_row: None,
            end_col: None,
            use_a1_notation: true,
        }
    }
    pub fn with_r1_c1_notation(mut self) -> Self {
        self.use_a1_notation = false;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_sheet(mut self, sheet: &str) -> Self {
        self.sheet = Some(sheet.to_string());
        self
    }

    pub fn with_start_cell(mut self, cell: CellRange) -> Self {
        self.start_row = Some(cell.row);
        self.start_col = Some(cell.col);
        self
    }

    pub fn with_end_cell(mut self, cell: CellRange) -> Self {
        self.end_row = Some(cell.row);
        self.end_col = Some(cell.col);
        self
    }

    pub fn with_start_row(mut self, row: u32) -> Self {
        self.start_row = Some(row);
        self
    }

    pub fn with_start_col(mut self, col: u32) -> Self {
        self.start_col = Some(col);
        self
    }

    pub fn with_end_row(mut self, row: u32) -> Self {
        self.end_row = Some(row);
        self
    }
    pub fn with_end_col(mut self, col: u32) -> Self {
        self.end_col = Some(col);
        self
    }
}

impl TryFrom<&str> for Range {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let Some((sheet, range)) = value.split_once('!') else {
            // Looks like a named range
            if !is_valid_range_name(value) {
                println!("Found a bad range name");
                return Err(Error::BadRange(value.to_string()).into());
            }
            return Ok(Self::new().with_name(value));
        };

        let mut result = Self::new().with_sheet(sheet);
        // The range may be a single part or 2 parts
        let parts = range.split(':').collect::<Vec<&str>>();
        let start = CellRange::try_from(parts[0]).map_err(Error::FromAnyhow)?;
        result = result.with_start_cell(start);
        if let Some(end_range) = parts.get(1) {
            let end = CellRange::try_from(*end_range).map_err(Error::FromAnyhow)?;
            result = result.with_end_cell(end);
        }
        Ok(result)
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        if let Some(sheet) = &self.sheet {
            result += &format!("{}!", sheet);
        }

        if let Some(start_col) = &self.start_col {
            result += &to_a1_col(*start_col).unwrap();
        }
        if let Some(start_row) = &self.start_row {
            result += &format!("{}", start_row);
        }

        if let Some(end_col) = &self.end_col {
            result += &format!(":{}", &to_a1_col(*end_col).unwrap());
        }
        if let Some(end_row) = &self.end_row {
            result += &format!("{}", end_row);
        }

        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_display() {
        let range = Range::new()
            .with_sheet("Sheet1")
            .with_start_cell(CellRange::from_a1("A1").expect("oops"))
            .with_end_cell(CellRange::from_a1("C3").expect("oops"));

        assert_eq!(format!("{}", range), "Sheet1!A1:C3");
    }

    #[test]
    fn test_range_try_from_str() {
        assert_eq!(
            Range::try_from("MyRange").expect("oops"),
            Range::new().with_name("MyRange")
        );

        assert_eq!(
            Range::try_from("Sheet1!A1:C3").expect("oops"),
            Range::new()
                .with_sheet("Sheet1")
                .with_start_cell(CellRange::from_a1("A1").expect("oops"))
                .with_end_cell(CellRange::from_a1("C3").expect("oops"))
        );
        assert_eq!(
            Range::try_from("Sheet1!R1C1:R3C3").expect("oops"),
            Range::new()
                .with_sheet("Sheet1")
                .with_start_cell(CellRange::from_r1c1("R1C1").expect("oops"))
                .with_end_cell(CellRange::from_r1c1("R3C3").expect("oops"))
        );
    }

    #[test]
    fn test_is_valid_range_name() {
        assert!(is_valid_range_name("AbC"));
        assert!(is_valid_range_name("Abc_123"));
        assert!(!is_valid_range_name(""));
        assert!(!is_valid_range_name("123"));
        assert!(!is_valid_range_name("1abc"));
        assert!(!is_valid_range_name("trueAbc"));
        assert!(!is_valid_range_name("falseAbc"));
        assert!(!is_valid_range_name("r1c1"));
    }
}
