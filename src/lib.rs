/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE file)
*/

use std::str::FromStr;
use std::error::Error;

use chrono::TimeDelta;


#[derive(Debug)]
pub enum Format {
    UInt,
    RFC3339,
}
impl TryFrom<String> for Format {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "uint" => Ok(Self::UInt),
            "rfc-3339" => Ok(Self::RFC3339),
            _ => Err(format!("invalid format string: '{}'", s))
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Number(u32),
    Duration(TimeDelta),
}
impl Value {
    pub fn new(mut s: String, format: &Format) -> Result<Self, String> {
        match format {
            Format::UInt => Ok(Value::Number(u32::from_str(&s).map_err(|_| format!("invalid uint gap value string: '{}'", s))?)),
            Format::RFC3339 => {
                //Using "1h" as default
                //Note: invalid "1" given will also be accepted this way
                if s == "1" { s = "1h".to_string(); }

                let err = format!("invalid rfc-3339 gap value string: '{}'", s.as_str());
                let base: char = s.pop().ok_or(&err)?;
                let value: u32 = u32::from_str(&s).map_err(|_| &err)?;
                match base {
                    'd' => Ok(Self::Duration(TimeDelta::days(value.into()))),
                    'h' => Ok(Self::Duration(TimeDelta::hours(value.into()))),
                    'm' => Ok(Self::Duration(TimeDelta::minutes(value.into()))),
                    's' => Ok(Self::Duration(TimeDelta::seconds(value.into()))),
                    _ => Err(err)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Comparison {
    GreaterThan(Value),
    GreaterOrEqual(Value),
    LessThan(Value),
    LessOrEqual(Value),
}

#[derive(Debug)]
pub enum Mode {
    Diff(String),
    Filter,
}

#[derive(Debug)]
pub struct Arguments {
    pub delimiter: String,
    pub index: u16,
    pub format: Format,
    pub comparison: Comparison,
    pub comment: String,
    pub allow_empty: bool,
    pub verbose: bool,
    pub mode: Mode,
    pub file: std::path::PathBuf,
}


pub fn csv_detect_missing(args: Arguments) -> Result<(), Box<dyn Error>> {
    println!("{:?}", args);
    Ok(())
}
