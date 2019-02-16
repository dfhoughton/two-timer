# two-timer
Rust library for parsing English time expressions into start and end timestamps

This takes English expressions and returns a time range which ideally matches the expression.
You might use this for registering the temporal extent of an event, say, or finding
lines in a log file.

Some expressions it can handle:

* from now to eternity
* today
* tomorrow
* last month
* this year
* 5/6/69
* June 6, 2010
* forever
* 3:00 AM
* 3AM
* June '05
* Monday through next Thursday
* from mon at 15:00:05 to now
* 1960-05-06
* 5000BCE
* next weekend
* 2000
* the nineteenth of March 1810
* the 5th of November
* the ides of March
* the first
* two seconds before 12:00 PM
* 1 week after May first
* 15 minutes around 12:13:43 PM
* noon on May 6, 1969
* midnight on May 6, 1969
* Friday the 13th
* 2 weeks ago
* ten seconds from now
* 5 minutes before and after midnight
* 1969-05-06 12:03:05

The complete API is available at https://docs.rs/two_timer/0.1.0/two_timer/.
