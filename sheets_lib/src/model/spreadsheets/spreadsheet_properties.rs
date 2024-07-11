use crate::CellFormat;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{IterativeCalculationSettings, RecalculationIntervalType, SpreadsheetTheme};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadsheetProperties {
    pub title: String,
    pub locale: String,
    pub auto_recalc: RecalculationIntervalType,
    /// The time zone of the spreadsheet, in CLDR format such as America/New_York.
    /// If the time zone isn't recognized, this may be a custom time zone such as GMT-07:00.
    pub time_zone: String,
    pub default_format: CellFormat,
    pub iterative_calculation_settings: Option<IterativeCalculationSettings>,
    pub spreadsheet_theme: SpreadsheetTheme,
    pub import_functions_external_url_access_allowed: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        let json = r#"{
          "title": "Accounts_Test",
          "locale": "en_US",
          "autoRecalc": "ON_CHANGE",
          "timeZone": "America/New_York",
          "defaultFormat": {
            "backgroundColor": {
              "red": 1,
              "green": 1,
              "blue": 1
            },
            "padding": {
              "top": 2,
              "right": 3,
              "bottom": 2,
              "left": 3
            },
            "verticalAlignment": "BOTTOM",
            "wrapStrategy": "OVERFLOW_CELL",
            "textFormat": {
              "foregroundColor": {},
              "fontFamily": "arial,sans,sans-serif",
              "fontSize": 10,
              "bold": false,
              "italic": false,
              "strikethrough": false,
              "underline": false,
              "foregroundColorStyle": {
                "rgbColor": {}
              }
            },
            "backgroundColorStyle": {
              "rgbColor": {
                "red": 1,
                "green": 1,
                "blue": 1
              }
            }
          },
          "spreadsheetTheme": {
            "primaryFontFamily": "Arial",
            "themeColors": [
              {
                "colorType": "TEXT",
                "color": {
                  "rgbColor": {}
                }
              },
              {
                "colorType": "BACKGROUND",
                "color": {
                  "rgbColor": {
                    "red": 1,
                    "green": 1,
                    "blue": 1
                  }
                }
              },
              {
                "colorType": "ACCENT1",
                "color": {
                  "rgbColor": {
                    "red": 0.25882354,
                    "green": 0.52156866,
                    "blue": 0.95686275
                  }
                }
              },
              {
                "colorType": "ACCENT2",
                "color": {
                  "rgbColor": {
                    "red": 0.91764706,
                    "green": 0.2627451,
                    "blue": 0.20784314
                  }
                }
              },
              {
                "colorType": "ACCENT3",
                "color": {
                  "rgbColor": {
                    "red": 0.9843137,
                    "green": 0.7372549,
                    "blue": 0.015686275
                  }
                }
              },
              {
                "colorType": "ACCENT4",
                "color": {
                  "rgbColor": {
                    "red": 0.20392157,
                    "green": 0.65882355,
                    "blue": 0.3254902
                  }
                }
              },
              {
                "colorType": "ACCENT5",
                "color": {
                  "rgbColor": {
                    "red": 1,
                    "green": 0.42745098,
                    "blue": 0.003921569
                  }
                }
              },
              {
                "colorType": "ACCENT6",
                "color": {
                  "rgbColor": {
                    "red": 0.27450982,
                    "green": 0.7411765,
                    "blue": 0.7764706
                  }
                }
              },
              {
                "colorType": "LINK",
                "color": {
                  "rgbColor": {
                    "red": 0.06666667,
                    "green": 0.33333334,
                    "blue": 0.8
                  }
                }
              }
            ]
          }
        }"#;
        let result = serde_json::from_str::<SpreadsheetProperties>(json).expect("oops");
        dbg!(result);
    }
}
