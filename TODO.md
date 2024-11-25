
TODO
====

## Bugs ##
* Fails to parse space separated rfc-3339 in space delimited file even if quoted, e.g. "1971-07-30 22:16:29Z"
* Broken pipe panic when run as input to 'head'

## Unimplemented features ##
* Ellipsize field in error messages
* FILE argument '-' should mean to read the argument/content from STDIN (file descriptor 0)
* Should be able to use negative time-base gaps for timestamps

## Proposed features ##
* Gap resolution: should be dependent of time-base used (and clarify in help)
* 'Equals' comparison with --eq, also --ne
* Diff mode should also output the calculated gap (in seconds or other format), and possibly the line number
* Header in diff mode and quiet flag with -q to turn it off
* Output format option (for timestamps)
* Multiple comparison options, or some way to chain?
* Floating point numeric format
* Full ISO 8601 support, including period format for gaps
