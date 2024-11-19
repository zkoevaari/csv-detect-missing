
TESTS
=====

These test CSV files should collectively be able to exercise all features of the csv-detect-missing program:
- delimiters: comma, semicolon, space, tab
- index: 1, 2, 3+
- format: rfc-3339, uint (timestamp, value)
- comment: # ; (maybe something longer like "Error")
- empty line detection
- invalid line detection: incorrect or wrong format, missing or empty field
- comparison: possibility of all 4 relations with all 5 gaps (4 time bases, plus uint)


Synthetic tests
---------------

Tests designed for easy and quick demonstration, or to exercise obscure program features.

Important: Source for the data used in many of these synthetic tests is en.wikipedia.org. In accordance with their CC BY-SA 4.0 license, I am hereby giving full credit to the original authors, and release these with the same license CC BY-SA 4.0. Please see the TEST_LICENSE file for the full text. I have to also declare that the original data might have been remixed, transformed, and built upon in various ways so I can use them for these demonstrations, so I cannot guarantee that the data itself is still intact and urge you to use them with caution (although obviously my intent was to alter the format only, not the data).


### winter_olympics.csv

List of Winter Olympic years and cities.
Source: https://en.wikipedia.org/wiki/Winter_Olympic_Games

- delimiter: comma
- index: 2
- format: uint

There is an obvious gap around WW2:
- by default, program should halt on line 6, both when the index (-i) is 1 or 2
- with the allow flag (-a), when invoked with --gt "4" it should output the 12-year gap after 1936
- with --lt "4", program should report one find at 1992
- long comment can be demonstrated with -c "N/A" (and without -a flag)


### summer_olympics.csv

List of Winter Olympic years and cities.
Source: https://en.wikipedia.org/wiki/Summer_Olympic_Games

- delimiter: tab
- index: 2
- format: uint

Some further features to show with this file:
- comment: # (default)
- program should halt on line 7 (1916) as invalid due to missing field
- with the allow flag (-a) it should output the two gaps due to the wars


### apollo.csv

List of crewed NASA Apollo missions, launch and Moon landing dates, with mission duration data.
Sources:
https://en.wikipedia.org/wiki/List_of_Apollo_missions
https://en.wikipedia.org/wiki/Moon_landing
also
https://en.wikipedia.org/wiki/Apollo_9
through
https://en.wikipedia.org/wiki/Apollo_17

- delimiter: ?comma?
- index: 2 or 7
- format: ?rfc-3339?
- comment: # (default)

Date capabilities can be demonstrated with this file, including various time zones. Also because several missions did not reach the Moon (although some were close), the empty landing date field should halt the program as invalid, needing the -a flag to proceed.


### moon.csv

TODO



Sensor tests
------------

Tests that much more akin to real-world engineering data, like periodic sensor readings.

Important: These files are based on actual measurements from my own collection, severly edited. To be symmetric, I am releasing these under CC BY-SA 4.0 license as well. Please see the TEST_LICENSE file for the full text.


### sensor1.csv

File contains readings from three different sensors, one measurement per line. First field is the sensor ID, second the nominal timestamp and third is actual timestamp. Readout interval is 2 minutes nominal.

- delimiter: comma
- index: 2 or 3
- format: uint (timestamp)

When considering field no.2, program should produce empty output, but there are a couple of places where fields no.3 (and rest) are completely missing. These should halt the program when run with index set to 3 (-i3), and whith the allow flag (-a) also set, it should give a single result considering that on all but one occasion only 2 of the 3 sensors missed. When run through grep to filter for only one of the IDs, it should report all ? pieces of 4-minute gaps.


### sensor2.csv

File contains readings from four temperature sensors. Measurement interval is 5 minutes nominal.

- delimiter: semicolon
- index: 1
- format: rfc-3339

Program should detect a gap of slightly more then an hour around 4PM, where also there is an empty line insterted.


### sensor3.csv

File contains readings from a weather station. Measurement interval is 1 minute nominal.

- delimiter: tab
- index: 1
- format: rfc-3339
- comment: # on line 1

TODO


### sensor4.csv

- delimiter: space
- index: 1
- format: ?

File contains counter data, with readout interval of 1 hour nominal.

There is a gap around the 22nd day, with an error message:
- Should detect wrong format for both fields
- Should be able to ignore with either the allow flag or defining a comment string
