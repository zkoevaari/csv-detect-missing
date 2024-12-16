/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE file)
*/

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset, SecondsFormat, TimeDelta};

#[derive(Debug, PartialEq)]
pub enum Difference {
    Number(i64),
    Duration(TimeDelta),
}
impl PartialOrd<Self> for Difference {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(i), Self::Number(o)) => i.partial_cmp(o),
            (Self::Duration(d), Self::Duration(o)) => d.partial_cmp(o),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Value {
    Number(i64),
    Timestamp(DateTime<FixedOffset>),
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(i) => i.fmt(f),
            Self::Timestamp(t) => t.to_rfc3339_opts(SecondsFormat::AutoSi, true).fmt(f),
        }
    }
}
impl std::ops::Sub for Value {
    type Output = Difference;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Number(i), Self::Number(o)) => Difference::Number(i - o),
            (Self::Timestamp(t), Self::Timestamp(o)) => Difference::Duration(t - o),
            _ => panic!("cannot use subtract on Values of different variants"),
        }
    }
}

#[derive(Debug)]
pub enum Format {
    UInt,
    Int,
    Unix,
    UnixMs,
    RFC3339,
}
impl TryFrom<String> for Format {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "uint" => Ok(Self::UInt),
            "int" => Ok(Self::Int),
            "unix" => Ok(Self::Unix),
            "unix_ms" => Ok(Self::UnixMs),
            "rfc-3339" => Ok(Self::RFC3339),
            _ => Err(format!("invalid format string: '{}'", s)),
        }
    }
}
impl Format {
    fn parse_value(&self, s: String) -> Result<Value, String> {
        fn format_err(e: impl Error) -> String {
            format!("could not be parsed: {}", e)
        }
        let timestamp_err = "could not be parsed: invalid timestamp";

        let s = s.trim().trim_start_matches("\"").trim_end_matches("\"");
        match self {
            Self::UInt => {
                let u = u64::from_str(s).map_err(format_err)?;
                match u > i64::MAX.try_into().unwrap() {
                    true => Err("could not be parsed: number too large (>2^64-1)".to_string()),
                    false => Ok(Value::Number(u.try_into().unwrap())),
                }
            }
            Self::Int => Ok(Value::Number(i64::from_str(s).map_err(format_err)?)),
            Self::Unix => Ok(Value::Timestamp(
                DateTime::from_timestamp(i64::from_str(s).map_err(format_err)?, 0)
                    .ok_or(timestamp_err)?
                    .into(),
            )),
            Self::UnixMs => Ok(Value::Timestamp(
                DateTime::from_timestamp_millis(i64::from_str(s).map_err(format_err)?)
                    .ok_or(timestamp_err)?
                    .into(),
            )),
            Self::RFC3339 => {
                //Not clear if underscore is valid in RFC3339, but it cannot hurt to allow here
                let s = s.replace('_', "T");
                Ok(Value::Timestamp(
                    DateTime::parse_from_rfc3339(&s).map_err(format_err)?,
                ))
            }
        }
    }

    pub fn parse_diff(&self, mut s: String) -> Result<Difference, String> {
        match self {
            Self::UInt | Self::Int => {
                Ok(Difference::Number(i64::from_str(&s).map_err(|e| {
                    format!("invalid numeric gap '{}': {}", s, e)
                })?))
            }
            Self::RFC3339 | Self::Unix | Self::UnixMs => {
                //Converting default "1" (which is otherwise a valid UInt etc.) to "1h"
                //Note: invalid "1" given explicitly will also be accepted this way
                if s == "1" {
                    s = "1h".to_string();
                }

                let err_base = format!("invalid rfc-3339 gap '{}'", s.as_str());
                let base: char = s.pop().ok_or(format!("{}: empty", &err_base))?;
                if s.is_empty() {
                    return Err(format!("{}: invalid value or timebase", &err_base));
                }
                let value = i64::from_str(&s).map_err(|e| format!("{}: {}", &err_base, e))?;
                match base {
                    's' => Ok(Difference::Duration(TimeDelta::seconds(value))),
                    'm' => Ok(Difference::Duration(TimeDelta::minutes(value))),
                    'h' => Ok(Difference::Duration(TimeDelta::hours(value))),
                    'd' => Ok(Difference::Duration(TimeDelta::days(value))),
                    'w' => Ok(Difference::Duration(TimeDelta::weeks(value))),
                    ch => Err(format!("{}: unexpected character '{}'", &err_base, ch)),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Comparison {
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}
impl Comparison {
    fn compare(&self, a: &Difference, b: &Difference) -> bool {
        match self {
            Self::GreaterThan => a > b,
            Self::GreaterOrEqual => a >= b,
            Self::LessThan => a < b,
            Self::LessOrEqual => a <= b,
        }
    }
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
    pub difference: Difference,
    pub comment: String,
    pub allow_empty: bool,
    pub verbose: bool,
    pub mode: Mode,
    pub path: PathBuf,
}

pub fn csv_detect_missing(mut args: Arguments) -> Result<(), Box<dyn Error>> {
    if args.verbose {
        writeln!(std::io::stdout(), "{:#?}", args)?
    };

    match args.delimiter.as_str() {
        "\\t" => {
            args.delimiter = char::from(9).to_string();
            if args.verbose {
                writeln!(std::io::stdout(), "Using Tabulator as input delimiter.")?;
            }
        }
        "" => {
            if args.index != 1 {
                return Err("supplied index and delimiter are incompatible".into());
            } else if args.verbose {
                writeln!(
                    std::io::stdout(),
                    "No delimiter, using whole line as target field.",
                )?;
            }
        }
        _ => (),
    }
    if let Mode::Diff(ref odelim) = args.mode {
        match odelim.as_str() {
            "\\t" => {
                args.mode = Mode::Diff(char::from(9).to_string());
                if args.verbose {
                    writeln!(std::io::stdout(), "Using Tabulator as output delimiter.")?;
                }
            }
            "" => {
                args.mode = Mode::Diff(args.delimiter.clone());
                if args.verbose {
                    writeln!(
                        std::io::stdout(),
                        "No output delimiter, using same as input.",
                    )?;
                }
            }
            _ => (),
        }
    }

    let mut reader: Box<dyn BufRead> = if args.path == PathBuf::from("-") {
        Box::new(BufReader::new(std::io::stdin().lock()))
    } else {
        Box::new(BufReader::new(File::open(args.path)?))
    };

    let mut buf = String::new();
    let mut n: u64 = 0;
    struct Previous {
        line: String,
        value: Value,
    }
    let mut prev: Option<Previous> = None;
    let mut first = true;

    while reader.read_line(&mut buf)? > 0 {
        n += 1;
        let line = buf.trim();

        'processing: {
            if !args.comment.is_empty() && line.starts_with(&args.comment) {
                break 'processing;
            }
            if line.is_empty() {
                match args.allow_empty {
                    true => break 'processing,
                    false => return Err(format!("line {} is empty", n).into()),
                }
            };

            let field = match args.delimiter.is_empty() {
                true => line,
                false => match line
                    .split(&args.delimiter)
                    .nth((args.index.checked_sub(1).unwrap()).into())
                {
                    Some(s) if !s.is_empty() => s,
                    Some(_) if args.allow_empty => break 'processing,
                    Some(_) => {
                        return Err(format!(
                            "line {} is invalid: empty field at index {}",
                            n, args.index,
                        )
                        .into())
                    }
                    None if args.allow_empty => break 'processing,
                    None => {
                        return Err(format!(
                            "line {} is invalid: no field could be found at index {}",
                            n, args.index,
                        )
                        .into())
                    }
                },
            };

            let value = args
                .format
                .parse_value(field.to_string())
                .map_err(|e| format!("line {} field '{}' {}", n, field, e))?;

            if let Some(prev) = prev {
                let diff = value - prev.value;

                let condition = args.comparison.compare(&diff, &args.difference);
                if condition {
                    match args.mode {
                        Mode::Diff(ref delim) => {
                            writeln!(std::io::stdout(), "{}{}{}", prev.value, delim, value)?
                        }
                        Mode::Filter => {
                            match first {
                                true => first = false,
                                false => writeln!(std::io::stdout())?,
                            }
                            writeln!(std::io::stdout(), "{}\n{}", prev.line, line)?;
                        }
                    }
                }
            }

            prev = Some(Previous {
                line: line.to_string(),
                value,
            });
        }

        buf.clear();
    }

    Ok(())
}
