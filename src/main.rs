/*
    Created by Zoltan Kovari, 2024.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE file)
*/

use csv_detect_missing::*;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg_matches = clap::Command::new("csv-detect-missing")
        .version(clap::crate_version!())
        .after_long_help("Created by Zoltan Kovari, 2024. Licensed under the Apache License, Version 2.0")
        .about("Tool to inspect CSV data, looking for (time) gaps between subsequent lines.")
        .long_about("Tool to inspect CSV data, looking for (time) gaps between subsequent lines.
In a more general sense:
Calculates the difference between numerical or time field values in subsequent
lines of text, and reports gaps greater/less than allowed.")
        .arg(clap::Arg::new("delimiter")
            .short('d')
            .help("Input delimiter")
            .long_help("Delimiter string that separate the input fields. Can be longer than
a single char. Empty string turns off field separation, resulting in
the whole line being treated as one field.")
            .num_args(1)
            .value_name("DELIM")
            .value_parser(clap::value_parser!(String))
            .default_value(",")
        )
        .arg(clap::Arg::new("index")
            .short('i')
            .help("Field index")
            .long_help("Index of the field to be parsed and evaluated, starting from 1.")
            .num_args(1)
            .value_name("INDEX")
            .value_parser(clap::value_parser!(u16).range(1..))
            .default_value("1")
        )
        .arg(clap::Arg::new("format")
            .short('f')
            .help("Format")
            .long_help("Format of the selected field, with the following options supported:
    uint: Unsigned integer value.
    int: Signed integer value.
    unix: Non-leap seconds passed since the Unix Epoch.
    unix_ms: Similar to 'unix' but in milliseconds.
    rfc-3339: Timestamp like \"yyyy-mm-ddTHH:MM:SSZ\".")
            .num_args(1)
            .value_name("FORMAT")
            .value_parser(["uint", "int", "unix", "unix_ms", "rfc-3339"])
            .hide_possible_values(true)
            .default_value("uint")
        )
        .arg(clap::Arg::new("greater-than")
            .long("gt")
            .help("'Greater-than' comparison behavior (default)")
            .long_help("Greater gaps than the value supplied do trigger output generation,
when comparing the difference between subsequent lines. This is
default behavior when omitted, unless one of --ge, --lt, or --le
is specified.
Gap syntax is according to selected format:
    uint and int: Specified as a signed integer. [default: 1]
    rfc-3339, unix, and unix_ms: Signed integer followed by one
        character from [dhms], like \"12h\". [default: 1h]")
            .num_args(1)
            .value_name("GAP")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .default_value("1")
            .hide_default_value(true)
            .conflicts_with_all(["greater-or-equal", "less-than", "less-or-equal"])
        )
        .arg(clap::Arg::new("greater-or-equal")
            .long("ge")
            .help("'Greater-or-equal' comparison behavior")
            .long_help("'Greater-or-equal' comparison behavior, also see -gt.")
            .num_args(1)
            .value_name("GAP")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .conflicts_with_all(["greater-than", "less-than", "less-or-equal"])
        )
        .arg(clap::Arg::new("less-than")
            .long("lt")
            .help("'Less-than' comparison behavior")
            .long_help("'Less-than' comparison behavior, also see -gt.")
            .num_args(1)
            .value_name("GAP")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .conflicts_with_all(["greater-than", "greater-or-equal", "less-or-equal"])
        )
        .arg(clap::Arg::new("less-or-equal")
            .long("le")
            .help("'Less-or-equal' comparison behavior")
            .long_help("'Less-or-equal' comparison behavior, also see -gt.")
            .num_args(1)
            .value_name("GAP")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .conflicts_with_all(["greater-than", "greater-or-equal", "less-than"])
        )
        .arg(clap::Arg::new("comment")
            .short('c')
            .help("Comment marker")
            .long_help("Comment string, skipping if detected at the start of a line. Empty
string turns off comment detection.")
            .num_args(1)
            .value_name("COMMENT")
            .value_parser(clap::value_parser!(String))
            .default_value("#")
        )
        .arg(clap::Arg::new("allow-empty")
            .short('a')
            .help("Allow empty or invalid lines")
            .long_help("Allow empty lines: contrary to default behavior, no error given when
invalid line is encountered (empty or less fields than expected).")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("diff")
            .short('D')
            .long("diff")
            .help("Diff mode (default): one delimiter-separated line per
gap")
            .long_help("Diff mode: reports one line per gap with the two values separated by
the given output delimiter (using same as input if empty). This is
the default behavior.")
            .num_args(0..=1)
            .value_name("DELIM")
            .value_parser(clap::value_parser!(String))
            .default_value(",")
            .default_missing_value(",")
        )
        .arg(clap::Arg::new("filter")
            .short('F')
            .long("filter")
            .help("Filter mode: keep only offending lines")
            .long_help("Filter mode: reports both \"side\" of the offending gap, as in both
lines unchanged, followed by an empty line.")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with("diff")
        )
        .arg(clap::Arg::new("verbose")
            .short('v')
            .help("Verbose mode: print debug header")
            .long_help("Verbose mode: print argument information header (for debug).")
            .action(clap::ArgAction::SetTrue)
        )
        .arg(clap::Arg::new("FILE")
            .help("Input file")
            .long_help("Input file must be a delimiter separated text file, or it should
contain one valid value per line.")
            .value_parser(clap::builder::NonEmptyStringValueParser::new())
            .required(true)
        )
        .get_matches();


    let format: Format = arg_matches.get_one::<String>("format").unwrap().to_string().try_into()?;

    let gt = arg_matches.get_one::<String>("greater-than").cloned();
    let ge = arg_matches.get_one::<String>("greater-or-equal").cloned();
    let lt = arg_matches.get_one::<String>("less-than").cloned();
    let le = arg_matches.get_one::<String>("less-or-equal").cloned();
    let (comparison, gap) = match (gt, ge, lt, le) {
        (Some(gap), None, None, None) => (Comparison::GreaterThan, gap),
        (_, Some(gap), None, None) => (Comparison::GreaterOrEqual, gap),
        (_, None, Some(gap), None) => (Comparison::LessThan, gap),
        (_, None, None, Some(gap)) => (Comparison::LessOrEqual, gap),
        _ => unreachable!()
    };
    let difference = format.parse_diff(gap)?;

    let mode = match arg_matches.get_flag("filter") {
        true => Mode::Filter,
        false => Mode::Diff(arg_matches.get_one::<String>("diff").unwrap().to_string())
    };

    let args = Arguments {
        delimiter: arg_matches.get_one::<String>("delimiter").unwrap().to_string(),
        index: *arg_matches.get_one("index").unwrap(),

        format,
        comparison,
        difference,

        comment: arg_matches.get_one::<String>("comment").unwrap().to_string(),
        allow_empty: arg_matches.get_flag("allow-empty"),
        verbose: arg_matches.get_flag("verbose"),

        mode,

        path: arg_matches.get_one::<String>("FILE").unwrap().into(),
    };


    match csv_detect_missing(args) {
        Err(err) => match err.downcast_ref::<std::io::Error>() {
            Some(ioerr) => match ioerr.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(err)
            },
            None => Err(err)
        },
        Ok(ok) => Ok(ok)
    }
}
