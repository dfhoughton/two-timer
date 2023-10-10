# Change Log

## 2.2.5 *2023-10-9*
* merged fix by @Paradyx for Numeral dates with december fail to parse #8
## 2.2.4 *2023-1-20*
* made cargo-audit happy
## 2.2.3 *2021-8-15*
* accepted PR from Geobert implementing std::error::Error and std::fmt::Display for TimeError
## 2.2.2 *2021-7-3*
* slight modernization
## 2.2.1 *2021-3-29*
* changing the license from GPL 2 to MIT
## 2.2.0 *2020-10-3*
* adding `default_to_past` configuration parameter to allow people to interpret relative times like "Tuesday" as the nearest such moment in the future rather than the past
## 2.1.0 *2020-5-17*
* added since expressions: "since yesterday", "since the beginning of the month", "since the end of last year", "after midnight", ...
* added "the" as a synonym of "this" as a period modifier: "the beginning of the month" = "the beginning of this month"
## 2.0.0 *2020-3-7*
* fixing specific time to specific time pattern: "noon yesterday through midnight today"
* allow parsing of hours with leading 0; e.g., "08:57:29"
* added "month the nth" pattern -- "July the 4th", "November the Fifth"
* ***IMPORTANT*** changing the nature of daytimes -- "3 PM", "13:14" -- so their period is always 1 second; this seems
more intuitively correct to me, but it changes the semantics sufficiently that I thought it necessary to bump the major version number
## 1.3.4
* fixed panic when parsing "24"
## 1.3.3
* revert `ToString` to `&str` for greater efficiency
## 1.3.2
* removed serialized grammar as it made maintenance unwieldy
* added "payperiod" as another synonym for "pay period"
* adding bare "pay period" (also bear "week", "month", "year", and "weekend")
## 1.3.1
* documentation fix
## 1.3.0
* adding small_grammar feature to further speed up common use case
## 1.2.1
* bumpled lazy_static and pidgin dependencies
* use serialized matcher to avoid the cost of generating two_timer::GRAMMAR via the macro;
this cuts about 0.4 seconds off the startup time of two_timer on my machine, going from 0.85 seconds to 0.48 seconds
## 1.2.0
* added parsable function
## 1.1.0
* bumped lazy_static dependency
## 1.0.8
* made the space between time and AM/PM optional
## 1.0.7
* added `<specific_time>` pattern: e.g., 1969-05-06 12:03:05
## 1.06
* added "before and after"
* fixed "Friday the 13th" and "the 31st" to scan back through the calendar to the nearest match
## 1.0.5
* better organization and documentation of grammar
## 1.0.4
* added `<year>` pattern
* added ordinals for days of the month
* added kalends, nones, ides
* added March 5th, the fifth, Friday the 13th, etc.
* added period before/after/around time
* added noon and midnight
* added `<count>` `<periods>` from now/ago
## 1.0.3
* fixed "12 pm" bug
## 1.0.2
* added `msg` method to `TimeError`
## 1.0.1
* removing some documentation
## 1.0.0
* convert `Date<Utc>` and `DateTime<Utc>` everywhere to `NaiveDate` and `NaiveDateTime`
* added "weekend" for the expressions "this weekend", "last weekend", etc.
* don't require space between era suffix and year -- "100AD" is as good as "100 AD"
