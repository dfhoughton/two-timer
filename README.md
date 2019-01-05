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
* June '05
* Monday through next Thursday
* from mon at 15:00:05 to now
* 1960-05-06
* 5000BCE
* next weekend

The complete API is available at https://docs.rs/two_timer/0.1.0/two_timer/.
