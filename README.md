csv-detect-missing
==================

This is a command line application written in Rust to inspect CSV data that 
contains a timestamp field, calculating the time difference between subsequent 
lines, and reporting if the difference is greater than some set gap. It's main 
purpose is to be able to examine sensor time series data for faulty periods.

## Usage ##

Short help using `-h`:
```
Usage: csv-detect-missing [OPTIONS] <FILE>

Arguments:
  <FILE>  Input file, or '-' to read from STDIN

Options:
  -d <DELIM>            Input delimiter [default: ,]
  -i <INDEX>            Field index [default: 1]
  -f <FORMAT>           Format [default: uint]
      --gt <GAP>        'Greater-than' comparison behavior (default)
      --ge <GAP>        'Greater-or-equal' comparison behavior
      --lt <GAP>        'Less-than' comparison behavior
      --le <GAP>        'Less-or-equal' comparison behavior
  -c <COMMENT>          Comment marker [default: #]
  -a                    Allow empty or invalid lines
  -D, --diff [<DELIM>]  Diff mode (default): one delimiter-separated line per
                        gap [default: ,]
  -F, --filter          Filter mode: keep only offending lines
  -v                    Verbose mode: print debug header
  -h, --help            Print help (see more with '--help')
  -V, --version         Print version
```

Long help using `--help`:
```
Usage: csv-detect-missing [OPTIONS] <FILE>

Arguments:
  <FILE>
          Input file must be a delimiter separated text file, or it should
          contain one valid value per line. If supplied a single hyphen ('-')
          instead of a file path, input is read from STDIN.

Options:
  -d <DELIM>
          Delimiter string that separate the input fields. Can be longer than
          a single char. Empty string turns off field separation, resulting in
          the whole line being treated as one field.
          
          [default: ,]

  -i <INDEX>
          Index of the field to be parsed and evaluated, starting from 1.
          
          [default: 1]

  -f <FORMAT>
          Format of the selected field, with the following options supported:
              uint: Unsigned integer value.
              int: Signed integer value.
              unix: Non-leap seconds passed since the Unix Epoch.
              unix_ms: Similar to 'unix' but in milliseconds.
              rfc-3339: Timestamp like "yyyy-mm-ddTHH:MM:SSZ".
          
          [default: uint]

      --gt <GAP>
          Greater gaps than the value supplied do trigger output generation,
          when comparing the difference between subsequent lines. This is
          default behavior when omitted, unless one of --ge, --lt, or --le
          is specified.
          Gap syntax is according to selected format:
              uint and int: Specified as a signed integer. [default: 1]
              rfc-3339, unix, and unix_ms: Signed integer followed by one
                  character from [dhms], like "12h". [default: 1h]

      --ge <GAP>
          'Greater-or-equal' comparison behavior, also see -gt.

      --lt <GAP>
          'Less-than' comparison behavior, also see -gt.

      --le <GAP>
          'Less-or-equal' comparison behavior, also see -gt.

  -c <COMMENT>
          Comment string, skipping if detected at the start of a line. Empty
          string turns off comment detection.
          
          [default: #]

  -a
          Allow empty lines: contrary to default behavior, no error given when
          invalid line is encountered (empty or less fields than expected).

  -D, --diff [<DELIM>]
          Diff mode: reports one line per gap with the two values separated by
          the given output delimiter (using same as input if empty). This is
          the default behavior.
          
          [default: ,]

  -F, --filter
          Filter mode: reports both "side" of the offending gap, as in both
          lines unchanged, followed by an empty line.

  -v
          Verbose mode: print argument information header (for debug).

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Created by Zoltan Kovari, 2024. Licensed under the Apache License, Version 2.0
```

## Example ##

Let's consider the following input (excerpt from 
[`winter_olympics.csv`](tests/synthetic/winter_olympics.csv) test 
file):
```
1,1924,Chamonix
2,1928,St. Moritz
3,1932,Lake Placid
4,1936,Garmisch-Partenkirchen
N/A,Cancelled
N/A,Cancelled
5,1948,St. Moritz
6,1952,Oslo
7,1956,Cortina d'Ampezzo
8,1960,Squaw Valley
9,1964,Innsbruck
...
```

The following sample invocation would detect the missing years:
```
./csv-detect-missing -d "," -i 2 -f uint --gt 4 -c "N/A" winter_olympics.csv
1936,1948
```

## Further reading ##

If interested, see the related blog post about the development of version 1.0.0:

https://www.rockfort.io/blog/2024/241227_csv_detect_missing.html
