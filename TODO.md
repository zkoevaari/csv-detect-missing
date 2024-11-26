
TODO
====

## Bugs ##
* Fails to parse space separated rfc-3339 in space delimited file even if quoted, e.g. "1971-07-30 22:16:29Z"
* Should ellipsize field in error messages

## Features ##
* Full ISO 8601 support, including period format for gaps
* Gap resolution: should be dependent of time-base used (and clarify in help)
* Output format option (for timestamps)
* 'Equals' comparison with --eq, also --ne
* Multiple comparison options, or some way to chain?
* Diff mode should also output the calculated gap (in seconds or other format), and possibly the line number
* Header in diff mode and quiet flag with -q to turn it off
* Floating point numeric format
