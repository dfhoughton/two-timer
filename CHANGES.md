# Change Log

## 1.0.0
* convert `Date<Utc>` and `DateTime<Utc>` everywhere to `NaiveDate` and `NaiveDateTime`
* added "weekend" for the expressions "this weekend", "last weekend", etc.
* don't require space between era suffix and year -- "100AD" is as good as "100 AD"
## 1.0.1
* removing some documentation
## 1.0.2
* added `msg` method to `TimeError`
## 1.0.3
* fixed "12 pm" bug
## 1.0.4
* added `<year>` pattern
* added ordinals for days of the month
* added kalends, nones, ides
* added March 5th, the fifth, Friday the 13th, etc.
* added period before/after/around time
* added noon and midnight
* added `<count>` `<periods>` from now/ago
## 1.0.5
* better organization and documentation of grammar
## 1.06
* added "before and after"
* fixed "Friday the 13th" and "the 31st" to scan back through the calendar to the nearest match
## 1.0.7
* added `<specific_time>` pattern: e.g., 1969-05-06 12:03:05