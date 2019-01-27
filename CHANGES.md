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