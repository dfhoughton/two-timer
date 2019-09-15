/*!

This crate provides a `parse` function to convert English time expressions into a pair
of timestamps representing a time range. It converts "today" into the first and last
moments of today, "May 6, 1968" into the first and last moments of that day, "last year"
into the first and last moments of that year, and so on. It does this even for expressions
generally interpreted as referring to a point in time, such as "3 PM". In these cases
the width of the time span varies according to the specificity of the expression. "3 PM" has
a granularity of an hour, "3:00 PM", of a minute, "3:00:00 PM", of a second. For pointwise
expression the first moment is the point explicitly named. The `parse` expression actually
returns a 3-tuple consisting of the two timestamps and whether the expression is literally
a range -- two time expressions separated by a preposition such as "to", "through", "up to",
or "until".

# Example

```rust
extern crate two_timer;
use two_timer::{parse, Config};
extern crate chrono;
use chrono::naive::NaiveDate;

pub fn main() {
    let phrases = [
        "now",
        "this year",
        "last Friday",
        "from now to the end of time",
        "Ragnarok",
        "at 3:00 pm today",
        "5/6/69",
        "Tuesday, May 6, 1969 at 3:52 AM",
        "March 15, 44 BC",
        "Friday the 13th",
        "five minutes before and after midnight",
    ];
    // find the maximum phrase length for pretty formatting
    let max = phrases
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();
    for phrase in phrases.iter() {
        match parse(phrase, None) {
            Ok((d1, d2, _)) => println!("{:width$} => {} --- {}", phrase, d1, d2, width = max),
            Err(e) => println!("{:?}", e),
        }
    }
    let now = NaiveDate::from_ymd_opt(1066, 10, 14).unwrap().and_hms(12, 30, 15);
    println!("\nlet \"now\" be some moment during the Battle of Hastings, specifically {}\n", now);
    let conf = Config::new().now(now);
    for phrase in phrases.iter() {
        match parse(phrase, Some(conf.clone())) {
            Ok((d1, d2, _)) => println!("{:width$} => {} --- {}", phrase, d1, d2, width = max),
            Err(e) => println!("{:?}", e),
        }
    }
}
```
produces
```text
now                                    => 2019-02-03 14:40:00 --- 2019-02-03 14:41:00
this year                              => 2019-01-01 00:00:00 --- 2020-01-01 00:00:00
last Friday                            => 2019-01-25 00:00:00 --- 2019-01-26 00:00:00
from now to the end of time            => 2019-02-03 14:40:00 --- +262143-12-31 23:59:59.999
Ragnarok                               => +262143-12-31 23:59:59.999 --- +262143-12-31 23:59:59.999
at 3:00 pm today                       => 2019-02-03 15:00:00 --- 2019-02-03 15:01:00
5/6/69                                 => 1969-05-06 00:00:00 --- 1969-05-07 00:00:00
Tuesday, May 6, 1969 at 3:52 AM        => 1969-05-06 03:52:00 --- 1969-05-06 03:53:00
March 15, 44 BC                        => -0043-03-15 00:00:00 --- -0043-03-16 00:00:00
Friday the 13th                        => 2018-07-13 00:00:00 --- 2018-07-14 00:00:00
five minutes before and after midnight => 2019-02-02 23:55:00 --- 2019-02-03 00:05:00

let "now" be some moment during the Battle of Hastings, specifically 1066-10-14 12:30:15

now                                    => 1066-10-14 12:30:00 --- 1066-10-14 12:31:00
this year                              => 1066-01-01 00:00:00 --- 1067-01-01 00:00:00
last Friday                            => 1066-10-05 00:00:00 --- 1066-10-06 00:00:00
from now to the end of time            => 1066-10-14 12:30:00 --- +262143-12-31 23:59:59.999
Ragnarok                               => +262143-12-31 23:59:59.999 --- +262143-12-31 23:59:59.999
at 3:00 pm today                       => 1066-10-14 15:00:00 --- 1066-10-14 15:01:00
5/6/69                                 => 0969-05-06 00:00:00 --- 0969-05-07 00:00:00
Tuesday, May 6, 1969 at 3:52 AM        => 1969-05-06 03:52:00 --- 1969-05-06 03:53:00
March 15, 44 BC                        => -0043-03-15 00:00:00 --- -0043-03-16 00:00:00
Friday the 13th                        => 1066-07-13 00:00:00 --- 1066-07-14 00:00:00
five minutes before and after midnight => 1066-10-13 23:55:00 --- 1066-10-14 00:05:00
```

For the full grammar of time expressions, view the source of the `parse` function and
scroll up. The grammar is provided at the top of the file.

# Relative Times

It is common in English to use time expressions which must be interpreted relative to some
context. The context may be verb tense, other events in the discourse, or other semantic or
pragmatic clues. The `two_timer` `parse` function doesn't attempt to infer context perfectly, but
it does make some attempt to get the context right. So, for instance "last Monday through Friday", said
on Saturday, will end on a different day from "next Monday through Friday". The general rules
are

1. a fully-specified expression in a pair will provide the context for the other expression
2. a relative expression will be interpreted as appropriate given its order -- the second expression
describes a time after the first
3. if neither expression is fully-specified, the first will be interpreted relative to "now" and the
second relative ot the first

The rules of interpretation for relative time expressions in ranges will likely be refined further
in the future.

# Clock Time

The parse function interprets expressions such as "3:00" as referring to time on a 24 hour clock, so
"3:00" will be interpreted as "3:00 AM". This is true even in ranges such as "3:00 PM to 4", where the
more natural interpretation might be "3:00 PM to 4:00 PM".

# Years Near 0

Since it is common to abbreviate years to the last two digits of the century, two-digit
years will be interpreted as abbreviated unless followed by a suffix such as "B.C.E." or "AD".
They will be interpreted as the the nearest appropriate *previous* year to the current moment,
so in 2010 "'11" will be interpreted as 1911, not 2011.

# The Second Time in Ranges

For single expressions, like "this year", "today", "3:00", or "next month", the second of the
two timestamps is straightforward -- it is the end of the relevant temporal unit. "1971" will
be interpreted as the first moment of the first day of 1971 through, but excluding, the first
moment of the first day of 1972, so the second timestamp will be this first excluded moment.

When the parsed expression describes a range, we're really dealing with two potentially overlapping
pairs of timestamps and the choice of the terminal timestamp gets trickier. The general rule
will be that if the second interval is shorter than a day, the first timestamp is the first excluded moment,
so "today to 3:00 PM" means the first moment of the day up to, but excluding, 3:00 PM. If the second
unit is as big as or larger than a day, which timestamp is used varies according to the preposition.
"This week up to Friday" excludes all of Friday. "This week through Friday" includes all of Friday.
Prepositions are assumed to fall into either the "to" class or the "through" class. You may also use
a series of dashes as a synonym for "through", so "this week - fri" is equivalent to "this week through Friday".
For the most current list of prepositions in each class, consult the grammar used for parsing, but
as of the moment, these are the rules:

```text
        up_to => [["to", "until", "up to", "till"]]
        through => [["up through", "through", "thru"]] | r("-+")
```

# Pay Periods

I'm writing this library in anticipation of, for the sake of amusement, rewriting [JobLog](https://metacpan.org/pod/App::JobLog)
in Rust. This means I need the time expressions parsed to include pay periods. Pay periods, though,
are defined relative to some reference date -- a particular Sunday, say -- and have a variable period.
`two_timer`, and JobLog, assume pay periods are of a fixed length and tile the timeline without overlap, so a
pay period of a calendrical month is problematic.

If you need to interpret "last pay period", say, you will need to specify when this pay period began, or
when some pay period began or will begin, and a pay period length in days. The `parse` function has a second
optional argument, a `Config` object, whose chief function outside of testing is to provide this information. So,
for example, you could do this:

```rust
# extern crate two_timer;
# use two_timer::{parse, Config};
let (reference_time, _, _) = parse("5/6/69", None).unwrap();
let config = Config::new().pay_period_start(Some(reference_time.date()));
let (t1, t2, _) = parse("next pay period", Some(config)).unwrap();
```

# Ambiguous Year Formats

`two_timer` will try various year-month-day permutations until one of them parses given that days are in the range 1-31 and
months, 1-12. This is the order in which it tries these permutations:

1. year/month/day
2. year/day/month
3. month/day/year
4. day/month/year

The potential unit separators are `/`, `.`, and `-`. Whitespace is optional.

# Timezones

At the moment `two_timer` only produces "naive" times. Sorry about that.

*/

#![recursion_limit = "1024"]
#[macro_use]
extern crate pidgin;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate serde_json;
use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::{Datelike, Duration, Local, Timelike, Weekday};
use pidgin::{Grammar, Match, Matcher};
use regex::Regex;

lazy_static! {
    // making this public is useful for testing, but best to keep it hidden to
    // limit complexity and commitment
    #[doc(hidden)]
    pub static ref GRAMMAR: Grammar = grammar!{
        (?ibBw)

        TOP -> r(r"\A") <time_expression> r(r"\z")

        // non-terminal patterns
        // these are roughly ordered by dependency

        time_expression => <universal> | <particular>

        particular => <one_time> | <two_times>

        one_time => <moment_or_period>

        two_times -> ("from")? <moment_or_period> <to> <moment_or_period>

        to => <up_to> | <through>

        moment_or_period => <moment> | <period>

        period => <named_period> | <specific_period>

        specific_period => <modified_period> | <month_and_year> | <year> | <relative_period>

        modified_period -> <modifier> <modifiable_period>

        modifiable_period => [["week", "month", "year", "pay period", "pp", "weekend"]] | <a_month> | <a_day>

        month_and_year -> <a_month> <year>

        year => <short_year> | ("-")? <n_year>
        year -> <suffix_year> <year_suffix>

        year_suffix => <ce> | <bce>

        relative_period -> <count> <displacement> <from_now_or_ago>

        count => r(r"[1-9][0-9]*") | <a_count>

        named_period => <a_day> | <a_month>

        moment -> <adjustment>? <point_in_time>

        adjustment -> <amount> <direction> // two minutes before

        amount -> <count> <unit>

        point_in_time -> <at_time_on>? <some_day> <at_time>? | <specific_time> | <time>

        at_time_on -> ("at")? <time> ("on")?

        some_day => <specific_day> | <relative_day>

        specific_day => <adverb> | <date_with_year>

        date_with_year => <n_date> | <a_date>

        n_date -> <year>    r("[./-]") <n_month> r("[./-]") <n_day>
        n_date -> <year>    r("[./-]") <n_day>   r("[./-]") <n_month>
        n_date -> <n_month> r("[./-]") <n_day>   r("[./-]") <year>
        n_date -> <n_day>   r("[./-]") <n_month> r("[./-]") <year>

        a_date -> <day_prefix>? <a_month> <o_n_day> (",") <year>
        a_date -> <day_prefix>? <n_day> <a_month> <year>
        a_date -> <day_prefix>? ("the") <o_day> ("of") <a_month> <year>

        day_prefix => <a_day> (",")?

        relative_day => <a_day> | <a_day_in_month>

        at_time -> ("at") <time>

        specific_time => <first_time> | <last_time> | <precise_time>

        precise_time -> <n_date> <hour_24>

        time -> <hour_12> <am_pm>? | <hour_24> | <named_time>

        hour_12 => <h12>
        hour_12 => <h12> (":") <minute>
        hour_12 => <h12> (":") <minute> (":") <second>

        hour_24 => <h24>
        hour_24 => <h24> (":") <minute>
        hour_24 => <h24> (":") <minute> (":") <second>

        a_day_in_month => <ordinal_day> | <day_and_month>

        ordinal_day   -> <day_prefix>? ("the") <o_day>    // the first

        o_day => <n_ordinal> | <a_ordinal> | <roman>

        day_and_month -> <n_month> r("[./-]") <n_day>     // 5-6
        day_and_month -> <a_month> <o_n_day>              // June 5, June 5th, June fifth
        day_and_month -> ("the") <o_day> ("of") <a_month> // the 5th of June, the fifth of June

        o_n_day => <n_day> | <o_day>

        // terminal patterns
        // these are organized into single-line and multi-line patterns, with each group alphabetized

        // various phrases all meaning from the first measurable moment to the last
        a_count         => [["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]]
        adverb          => [["now", "today", "tomorrow", "yesterday"]]
        am_pm           => (?-ib) [["am", "AM", "pm", "PM", "a.m.", "A.M.", "p.m.", "P.M."]]
        bce             => (?-ib) [["bce", "b.c.e.", "bc", "b.c.", "BCE", "B.C.E.", "BC", "B.C."]]
        ce              => (?-ib) [["ce", "c.e.", "ad", "a.d.", "CE", "C.E.", "AD", "A.D."]]
        direction       -> [["before", "after", "around", "before and after"]]
        displacement    => [["week", "day", "hour", "minute", "second"]] ("s")?   // not handling variable-width periods like months or years
        from_now_or_ago => [["from now", "ago"]]
        h12             => (?-B) [(1..=12).into_iter().collect::<Vec<_>>()]
        h24             => [(1..=24).into_iter().collect::<Vec<_>>()]
        minute          => (?-B) [ (0..60).into_iter().map(|i| format!("{:02}", i)).collect::<Vec<_>>() ]
        modifier        => [["this", "last", "next"]]
        named_time      => [["noon", "midnight"]]
        n_year          => r(r"\b(?:[1-9][0-9]{0,4}|0)\b")
        roman           => [["nones", "ides", "kalends"]]
        unit            => [["week", "day", "hour", "minute", "second"]] ("s")?
        universal       => [["always", "ever", "all time", "forever", "from beginning to end", "from the beginning to the end"]]
        up_to           => [["to", "until", "up to", "till"]]
        second          => (?-B) [ (0..60).into_iter().map(|i| format!("{:02}", i)).collect::<Vec<_>>() ]
        suffix_year     => r(r"\b[1-9][0-9]{0,4}")
        through         => [["up through", "through", "thru"]] | r("-+")

        a_day => (?-i) [["M", "T", "W", "R", "F", "S", "U"]]
        a_day => [
                "Sunday Monday Tuesday Wednesday Thursday Friday Saturday Tues Weds Thurs Tues. Weds. Thurs."
                    .split(" ")
                    .into_iter()
                    .flat_map(|w| vec![
                        w.to_string(),
                        w[0..2].to_string(),
                        w[0..3].to_string(),
                        format!("{}.", w[0..2].to_string()),
                        format!("{}.", w[0..3].to_string()),
                    ])
                    .collect::<Vec<_>>()
            ]
        a_month => [
                "January February March April May June July August September October November December"
                     .split(" ")
                     .into_iter()
                     .flat_map(|w| vec![w.to_string(), w[0..3].to_string()])
                     .collect::<Vec<_>>()
            ]
        a_ordinal => [[
                "first",
                "second",
                "third",
                "fourth",
                "fifth",
                "sixth",
                "seventh",
                "eighth",
                "ninth",
                "tenth",
                "eleventh",
                "twelfth",
                "thirteenth",
                "fourteenth",
                "fifteenth",
                "sixteenth",
                "seventeenth",
                "eighteenth",
                "nineteenth",
                "twentieth",
                "twenty-first",
                "twenty-second",
                "twenty-third",
                "twenty-fourth",
                "twenty-fifth",
                "twenty-sixth",
                "twenty-seventh",
                "twenty-eighth",
                "twenty-ninth",
                "thirtieth",
                "thirty-first"
            ]]
        first_time => [[
                "the beginning",
                "the beginning of time",
                "the first moment",
                "the start",
                "the very start",
                "the first instant",
                "the dawn of time",
                "the big bang",
                "the birth of the universe",
            ]]
        last_time => [[
                "the end",
                "the end of time",
                "the very end",
                "the last moment",
                "eternity",
                "infinity",
                "doomsday",
                "the crack of doom",
                "armageddon",
                "ragnarok",
                "the big crunch",
                "the heat death of the universe",
                "doom",
                "death",
                "perdition",
                "the last hurrah",
                "ever after",
                "the last syllable of recorded time",
            ]]
        n_day => [
                (1..=31)
                    .into_iter()
                    .flat_map(|i| vec![i.to_string(), format!("{:02}", i)])
                    .collect::<Vec<_>>()
            ]
        n_month => [
                (1..12)
                    .into_iter()
                    .flat_map(|i| vec![format!("{:02}", i), format!("{}", i)])
                    .collect::<Vec<_>>()
            ]
        n_ordinal => [[
                "1st",
                "2nd",
                "3rd",
                "4th",
                "5th",
                "6th",
                "7th",
                "8th",
                "9th",
                "10th",
                "11th",
                "12th",
                "13th",
                "14th",
                "15th",
                "16th",
                "17th",
                "18th",
                "19th",
                "20th",
                "21st",
                "22nd",
                "23rd",
                "24th",
                "25th",
                "26th",
                "27th",
                "28th",
                "29th",
                "30th",
                "31st",
            ]]
        short_year => [
                (0..=99)
                    .into_iter()
                    .flat_map(|i| vec![format!("'{:02}", i), format!("{:02}", i)])
                    .collect::<Vec<_>>()
            ]
    };
}
// code generated via cargo run --bin serializer
// this saves the cost of generating GRAMMAR
lazy_static! {
    #[doc(hidden)]
    pub static ref MATCHER: Matcher = serde_json::from_str(r#"{"rx":"(?P<m0>(?i:\\A\\s*(?P<m1>(?:(?P<m2>\\b(?:ever|al(?:ways|l(?:\\s+)time)|f(?:orever|rom(?:\\s+)(?:beginning(?:\\s+)to|the(?:\\s+)beginning(?:\\s+)to(?:\\s+)the)(?:\\s+)end))\\b)|(?P<m3>(?:(?P<m4>(?P<m5>(?:(?P<m6>(?:\\s*(?P<m7>(?P<m8>(?P<m9>(?:[1-9][0-9]*|(?P<m10>\\b(?:one|nine|eight|f(?:ive|our)|s(?:ix|even)|t(?:en|wo|hree))\\b)))\\s*(?P<m11>\\b(?:day|hour|week|minute|second)(?:s\\b)?))\\s*(?P<m12>\\b(?:a(?:fter|round)|before(:?(?:\\s+)and(?:\\s+)after)?)\\b)))?\\s*(?P<m13>(?:(?:\\s*(?P<m14>(?:\\s*\\bat)?\\s*(?P<m15>(?:(?P<m16>(?:(?P<m17>\\b(?:[2-9]|1[0-2]?))|(?P<m18>\\b(?:[2-9]|1[0-2]?)):(?P<m19>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m20>\\b(?:[2-9]|1[0-2]?)):(?P<m21>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m22>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m23>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m24>(?:(?P<m25>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m26>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m27>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m28>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m29>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m30>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m31>\\b(?:noon|midnight)\\b)))(?:\\s*on\\b)?))?\\s*(?P<m32>(?:(?P<m33>(?:(?P<m34>\\b(?:now|yesterday|to(?:day|morrow))\\b)|(?P<m35>(?:(?P<m36>(?:(?P<m37>(?:(?P<m38>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m39>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m40>\\b[1-9][0-9]{0,4})\\s*(?P<m41>(?:(?P<m42>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m43>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m44>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m45>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m46>(?:(?P<m47>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m48>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m49>\\b[1-9][0-9]{0,4})\\s*(?P<m50>(?:(?P<m51>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m52>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m53>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m54>\\b(?:[2-9]|1[01]?|0[1-9])\\b)|(?P<m55>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m56>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m57>(?:(?P<m58>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m59>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m60>\\b[1-9][0-9]{0,4})\\s*(?P<m61>(?:(?P<m62>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m63>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m64>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m65>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m66>(?:(?P<m67>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m68>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m69>\\b[1-9][0-9]{0,4})\\s*(?P<m70>(?:(?P<m71>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m72>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))|(?P<m73>(?:(?:\\s*(?P<m74>(?P<m75>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*(?P<m76>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m77>(?:(?P<m78>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m79>(?:(?P<m80>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m81>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m82>\\b(?:ide|none|kalend)s\\b)))))\\s*,\\s*(?P<m83>(?:(?P<m84>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m85>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m86>\\b[1-9][0-9]{0,4})\\s*(?P<m87>(?:(?P<m88>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m89>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?:\\s*(?P<m90>(?P<m91>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*(?P<m92>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*(?P<m93>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m94>(?:(?P<m95>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m96>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m97>\\b[1-9][0-9]{0,4})\\s*(?P<m98>(?:(?P<m99>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m100>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?:\\s*(?P<m101>(?P<m102>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*the\\s*(?P<m103>(?:(?P<m104>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m105>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m106>\\b(?:ide|none|kalend)s\\b)))\\s*of\\s*(?P<m107>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m108>(?:(?P<m109>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m110>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m111>\\b[1-9][0-9]{0,4})\\s*(?P<m112>(?:(?P<m113>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m114>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))))))|(?P<m115>(?:(?P<m116>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))|(?P<m117>(?:(?P<m118>(?:\\s*(?P<m119>(?P<m120>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*the\\s*(?P<m121>(?:(?P<m122>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m123>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m124>\\b(?:ide|none|kalend)s\\b))))|(?P<m125>(?:(?P<m126>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m127>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m128>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m129>(?:(?P<m130>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m131>(?:(?P<m132>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m133>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m134>\\b(?:ide|none|kalend)s\\b)))))|\\bthe\\s*(?P<m135>(?:(?P<m136>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m137>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m138>\\b(?:ide|none|kalend)s\\b)))\\s*of\\s*(?P<m139>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)))))))))(?:\\s*(?P<m140>\\bat\\s*(?P<m141>(?:(?P<m142>(?:(?P<m143>\\b(?:[2-9]|1[0-2]?))|(?P<m144>\\b(?:[2-9]|1[0-2]?)):(?P<m145>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m146>\\b(?:[2-9]|1[0-2]?)):(?P<m147>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m148>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m149>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m150>(?:(?P<m151>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m152>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m153>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m154>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m155>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m156>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m157>\\b(?:noon|midnight)\\b)))))?|(?P<m158>(?:(?P<m159>\\bthe(?:\\s+)(?:start|very(?:\\s+)start|first(?:\\s+)(?:mome|insta)nt|dawn(?:\\s+)of(?:\\s+)time|b(?:eginning(:?(?:\\s+)of(?:\\s+)time)?|i(?:g(?:\\s+)bang|rth(?:\\s+)of(?:\\s+)the(?:\\s+)universe)))\\b)|(?P<m160>\\b(?:infinity|ragnarok|perdition|armageddon|d(?:eath|oom(:?sday)?)|e(?:ternity|ver(?:\\s+)after)|the(?:\\s+)(?:very(?:\\s+)end|big(?:\\s+)crunch|end(:?(?:\\s+)of(?:\\s+)time)?|crack(?:\\s+)of(?:\\s+)doom|heat(?:\\s+)death(?:\\s+)of(?:\\s+)the(?:\\s+)universe|last(?:\\s+)(?:hurrah|moment|syllable(?:\\s+)of(?:\\s+)recorded(?:\\s+)time)))\\b)|(?P<m161>(?P<m162>(?:(?P<m163>(?:(?P<m164>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m165>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m166>\\b[1-9][0-9]{0,4})\\s*(?P<m167>(?:(?P<m168>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m169>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m170>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m171>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m172>(?:(?P<m173>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m174>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m175>\\b[1-9][0-9]{0,4})\\s*(?P<m176>(?:(?P<m177>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m178>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m179>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m180>\\b(?:[2-9]|1[01]?|0[1-9])\\b)|(?P<m181>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m182>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m183>(?:(?P<m184>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m185>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m186>\\b[1-9][0-9]{0,4})\\s*(?P<m187>(?:(?P<m188>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m189>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m190>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m191>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m192>(?:(?P<m193>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m194>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m195>\\b[1-9][0-9]{0,4})\\s*(?P<m196>(?:(?P<m197>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m198>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))\\s*(?P<m199>(?:(?P<m200>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m201>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m202>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m203>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m204>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m205>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])))))))|(?P<m206>(?:(?P<m207>(?:(?P<m208>\\b(?:[2-9]|1[0-2]?))|(?P<m209>\\b(?:[2-9]|1[0-2]?)):(?P<m210>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m211>\\b(?:[2-9]|1[0-2]?)):(?P<m212>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m213>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m214>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m215>(?:(?P<m216>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m217>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m218>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m219>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m220>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m221>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m222>\\b(?:noon|midnight)\\b))))))|(?P<m223>(?:(?P<m224>(?:(?P<m225>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))|(?P<m226>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)))|(?P<m227>(?:(?P<m228>(?P<m229>\\b(?:last|next|this)\\b)\\s*(?P<m230>(?:\\b(?:year|month|week(:?end)?|p(?:p|ay(?:\\s+)period))\\b|(?P<m231>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)|(?P<m232>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b))))))))))|(?P<m233>(?P<m234>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m235>(?:(?P<m236>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m237>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m238>\\b[1-9][0-9]{0,4})\\s*(?P<m239>(?:(?P<m240>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m241>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b)))))))))|(?P<m242>(?:(?P<m243>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m244>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m245>\\b[1-9][0-9]{0,4})\\s*(?P<m246>(?:(?P<m247>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m248>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m249>(?P<m250>(?:[1-9][0-9]*|(?P<m251>\\b(?:one|nine|eight|f(?:ive|our)|s(?:ix|even)|t(?:en|wo|hree))\\b)))\\s*(?P<m252>\\b(?:day|hour|week|minute|second)(?:s\\b)?)\\s*(?P<m253>\\b(?:ago|from(?:\\s+)now)\\b)))))))))|(?P<m254>(?:\\s*\\bfrom)?\\s*(?P<m255>(?:(?P<m256>(?:\\s*(?P<m257>(?P<m258>(?P<m259>(?:[1-9][0-9]*|(?P<m260>\\b(?:one|nine|eight|f(?:ive|our)|s(?:ix|even)|t(?:en|wo|hree))\\b)))\\s*(?P<m261>\\b(?:day|hour|week|minute|second)(?:s\\b)?))\\s*(?P<m262>\\b(?:a(?:fter|round)|before(:?(?:\\s+)and(?:\\s+)after)?)\\b)))?\\s*(?P<m263>(?:(?:\\s*(?P<m264>(?:\\s*\\bat)?\\s*(?P<m265>(?:(?P<m266>(?:(?P<m267>\\b(?:[2-9]|1[0-2]?))|(?P<m268>\\b(?:[2-9]|1[0-2]?)):(?P<m269>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m270>\\b(?:[2-9]|1[0-2]?)):(?P<m271>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m272>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m273>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m274>(?:(?P<m275>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m276>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m277>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m278>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m279>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m280>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m281>\\b(?:noon|midnight)\\b)))(?:\\s*on\\b)?))?\\s*(?P<m282>(?:(?P<m283>(?:(?P<m284>\\b(?:now|yesterday|to(?:day|morrow))\\b)|(?P<m285>(?:(?P<m286>(?:(?P<m287>(?:(?P<m288>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m289>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m290>\\b[1-9][0-9]{0,4})\\s*(?P<m291>(?:(?P<m292>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m293>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m294>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m295>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m296>(?:(?P<m297>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m298>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m299>\\b[1-9][0-9]{0,4})\\s*(?P<m300>(?:(?P<m301>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m302>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m303>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m304>\\b(?:[2-9]|1[01]?|0[1-9])\\b)|(?P<m305>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m306>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m307>(?:(?P<m308>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m309>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m310>\\b[1-9][0-9]{0,4})\\s*(?P<m311>(?:(?P<m312>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m313>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m314>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m315>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m316>(?:(?P<m317>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m318>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m319>\\b[1-9][0-9]{0,4})\\s*(?P<m320>(?:(?P<m321>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m322>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))|(?P<m323>(?:(?:\\s*(?P<m324>(?P<m325>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*(?P<m326>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m327>(?:(?P<m328>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m329>(?:(?P<m330>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m331>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m332>\\b(?:ide|none|kalend)s\\b)))))\\s*,\\s*(?P<m333>(?:(?P<m334>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m335>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m336>\\b[1-9][0-9]{0,4})\\s*(?P<m337>(?:(?P<m338>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m339>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?:\\s*(?P<m340>(?P<m341>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*(?P<m342>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*(?P<m343>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m344>(?:(?P<m345>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m346>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m347>\\b[1-9][0-9]{0,4})\\s*(?P<m348>(?:(?P<m349>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m350>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?:\\s*(?P<m351>(?P<m352>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*the\\s*(?P<m353>(?:(?P<m354>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m355>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m356>\\b(?:ide|none|kalend)s\\b)))\\s*of\\s*(?P<m357>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m358>(?:(?P<m359>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m360>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m361>\\b[1-9][0-9]{0,4})\\s*(?P<m362>(?:(?P<m363>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m364>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))))))|(?P<m365>(?:(?P<m366>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))|(?P<m367>(?:(?P<m368>(?:\\s*(?P<m369>(?P<m370>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*the\\s*(?P<m371>(?:(?P<m372>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m373>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m374>\\b(?:ide|none|kalend)s\\b))))|(?P<m375>(?:(?P<m376>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m377>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m378>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m379>(?:(?P<m380>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m381>(?:(?P<m382>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m383>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m384>\\b(?:ide|none|kalend)s\\b)))))|\\bthe\\s*(?P<m385>(?:(?P<m386>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m387>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m388>\\b(?:ide|none|kalend)s\\b)))\\s*of\\s*(?P<m389>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)))))))))(?:\\s*(?P<m390>\\bat\\s*(?P<m391>(?:(?P<m392>(?:(?P<m393>\\b(?:[2-9]|1[0-2]?))|(?P<m394>\\b(?:[2-9]|1[0-2]?)):(?P<m395>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m396>\\b(?:[2-9]|1[0-2]?)):(?P<m397>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m398>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m399>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m400>(?:(?P<m401>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m402>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m403>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m404>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m405>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m406>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m407>\\b(?:noon|midnight)\\b)))))?|(?P<m408>(?:(?P<m409>\\bthe(?:\\s+)(?:start|very(?:\\s+)start|first(?:\\s+)(?:mome|insta)nt|dawn(?:\\s+)of(?:\\s+)time|b(?:eginning(:?(?:\\s+)of(?:\\s+)time)?|i(?:g(?:\\s+)bang|rth(?:\\s+)of(?:\\s+)the(?:\\s+)universe)))\\b)|(?P<m410>\\b(?:infinity|ragnarok|perdition|armageddon|d(?:eath|oom(:?sday)?)|e(?:ternity|ver(?:\\s+)after)|the(?:\\s+)(?:very(?:\\s+)end|big(?:\\s+)crunch|end(:?(?:\\s+)of(?:\\s+)time)?|crack(?:\\s+)of(?:\\s+)doom|heat(?:\\s+)death(?:\\s+)of(?:\\s+)the(?:\\s+)universe|last(?:\\s+)(?:hurrah|moment|syllable(?:\\s+)of(?:\\s+)recorded(?:\\s+)time)))\\b)|(?P<m411>(?P<m412>(?:(?P<m413>(?:(?P<m414>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m415>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m416>\\b[1-9][0-9]{0,4})\\s*(?P<m417>(?:(?P<m418>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m419>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m420>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m421>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m422>(?:(?P<m423>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m424>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m425>\\b[1-9][0-9]{0,4})\\s*(?P<m426>(?:(?P<m427>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m428>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m429>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m430>\\b(?:[2-9]|1[01]?|0[1-9])\\b)|(?P<m431>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m432>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m433>(?:(?P<m434>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m435>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m436>\\b[1-9][0-9]{0,4})\\s*(?P<m437>(?:(?P<m438>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m439>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m440>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m441>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m442>(?:(?P<m443>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m444>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m445>\\b[1-9][0-9]{0,4})\\s*(?P<m446>(?:(?P<m447>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m448>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))\\s*(?P<m449>(?:(?P<m450>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m451>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m452>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m453>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m454>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m455>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])))))))|(?P<m456>(?:(?P<m457>(?:(?P<m458>\\b(?:[2-9]|1[0-2]?))|(?P<m459>\\b(?:[2-9]|1[0-2]?)):(?P<m460>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m461>\\b(?:[2-9]|1[0-2]?)):(?P<m462>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m463>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m464>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m465>(?:(?P<m466>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m467>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m468>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m469>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m470>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m471>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m472>\\b(?:noon|midnight)\\b))))))|(?P<m473>(?:(?P<m474>(?:(?P<m475>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))|(?P<m476>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)))|(?P<m477>(?:(?P<m478>(?P<m479>\\b(?:last|next|this)\\b)\\s*(?P<m480>(?:\\b(?:year|month|week(:?end)?|p(?:p|ay(?:\\s+)period))\\b|(?P<m481>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)|(?P<m482>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b))))))))))|(?P<m483>(?P<m484>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m485>(?:(?P<m486>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m487>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m488>\\b[1-9][0-9]{0,4})\\s*(?P<m489>(?:(?P<m490>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m491>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b)))))))))|(?P<m492>(?:(?P<m493>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m494>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m495>\\b[1-9][0-9]{0,4})\\s*(?P<m496>(?:(?P<m497>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m498>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m499>(?P<m500>(?:[1-9][0-9]*|(?P<m501>\\b(?:one|nine|eight|f(?:ive|our)|s(?:ix|even)|t(?:en|wo|hree))\\b)))\\s*(?P<m502>\\b(?:day|hour|week|minute|second)(?:s\\b)?)\\s*(?P<m503>\\b(?:ago|from(?:\\s+)now)\\b))))))))\\s*(?P<m504>(?:(?P<m505>\\b(?:t(?:o|ill)|u(?:ntil|p(?:\\s+)to))\\b)|(?P<m506>(?:\\b(?:thr(?:u|ough)|up(?:\\s+)through)\\b|-+))))\\s*(?P<m507>(?:(?P<m508>(?:\\s*(?P<m509>(?P<m510>(?P<m511>(?:[1-9][0-9]*|(?P<m512>\\b(?:one|nine|eight|f(?:ive|our)|s(?:ix|even)|t(?:en|wo|hree))\\b)))\\s*(?P<m513>\\b(?:day|hour|week|minute|second)(?:s\\b)?))\\s*(?P<m514>\\b(?:a(?:fter|round)|before(:?(?:\\s+)and(?:\\s+)after)?)\\b)))?\\s*(?P<m515>(?:(?:\\s*(?P<m516>(?:\\s*\\bat)?\\s*(?P<m517>(?:(?P<m518>(?:(?P<m519>\\b(?:[2-9]|1[0-2]?))|(?P<m520>\\b(?:[2-9]|1[0-2]?)):(?P<m521>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m522>\\b(?:[2-9]|1[0-2]?)):(?P<m523>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m524>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m525>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m526>(?:(?P<m527>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m528>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m529>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m530>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m531>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m532>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m533>\\b(?:noon|midnight)\\b)))(?:\\s*on\\b)?))?\\s*(?P<m534>(?:(?P<m535>(?:(?P<m536>\\b(?:now|yesterday|to(?:day|morrow))\\b)|(?P<m537>(?:(?P<m538>(?:(?P<m539>(?:(?P<m540>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m541>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m542>\\b[1-9][0-9]{0,4})\\s*(?P<m543>(?:(?P<m544>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m545>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m546>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m547>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m548>(?:(?P<m549>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m550>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m551>\\b[1-9][0-9]{0,4})\\s*(?P<m552>(?:(?P<m553>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m554>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m555>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m556>\\b(?:[2-9]|1[01]?|0[1-9])\\b)|(?P<m557>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m558>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m559>(?:(?P<m560>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m561>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m562>\\b[1-9][0-9]{0,4})\\s*(?P<m563>(?:(?P<m564>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m565>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m566>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m567>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m568>(?:(?P<m569>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m570>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m571>\\b[1-9][0-9]{0,4})\\s*(?P<m572>(?:(?P<m573>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m574>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))|(?P<m575>(?:(?:\\s*(?P<m576>(?P<m577>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*(?P<m578>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m579>(?:(?P<m580>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m581>(?:(?P<m582>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m583>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m584>\\b(?:ide|none|kalend)s\\b)))))\\s*,\\s*(?P<m585>(?:(?P<m586>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m587>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m588>\\b[1-9][0-9]{0,4})\\s*(?P<m589>(?:(?P<m590>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m591>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?:\\s*(?P<m592>(?P<m593>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*(?P<m594>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*(?P<m595>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m596>(?:(?P<m597>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m598>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m599>\\b[1-9][0-9]{0,4})\\s*(?P<m600>(?:(?P<m601>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m602>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?:\\s*(?P<m603>(?P<m604>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*the\\s*(?P<m605>(?:(?P<m606>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m607>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m608>\\b(?:ide|none|kalend)s\\b)))\\s*of\\s*(?P<m609>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m610>(?:(?P<m611>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m612>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m613>\\b[1-9][0-9]{0,4})\\s*(?P<m614>(?:(?P<m615>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m616>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))))))|(?P<m617>(?:(?P<m618>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))|(?P<m619>(?:(?P<m620>(?:\\s*(?P<m621>(?P<m622>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))(?:,)?))?\\s*the\\s*(?P<m623>(?:(?P<m624>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m625>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m626>\\b(?:ide|none|kalend)s\\b))))|(?P<m627>(?:(?P<m628>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m629>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m630>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m631>(?:(?P<m632>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m633>(?:(?P<m634>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m635>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m636>\\b(?:ide|none|kalend)s\\b)))))|\\bthe\\s*(?P<m637>(?:(?P<m638>\\b(?:4th|5th|6th|7th|8th|9th|3(?:rd|0th|1st)|1(?:st|0th|1th|2th|3th|4th|5th|6th|7th|8th|9th)|2(?:nd|0th|1st|2nd|3rd|4th|5th|6th|7th|8th|9th))\\b)|(?P<m639>\\b(?:nin(:?eteen)?th|e(?:leven|igh(:?teen)?)th|f(?:ourt(:?eent)?h|i(?:rst|ft(:?eent)?h))|s(?:ixt(:?eent)?h|e(?:cond|vent(:?eent)?h))|t(?:enth|hir(?:d|t(?:ieth|eenth|y\\-first))|we(?:lfth|nt(?:ieth|y\\-(?:ninth|third|eighth|f(?:ourth|i(?:fth|rst))|s(?:ixth|e(?:cond|venth)))))))\\b)|(?P<m640>\\b(?:ide|none|kalend)s\\b)))\\s*of\\s*(?P<m641>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)))))))))(?:\\s*(?P<m642>\\bat\\s*(?P<m643>(?:(?P<m644>(?:(?P<m645>\\b(?:[2-9]|1[0-2]?))|(?P<m646>\\b(?:[2-9]|1[0-2]?)):(?P<m647>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m648>\\b(?:[2-9]|1[0-2]?)):(?P<m649>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m650>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m651>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m652>(?:(?P<m653>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m654>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m655>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m656>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m657>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m658>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m659>\\b(?:noon|midnight)\\b)))))?|(?P<m660>(?:(?P<m661>\\bthe(?:\\s+)(?:start|very(?:\\s+)start|first(?:\\s+)(?:mome|insta)nt|dawn(?:\\s+)of(?:\\s+)time|b(?:eginning(:?(?:\\s+)of(?:\\s+)time)?|i(?:g(?:\\s+)bang|rth(?:\\s+)of(?:\\s+)the(?:\\s+)universe)))\\b)|(?P<m662>\\b(?:infinity|ragnarok|perdition|armageddon|d(?:eath|oom(:?sday)?)|e(?:ternity|ver(?:\\s+)after)|the(?:\\s+)(?:very(?:\\s+)end|big(?:\\s+)crunch|end(:?(?:\\s+)of(?:\\s+)time)?|crack(?:\\s+)of(?:\\s+)doom|heat(?:\\s+)death(?:\\s+)of(?:\\s+)the(?:\\s+)universe|last(?:\\s+)(?:hurrah|moment|syllable(?:\\s+)of(?:\\s+)recorded(?:\\s+)time)))\\b)|(?P<m663>(?P<m664>(?:(?P<m665>(?:(?P<m666>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m667>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m668>\\b[1-9][0-9]{0,4})\\s*(?P<m669>(?:(?P<m670>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m671>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m672>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m673>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)|(?P<m674>(?:(?P<m675>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m676>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m677>\\b[1-9][0-9]{0,4})\\s*(?P<m678>(?:(?P<m679>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m680>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))\\s*[./-]\\s*(?P<m681>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m682>\\b(?:[2-9]|1[01]?|0[1-9])\\b)|(?P<m683>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m684>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m685>(?:(?P<m686>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m687>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m688>\\b[1-9][0-9]{0,4})\\s*(?P<m689>(?:(?P<m690>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m691>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m692>\\b(?:[4-9]|3[01]?|0[1-9]|1[0-9]?|2[0-9]?)\\b)\\s*[./-]\\s*(?P<m693>\\b(?:[2-9]|1[01]?|0[1-9])\\b)\\s*[./-]\\s*(?P<m694>(?:(?P<m695>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m696>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m697>\\b[1-9][0-9]{0,4})\\s*(?P<m698>(?:(?P<m699>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m700>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))))\\s*(?P<m701>(?:(?P<m702>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m703>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m704>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m705>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m706>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m707>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])))))))|(?P<m708>(?:(?P<m709>(?:(?P<m710>\\b(?:[2-9]|1[0-2]?))|(?P<m711>\\b(?:[2-9]|1[0-2]?)):(?P<m712>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m713>\\b(?:[2-9]|1[0-2]?)):(?P<m714>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m715>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))(?-i:\\s*(?P<m716>(?:A(?:\\.M\\.|M\\b)|P(?:\\.M\\.|M\\b)|a(?:\\.m\\.|m\\b)|p(?:\\.m\\.|m\\b))))?|(?P<m717>(?:(?P<m718>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b)|(?P<m719>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m720>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))|(?P<m721>\\b(?:[3-9]|1[0-9]?|2[0-4]?)\\b):(?P<m722>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9])):(?P<m723>\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]))))|(?P<m724>\\b(?:noon|midnight)\\b))))))|(?P<m725>(?:(?P<m726>(?:(?P<m727>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b)))))))|(?P<m728>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)))|(?P<m729>(?:(?P<m730>(?P<m731>\\b(?:last|next|this)\\b)\\s*(?P<m732>(?:\\b(?:year|month|week(:?end)?|p(?:p|ay(?:\\s+)period))\\b|(?P<m733>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)|(?P<m734>(?:(?-i:\\b[FMR-UW]\\b)|\\b(?:Fr(?:\\.|\\b|i(?:\\.|\\b|day\\b))|Mo(?:\\.|\\b|n(?:\\.|\\b|day\\b))|We(?:\\.|\\b|d(?:\\.|\\b|s(?:\\.|\\b)|nesday\\b))|S(?:u(?:\\.|\\b|n(?:\\.|\\b|day\\b))|a(?:\\.|\\b|t(?:\\.|\\b|urday\\b)))|T(?:u(?:\\.|\\b|e(?:\\.|\\b|s(?:\\.|\\b|day\\b)))|h(?:\\.|\\b|u(?:\\.|\\b|rs(?:\\.|\\b|day\\b))))))))))|(?P<m735>(?P<m736>\\b(?:Ma(?:y|r(:?ch)?)|Oct(:?ober)?|Dec(:?ember)?|Feb(:?ruary)?|Nov(:?ember)?|Sep(:?tember)?|A(?:pr(:?il)?|ug(:?ust)?)|J(?:u(?:ly?|ne?)|an(:?uary)?))\\b)\\s*(?P<m737>(?:(?P<m738>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m739>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m740>\\b[1-9][0-9]{0,4})\\s*(?P<m741>(?:(?P<m742>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m743>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b)))))))))|(?P<m744>(?:(?P<m745>(?:'(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9])|\\b(?:0[0-9]|1[0-9]|2[0-9]|3[0-9]|4[0-9]|5[0-9]|6[0-9]|7[0-9]|8[0-9]|9[0-9]))\\b)|(?:\\-)?(?P<m746>\\b(?:[1-9][0-9]{0,4}|0)\\b)|(?P<m747>\\b[1-9][0-9]{0,4})\\s*(?P<m748>(?:(?P<m749>(?-i:(?:A(?:\\.D\\.|D\\b)|C(?:\\.E\\.|E\\b)|a(?:\\.d\\.|d\\b)|c(?:\\.e\\.|e\\b))))|(?P<m750>(?-i:(?:B(?:\\.C\\.(:?E\\.)?|CE?\\b)|b(?:\\.c\\.(:?e\\.)?|ce?\\b))))))))|(?P<m751>(?P<m752>(?:[1-9][0-9]*|(?P<m753>\\b(?:one|nine|eight|f(?:ive|our)|s(?:ix|even)|t(?:en|wo|hree))\\b)))\\s*(?P<m754>\\b(?:day|hour|week|minute|second)(?:s\\b)?)\\s*(?P<m755>\\b(?:ago|from(?:\\s+)now)\\b)))))))))))))\\s*\\z))","root":"TOP","translation":{"m273":"am_pm","m594":"n_day","m202":"minute","m575":"a_date","m157":"named_time","m152":"h24","m319":"suffix_year","m538":"n_date","m95":"short_year","m261":"unit","m586":"short_year","m371":"o_day","m709":"hour_12","m79":"o_day","m270":"h12","m0":"TOP","m324":"day_prefix","m10":"a_count","m291":"year_suffix","m312":"ce","m354":"n_ordinal","m591":"bce","m455":"second","m429":"n_day","m486":"short_year","m430":"n_month","m564":"ce","m162":"n_date","m283":"specific_day","m308":"short_year","m413":"year","m585":"year","m668":"suffix_year","m754":"displacement","m421":"n_day","m464":"am_pm","m28":"h24","m320":"year_suffix","m203":"h24","m183":"year","m481":"a_month","m534":"some_day","m658":"second","m349":"ce","m584":"roman","m204":"minute","m86":"suffix_year","m422":"year","m177":"ce","m498":"bce","m521":"minute","m545":"bce","m549":"short_year","m734":"a_day","m666":"short_year","m262":"direction","m743":"bce","m165":"n_year","m115":"relative_day","m369":"day_prefix","m406":"second","m29":"minute","m402":"h24","m258":"amount","m4":"one_time","m114":"bce","m316":"year","m523":"minute","m701":"hour_24","m313":"bce","m346":"n_year","m502":"displacement","m532":"second","m603":"day_prefix","m621":"day_prefix","m271":"minute","m527":"h24","m751":"relative_period","m85":"n_year","m150":"hour_24","m296":"year","m730":"modified_period","m383":"a_ordinal","m553":"ce","m21":"minute","m405":"minute","m466":"h24","m557":"n_month","m93":"a_month","m514":"direction","m516":"at_time_on","m223":"period","m412":"n_date","m305":"n_month","m632":"n_day","m355":"a_ordinal","m642":"at_time","m639":"a_ordinal","m451":"h24","m278":"h24","m653":"h24","m727":"a_day","m572":"year_suffix","m353":"o_day","m196":"year_suffix","m562":"suffix_year","m147":"minute","m627":"day_and_month","m677":"suffix_year","m689":"year_suffix","m175":"suffix_year","m357":"a_month","m366":"a_day","m435":"n_year","m616":"bce","m693":"n_month","m419":"bce","m292":"ce","m393":"h12","m719":"h24","m18":"h12","m252":"displacement","m656":"h24","m499":"relative_period","m51":"ce","m663":"precise_time","m345":"short_year","m186":"suffix_year","m2":"universal","m272":"second","m484":"a_month","m487":"n_year","m97":"suffix_year","m501":"a_count","m619":"a_day_in_month","m20":"h12","m662":"last_time","m722":"minute","m22":"second","m65":"n_month","m247":"ce","m415":"n_year","m453":"h24","m476":"a_month","m47":"short_year","m447":"ce","m33":"specific_day","m24":"hour_24","m522":"h12","m645":"h12","m222":"named_time","m127":"n_day","m496":"year_suffix","m390":"at_time","m690":"ce","m178":"bce","m145":"minute","m264":"at_time_on","m703":"h24","m477":"specific_period","m234":"a_month","m78":"n_day","m311":"year_suffix","m31":"named_time","m69":"suffix_year","m597":"short_year","m718":"h24","m399":"am_pm","m394":"h12","m661":"first_time","m163":"year","m182":"n_day","m200":"h24","m540":"short_year","m70":"year_suffix","m126":"n_month","m379":"o_n_day","m290":"suffix_year","m268":"h12","m27":"minute","m542":"suffix_year","m131":"o_day","m336":"suffix_year","m218":"minute","m118":"ordinal_day","m193":"short_year","m503":"from_now_or_ago","m509":"adjustment","m469":"h24","m738":"short_year","m602":"bce","m535":"specific_day","m441":"n_month","m16":"hour_12","m350":"bce","m263":"point_in_time","m307":"year","m729":"specific_period","m125":"day_and_month","m63":"bce","m505":"up_to","m588":"suffix_year","m700":"bce","m708":"time","m172":"year","m675":"short_year","m657":"minute","m332":"roman","m288":"short_year","m524":"second","m391":"time","m445":"suffix_year","m456":"time","m726":"named_period","m279":"minute","m740":"suffix_year","m735":"month_and_year","m442":"year","m141":"time","m652":"hour_24","m520":"h12","m256":"moment","m536":"adverb","m39":"n_year","m407":"named_time","m457":"hour_12","m482":"a_day","m711":"h12","m395":"minute","m333":"year","m103":"o_day","m529":"minute","m448":"bce","m41":"year_suffix","m259":"count","m372":"n_ordinal","m389":"a_month","m176":"year_suffix","m71":"ce","m539":"year","m680":"bce","m58":"short_year","m267":"h12","m217":"h24","m340":"day_prefix","m425":"suffix_year","m483":"month_and_year","m338":"ce","m478":"modified_period","m515":"point_in_time","m52":"bce","m533":"named_time","m559":"year","m228":"modified_period","m397":"minute","m506":"through","m565":"bce","m614":"year_suffix","m655":"minute","m737":"year","m57":"year","m317":"short_year","m589":"year_suffix","m472":"named_time","m206":"time","m89":"bce","m286":"n_date","m42":"ce","m68":"n_year","m624":"n_ordinal","m424":"n_year","m233":"month_and_year","m665":"year","m171":"n_day","m581":"o_day","m232":"a_day","m679":"ce","m748":"year_suffix","m109":"short_year","m434":"short_year","m732":"modifiable_period","m149":"am_pm","m48":"n_year","m465":"hour_24","m556":"n_month","m604":"a_day","m750":"bce","m194":"n_year","m50":"year_suffix","m30":"second","m651":"am_pm","m598":"n_year","m82":"roman","m409":"first_time","m337":"year_suffix","m380":"n_day","m617":"relative_day","m664":"n_date","m339":"bce","m211":"h12","m752":"count","m723":"second","m94":"year","m427":"ce","m108":"year","m219":"h24","m246":"year_suffix","m541":"n_year","m683":"n_month","m643":"time","m547":"n_day","m363":"ce","m728":"a_month","m731":"modifier","m583":"a_ordinal","m517":"time","m44":"n_month","m180":"n_month","m544":"ce","m596":"year","m600":"year_suffix","m439":"bce","m208":"h12","m329":"o_day","m370":"a_day","m660":"specific_time","m111":"suffix_year","m634":"n_ordinal","m7":"adjustment","m136":"n_ordinal","m221":"second","m251":"a_count","m528":"h24","m573":"ce","m595":"a_month","m631":"o_n_day","m570":"n_year","m507":"moment_or_period","m669":"year_suffix","m276":"h24","m304":"n_month","m188":"ce","m230":"modifiable_period","m550":"n_year","m342":"n_day","m88":"ce","m274":"hour_24","m56":"n_day","m495":"suffix_year","m531":"minute","m635":"a_ordinal","m644":"hour_12","m650":"second","m471":"second","m673":"n_day","m130":"n_day","m367":"a_day_in_month","m479":"modifier","m121":"o_day","m444":"n_year","m166":"suffix_year","m77":"o_n_day","m227":"specific_period","m494":"n_year","m315":"n_month","m574":"bce","m314":"n_day","m485":"year","m674":"year","m8":"amount","m681":"n_day","m710":"h12","m736":"a_month","m43":"bce","m480":"modifiable_period","m277":"minute","m146":"h12","m685":"year","m40":"suffix_year","m55":"n_month","m62":"ce","m492":"year","m96":"n_year","m560":"short_year","m530":"h24","m493":"short_year","m512":"a_count","m629":"n_day","m309":"n_year","m626":"roman","m715":"second","m356":"roman","m269":"minute","m348":"year_suffix","m76":"a_month","m98":"year_suffix","m116":"a_day","m73":"a_date","m747":"suffix_year","m14":"at_time_on","m302":"bce","m167":"year_suffix","m184":"short_year","m330":"n_ordinal","m576":"day_prefix","m209":"h12","m198":"bce","m327":"o_n_day","m467":"h24","m543":"year_suffix","m579":"o_n_day","m142":"hour_12","m236":"short_year","m384":"roman","m704":"minute","m446":"year_suffix","m518":"hour_12","m241":"bce","m100":"bce","m688":"suffix_year","m385":"o_day","m187":"year_suffix","m420":"n_month","m255":"moment_or_period","m613":"suffix_year","m440":"n_day","m133":"a_ordinal","m359":"short_year","m318":"n_year","m443":"short_year","m34":"adverb","m250":"count","m6":"moment","m260":"a_count","m377":"n_day","m404":"h24","m300":"year_suffix","m606":"n_ordinal","m249":"relative_period","m468":"minute","m641":"a_month","m671":"bce","m692":"n_day","m706":"minute","m189":"bce","m294":"n_month","m362":"year_suffix","m49":"suffix_year","m335":"n_year","m15":"time","m26":"h24","m622":"a_day","m755":"from_now_or_ago","m458":"h12","m1":"time_expression","m636":"roman","m667":"n_year","m670":"ce","m558":"n_day","m156":"second","m289":"n_year","m721":"h24","m137":"a_ordinal","m450":"h24","m563":"year_suffix","m742":"ce","m224":"named_period","m510":"amount","m554":"bce","m684":"n_day","m128":"a_month","m712":"minute","m475":"a_day","m161":"precise_time","m705":"h24","m387":"a_ordinal","m19":"minute","m17":"h12","m216":"h24","m239":"year_suffix","m378":"a_month","m460":"minute","m628":"n_month","m633":"o_day","m694":"year","m714":"minute","m696":"n_year","m555":"n_day","m513":"unit","m105":"a_ordinal","m611":"short_year","m197":"ce","m205":"second","m388":"roman","m185":"n_year","m408":"specific_time","m229":"modifier","m376":"n_month","m148":"second","m654":"h24","m449":"hour_24","m179":"n_day","m306":"n_day","m328":"n_day","m360":"n_year","m724":"named_time","m672":"n_month","m12":"direction","m129":"o_n_day","m310":"suffix_year","m122":"n_ordinal","m580":"n_day","m352":"a_day","m173":"short_year","m199":"hour_24","m343":"a_month","m321":"ce","m625":"a_ordinal","m170":"n_month","m245":"suffix_year","m326":"a_month","m46":"year","m546":"n_month","m414":"short_year","m238":"suffix_year","m473":"period","m571":"suffix_year","m257":"adjustment","m382":"n_ordinal","m620":"ordinal_day","m686":"short_year","m699":"ce","m282":"some_day","m741":"year_suffix","m210":"minute","m143":"h12","m181":"n_month","m401":"h24","m497":"ce","m159":"first_time","m341":"a_day","m623":"o_day","m87":"year_suffix","m331":"a_ordinal","m438":"ce","m511":"count","m225":"a_day","m470":"minute","m107":"a_month","m717":"hour_24","m214":"am_pm","m593":"a_day","m753":"a_count","m358":"year","m733":"a_month","m301":"ce","m432":"n_day","m240":"ce","m124":"roman","m323":"a_date","m691":"bce","m676":"n_year","m11":"unit","m220":"minute","m351":"day_prefix","m592":"day_prefix","m38":"short_year","m80":"n_ordinal","m113":"ce","m303":"n_day","m297":"short_year","m525":"am_pm","m587":"n_year","m207":"hour_12","m13":"point_in_time","m9":"count","m716":"am_pm","m190":"n_day","m72":"bce","m110":"n_year","m92":"n_day","m322":"bce","m386":"n_ordinal","m361":"suffix_year","m561":"n_year","m609":"a_month","m192":"year","m638":"n_ordinal","m151":"h24","m226":"a_month","m678":"year_suffix","m687":"n_year","m713":"h12","m32":"some_day","m725":"period","m637":"o_day","m695":"short_year","m66":"year","m392":"hour_12","m154":"h24","m601":"ce","m568":"year","m117":"a_day_in_month","m489":"year_suffix","m91":"a_day","m45":"n_day","m81":"a_ordinal","m174":"n_year","m83":"year","m195":"suffix_year","m99":"ce","m242":"year","m381":"o_day","m436":"suffix_year","m452":"minute","m35":"date_with_year","m488":"suffix_year","m153":"minute","m396":"h12","m630":"a_month","m647":"minute","m612":"n_year","m608":"roman","m212":"minute","m53":"n_day","m164":"short_year","m75":"a_day","m295":"n_day","m3":"particular","m23":"am_pm","m104":"n_ordinal","m461":"h12","m298":"n_year","m682":"n_month","m426":"year_suffix","m720":"minute","m243":"short_year","m102":"a_day","m744":"year","m519":"h12","m285":"date_with_year","m403":"minute","m287":"year","m334":"short_year","m235":"year","m135":"o_day","m375":"day_and_month","m168":"ce","m567":"n_month","m707":"second","m84":"short_year","m299":"suffix_year","m215":"hour_24","m428":"bce","m500":"count","m649":"minute","m344":"year","m231":"a_month","m749":"ce","m490":"ce","m508":"moment","m431":"n_month","m138":"roman","m280":"second","m281":"named_time","m410":"last_time","m697":"suffix_year","m347":"suffix_year","m368":"ordinal_day","m284":"adverb","m578":"a_month","m605":"o_day","m112":"year_suffix","m599":"suffix_year","m373":"a_ordinal","m551":"suffix_year","m37":"year","m5":"moment_or_period","m253":"from_now_or_ago","m155":"minute","m54":"n_month","m160":"last_time","m417":"year_suffix","m140":"at_time","m60":"suffix_year","m213":"second","m398":"second","m266":"hour_12","m437":"year_suffix","m418":"ce","m36":"n_date","m698":"year_suffix","m119":"day_prefix","m739":"n_year","m746":"n_year","m702":"h24","m139":"a_month","m293":"bce","m64":"n_day","m158":"specific_time","m416":"suffix_year","m474":"named_period","m423":"short_year","m411":"precise_time","m106":"roman","m275":"h24","m400":"hour_24","m132":"n_ordinal","m74":"day_prefix","m462":"minute","m134":"roman","m201":"h24","m491":"bce","m537":"date_with_year","m325":"a_day","m459":"h12","m504":"to","m237":"n_year","m526":"hour_24","m463":"second","m548":"year","m552":"year_suffix","m569":"short_year","m433":"year","m59":"n_year","m25":"h24","m265":"time","m577":"a_day","m364":"bce","m144":"h12","m248":"bce","m582":"n_ordinal","m610":"year","m191":"n_month","m254":"two_times","m615":"ce","m454":"minute","m566":"n_day","m659":"named_time","m640":"roman","m120":"a_day","m123":"a_ordinal","m590":"ce","m169":"bce","m244":"n_year","m646":"h12","m607":"a_ordinal","m101":"day_prefix","m374":"roman","m365":"relative_day","m61":"year_suffix","m648":"h12","m745":"short_year","m90":"day_prefix","m618":"a_day","m67":"short_year"},"parentage":{"m400":["m401","m402","m403","m404","m405","m406"],"m158":["m159","m160","m161"],"m390":["m391"],"m442":["m443","m444","m445","m446"],"m621":["m622"],"m264":["m265"],"m161":["m162","m199"],"m559":["m560","m561","m562","m563"],"m239":["m240","m241"],"m119":["m120"],"m496":["m497","m498"],"m300":["m301","m302"],"m3":["m4","m254"],"m57":["m58","m59","m60","m61"],"m412":["m413","m420","m421","m422","m429","m430","m431","m432","m433","m440","m441","m442"],"TOP":["m0"],"m79":["m80","m81","m82"],"m730":["m731","m732"],"m489":["m490","m491"],"m121":["m122","m123","m124"],"m125":["m126","m127","m128","m129","m135","m139"],"m73":["m74","m76","m77","m83","m90","m92","m93","m94","m101","m103","m107","m108"],"m206":["m207","m214","m215","m222"],"m515":["m516","m534","m642","m660","m708"],"m0":["m1"],"m35":["m36","m73"],"m101":["m102"],"m228":["m229","m230"],"m417":["m418","m419"],"m14":["m15"],"m422":["m423","m424","m425","m426"],"m669":["m670","m671"],"m449":["m450","m451","m452","m453","m454","m455"],"m369":["m370"],"m242":["m243","m244","m245","m246"],"m187":["m188","m189"],"m610":["m611","m612","m613","m614"],"m698":["m699","m700"],"m446":["m447","m448"],"m552":["m553","m554"],"m5":["m6","m223"],"m291":["m292","m293"],"m112":["m113","m114"],"m256":["m257","m263"],"m140":["m141"],"m46":["m47","m48","m49","m50"],"m74":["m75"],"m367":["m368","m375"],"m15":["m16","m23","m24","m31"],"m709":["m710","m711","m712","m713","m714","m715"],"m117":["m118","m125"],"m311":["m312","m313"],"m41":["m42","m43"],"m90":["m91"],"m585":["m586","m587","m588","m589"],"m285":["m286","m323"],"m726":["m727","m728"],"m744":["m745","m746","m747","m748"],"m250":["m251"],"m379":["m380","m381"],"m543":["m544","m545"],"m717":["m718","m719","m720","m721","m722","m723"],"m392":["m393","m394","m395","m396","m397","m398"],"m249":["m250","m252","m253"],"m199":["m200","m201","m202","m203","m204","m205"],"m433":["m434","m435","m436","m437"],"m115":["m116","m117"],"m287":["m288","m289","m290","m291"],"m485":["m486","m487","m488","m489"],"m340":["m341"],"m492":["m493","m494","m495","m496"],"m499":["m500","m502","m503"],"m614":["m615","m616"],"m732":["m733","m734"],"m296":["m297","m298","m299","m300"],"m539":["m540","m541","m542","m543"],"m66":["m67","m68","m69","m70"],"m103":["m104","m105","m106"],"m4":["m5"],"m735":["m736","m737"],"m579":["m580","m581"],"m456":["m457","m464","m465","m472"],"m507":["m508","m725"],"m316":["m317","m318","m319","m320"],"m576":["m577"],"m365":["m366","m367"],"m274":["m275","m276","m277","m278","m279","m280"],"m36":["m37","m44","m45","m46","m53","m54","m55","m56","m57","m64","m65","m66"],"m511":["m512"],"m344":["m345","m346","m347","m348"],"m473":["m474","m477"],"m323":["m324","m326","m327","m333","m340","m342","m343","m344","m351","m353","m357","m358"],"m617":["m618","m619"],"m227":["m228","m233","m242","m249"],"m258":["m259","m261"],"m510":["m511","m513"],"m6":["m7","m13"],"m535":["m536","m537"],"m748":["m749","m750"],"m351":["m352"],"m283":["m284","m285"],"m353":["m354","m355","m356"],"m619":["m620","m627"],"m32":["m33","m115"],"m94":["m95","m96","m97","m98"],"m167":["m168","m169"],"m596":["m597","m598","m599","m600"],"m192":["m193","m194","m195","m196"],"m563":["m564","m565"],"m135":["m136","m137","m138"],"m329":["m330","m331","m332"],"m678":["m679","m680"],"m24":["m25","m26","m27","m28","m29","m30"],"m368":["m369","m371"],"m282":["m283","m365"],"m83":["m84","m85","m86","m87"],"m537":["m538","m575"],"m600":["m601","m602"],"m509":["m510","m514"],"m737":["m738","m739","m740","m741"],"m663":["m664","m701"],"m685":["m686","m687","m688","m689"],"m385":["m386","m387","m388"],"m504":["m505","m506"],"m526":["m527","m528","m529","m530","m531","m532"],"m265":["m266","m273","m274","m281"],"m694":["m695","m696","m697","m698"],"m118":["m119","m121"],"m572":["m573","m574"],"m337":["m338","m339"],"m480":["m481","m482"],"m517":["m518","m525","m526","m533"],"m652":["m653","m654","m655","m656","m657","m658"],"m263":["m264","m282","m390","m408","m456"],"m224":["m225","m226"],"m131":["m132","m133","m134"],"m246":["m247","m248"],"m708":["m709","m716","m717","m724"],"m575":["m576","m578","m579","m585","m592","m594","m595","m596","m603","m605","m609","m610"],"m108":["m109","m110","m111","m112"],"m437":["m438","m439"],"m508":["m509","m515"],"m518":["m519","m520","m521","m522","m523","m524"],"m592":["m593"],"m215":["m216","m217","m218","m219","m220","m221"],"m643":["m644","m651","m652","m659"],"m7":["m8","m12"],"m254":["m255","m504","m507"],"m620":["m621","m623"],"m631":["m632","m633"],"m327":["m328","m329"],"m568":["m569","m570","m571","m572"],"m235":["m236","m237","m238","m239"],"m741":["m742","m743"],"m286":["m287","m294","m295","m296","m303","m304","m305","m306","m307","m314","m315","m316"],"m408":["m409","m410","m411"],"m375":["m376","m377","m378","m379","m385","m389"],"m257":["m258","m262"],"m37":["m38","m39","m40","m41"],"m176":["m177","m178"],"m9":["m10"],"m196":["m197","m198"],"m474":["m475","m476"],"m142":["m143","m144","m145","m146","m147","m148"],"m230":["m231","m232"],"m581":["m582","m583","m584"],"m162":["m163","m170","m171","m172","m179","m180","m181","m182","m183","m190","m191","m192"],"m87":["m88","m89"],"m413":["m414","m415","m416","m417"],"m259":["m260"],"m589":["m590","m591"],"m605":["m606","m607","m608"],"m223":["m224","m227"],"m426":["m427","m428"],"m633":["m634","m635","m636"],"m129":["m130","m131"],"m751":["m752","m754","m755"],"m70":["m71","m72"],"m255":["m256","m473"],"m320":["m321","m322"],"m729":["m730","m735","m744","m751"],"m538":["m539","m546","m547","m548","m555","m556","m557","m558","m559","m566","m567","m568"],"m664":["m665","m672","m673","m674","m681","m682","m683","m684","m685","m692","m693","m694"],"m637":["m638","m639","m640"],"m183":["m184","m185","m186","m187"],"m172":["m173","m174","m175","m176"],"m77":["m78","m79"],"m534":["m535","m617"],"m233":["m234","m235"],"m689":["m690","m691"],"m98":["m99","m100"],"m752":["m753"],"m603":["m604"],"m16":["m17","m18","m19","m20","m21","m22"],"m500":["m501"],"m644":["m645","m646","m647","m648","m649","m650"],"m627":["m628","m629","m630","m631","m637","m641"],"m362":["m363","m364"],"m465":["m466","m467","m468","m469","m470","m471"],"m13":["m14","m32","m140","m158","m206"],"m674":["m675","m676","m677","m678"],"m701":["m702","m703","m704","m705","m706","m707"],"m324":["m325"],"m61":["m62","m63"],"m381":["m382","m383","m384"],"m477":["m478","m483","m492","m499"],"m50":["m51","m52"],"m150":["m151","m152","m153","m154","m155","m156"],"m516":["m517"],"m642":["m643"],"m348":["m349","m350"],"m725":["m726","m729"],"m1":["m2","m3"],"m411":["m412","m449"],"m33":["m34","m35"],"m371":["m372","m373","m374"],"m623":["m624","m625","m626"],"m660":["m661","m662","m663"],"m358":["m359","m360","m361","m362"],"m207":["m208","m209","m210","m211","m212","m213"],"m333":["m334","m335","m336","m337"],"m457":["m458","m459","m460","m461","m462","m463"],"m665":["m666","m667","m668","m669"],"m391":["m392","m399","m400","m407"],"m141":["m142","m149","m150","m157"],"m8":["m9","m11"],"m163":["m164","m165","m166","m167"],"m307":["m308","m309","m310","m311"],"m478":["m479","m480"],"m266":["m267","m268","m269","m270","m271","m272"],"m483":["m484","m485"],"m548":["m549","m550","m551","m552"]}}"#).unwrap();
}

/// Simply returns whether the given phrase is parsable as a time expression. This is slightly
/// more efficient than `parse(expression, None).is_ok()` as no parse tree is generated.
///
/// # Examples
///
/// ```rust
/// # extern crate two_timer;
/// # use two_timer::{parsable};
/// let copacetic = parsable("5/6/69");
/// ```
pub fn parsable(phrase: &str) -> bool {
    GRAMMAR.rx().unwrap().is_match(phrase)
}

/// Converts a time expression into a pair or timestamps and a boolean indicating whether
/// the expression was literally a range, such as "9 to 11", as opposed to "9 AM", say.
///
/// The second parameter is an optional `Config` object. In general you will not need to
/// use this except in testing or in the interpretation of pay periods.
///
/// # Examples
///
/// ```rust
/// # extern crate two_timer;
/// # use two_timer::{parse, Config};
/// let (reference_time, _, _) = parse("5/6/69", None).unwrap();
/// ```
pub fn parse(
    phrase: &str,
    config: Option<Config>,
) -> Result<(NaiveDateTime, NaiveDateTime, bool), TimeError> {
    let parse = MATCHER.parse(phrase);
    if parse.is_none() {
        return Err(TimeError::Parse(format!(
            "could not parse \"{}\" as a time expression",
            phrase
        )));
    }
    let parse = parse.unwrap();
    if parse.has("universal") {
        return Ok((first_moment(), last_moment(), false));
    }
    let parse = parse.name("particular").unwrap();
    let config = config.unwrap_or(Config::new());
    if let Some(moment) = parse.name("one_time") {
        return match handle_one_time(moment, &config) {
            Err(e) => Err(e),
            Ok((d1, d2, b)) => {
                let (d3, d4) = adjust(d1, d2, moment);
                if d1 == d3 {
                    Ok((d1, d2, b))
                } else {
                    Ok((d3, d4, b))
                }
            }
        };
    }
    if let Some(two_times) = parse.name("two_times") {
        let first = &two_times.children().unwrap()[0];
        let last = &two_times.children().unwrap()[2];
        let is_through = two_times.has("through");
        if specific(first) {
            if specific(last) {
                return match specific_moment(first, &config) {
                    Ok((d1, d2)) => {
                        let (d1, _) = adjust(d1, d2, first);
                        match specific_moment(last, &config) {
                            Ok((d2, d3)) => {
                                let (d2, d3) = adjust(d2, d3, last);
                                let d2 = pick_terminus(d2, d3, is_through);
                                if d1 <= d2 {
                                    Ok((d1, d2, true))
                                } else {
                                    Err(TimeError::Misordered(format!(
                                        "{} is after {}",
                                        first.as_str(),
                                        last.as_str()
                                    )))
                                }
                            }
                            Err(s) => Err(s),
                        }
                    }
                    Err(s) => Err(s),
                };
            } else {
                return match specific_moment(first, &config) {
                    Ok((d1, d2)) => {
                        let (d1, _) = adjust(d1, d2, first);
                        match relative_moment(last, &config, &d1, false) {
                            Ok((d2, d3)) => {
                                let (d2, d3) = adjust(d2, d3, last);
                                let d2 = pick_terminus(d2, d3, is_through);
                                Ok((d1, d2, true))
                            }
                            Err(s) => Err(s),
                        }
                    }
                    Err(s) => Err(s),
                };
            }
        } else if specific(last) {
            return match specific_moment(last, &config) {
                Ok((d2, d3)) => {
                    let (d2, d3) = adjust(d2, d3, last);
                    let d2 = pick_terminus(d2, d3, is_through);
                    match relative_moment(first, &config, &d2, true) {
                        Ok((d1, d3)) => {
                            let (d1, _) = adjust(d1, d3, first);
                            Ok((d1, d2, true))
                        }
                        Err(s) => Err(s),
                    }
                }
                Err(s) => Err(s),
            };
        } else {
            // the first moment is assumed to be before now
            return match relative_moment(first, &config, &config.now, true) {
                Ok((d1, d2)) => {
                    let (d1, _) = adjust(d1, d2, first);
                    // the second moment is necessarily after the first moment
                    match relative_moment(last, &config, &d1, false) {
                        Ok((d2, d3)) => {
                            let (d2, d3) = adjust(d2, d3, last);
                            let d2 = pick_terminus(d2, d3, is_through);
                            Ok((d1, d2, true))
                        }
                        Err(s) => Err(s),
                    }
                }
                Err(s) => Err(s),
            };
        }
    }
    unreachable!();
}

/// A collection of parameters that can influence the interpretation
/// of time expressions.
#[derive(Debug, Clone)]
pub struct Config {
    now: NaiveDateTime,
    monday_starts_week: bool,
    period: Period,
    pay_period_length: u32,
    pay_period_start: Option<NaiveDate>,
}

impl Config {
    /// Constructs an expression with the default parameters.
    pub fn new() -> Config {
        Config {
            now: Local::now().naive_local(),
            monday_starts_week: true,
            period: Period::Minute,
            pay_period_length: 7,
            pay_period_start: None,
        }
    }
    /// Returns a copy of the configuration parameters with the "now" moment
    /// set to the parameter supplied.
    pub fn now(&self, n: NaiveDateTime) -> Config {
        let mut c = self.clone();
        c.now = n;
        c
    }
    fn period(&self, period: Period) -> Config {
        let mut c = self.clone();
        c.period = period;
        c
    }
    /// Returns a copy of the configuration parameters with whether
    /// Monday is regarded as the first day of the week set to the parameter
    /// supplied. By default Monday *is* regarded as the first day. If this
    /// parameter is set to `false`, Sunday will be regarded as the first weekday.
    pub fn monday_starts_week(&self, monday_starts_week: bool) -> Config {
        let mut c = self.clone();
        c.monday_starts_week = monday_starts_week;
        c
    }
    /// Returns a copy of the configuration parameters with the pay period
    /// length in days set to the parameter supplied. The default pay period
    /// length is 7 days.
    pub fn pay_period_length(&self, pay_period_length: u32) -> Config {
        let mut c = self.clone();
        c.pay_period_length = pay_period_length;
        c
    }
    /// Returns a copy of the configuration parameters with the reference start
    /// date for a pay period set to the parameter supplied. By default this date
    /// is undefined. Unless it is defined, expressions containing the phrase "pay period"
    /// or "pp" cannot be interpreted.
    pub fn pay_period_start(&self, pay_period_start: Option<NaiveDate>) -> Config {
        let mut c = self.clone();
        c.pay_period_start = pay_period_start;
        c
    }
}

/// A simple categorization of things that could go wrong.
///
/// Every error provides a descriptive string that can be displayed.
#[derive(Debug, Clone)]
pub enum TimeError {
    /// The time expression cannot be parsed by the available grammar.
    Parse(String),
    /// The time expression consists of a time range and the end of the range is before
    /// the beginning.
    Misordered(String),
    /// The time expression specifies an impossible date, such as the 31st of September.
    ImpossibleDate(String),
    /// The time expression specifies a weekday different from that required by the rest
    /// of the expression, such as Wednesday, May 5, 1969, which was a Tuesday.
    Weekday(String),
    /// The time expression refers to a pay period, but the starting date of a reference
    /// pay period has not been provided, so the pay period is undefined.
    NoPayPeriod(String),
}

impl TimeError {
    /// Extracts error message.
    pub fn msg(&self) -> &str {
        match self {
            TimeError::Parse(s) => s.as_ref(),
            TimeError::Misordered(s) => s.as_ref(),
            TimeError::ImpossibleDate(s) => s.as_ref(),
            TimeError::Weekday(s) => s.as_ref(),
            TimeError::NoPayPeriod(s) => s.as_ref(),
        }
    }
}

// for the end time, if the span is less than a day, use the first, otherwise use the second
// e.g., Monday through Friday at 3 PM should end at 3 PM, but Monday through Friday should end at the end of Friday
fn pick_terminus(d1: NaiveDateTime, d2: NaiveDateTime, through: bool) -> NaiveDateTime {
    if d1.day() == d2.day() && d1.month() == d2.month() && d1.year() == d2.year() {
        d1
    } else if through {
        d2
    } else {
        d1
    }
}

/// The moment regarded as the beginning of time.
///
/// # Examples
///
/// ```rust
/// # extern crate two_timer;
/// # use two_timer::first_moment;
/// println!("{}", first_moment()); // -262144-01-01 00:00:00
/// ```
pub fn first_moment() -> NaiveDateTime {
    chrono::naive::MIN_DATE.and_hms_milli(0, 0, 0, 0)
}

/// The moment regarded as the end of time.
///
/// # Examples
///
/// ```rust
/// # extern crate two_timer;
/// # use two_timer::last_moment;
/// println!("{}", last_moment()); // +262143-12-31 23:59:59.999
/// ```
pub fn last_moment() -> NaiveDateTime {
    chrono::naive::MAX_DATE.and_hms_milli(23, 59, 59, 999)
}

fn specific(m: &Match) -> bool {
    m.has("specific_day") || m.has("specific_period") || m.has("specific_time")
}

fn n_date(date: &Match, config: &Config) -> Result<NaiveDate, TimeError> {
    let year = year(date, &config.now);
    let month = n_month(date);
    let day = n_day(date);
    match NaiveDate::from_ymd_opt(year, month, day) {
        None => Err(TimeError::ImpossibleDate(format!(
            "cannot construct date with year {}, month {}, and day {}",
            year, month, day
        ))),
        Some(d) => Ok(d),
    }
}

fn handle_specific_day(
    m: &Match,
    config: &Config,
) -> Result<(NaiveDateTime, NaiveDateTime), TimeError> {
    let now = config.now.clone();
    let mut times = m.all_names("time");
    if times.len() > 1 {
        return Err(TimeError::Parse(format!(
            "more than one daytime specified in {}",
            m.as_str()
        )));
    }
    let time = times.pop();
    if let Some(adverb) = m.name("adverb") {
        return match adverb.as_str().chars().nth(0).expect("empty string") {
            // now
            'n' | 'N' => Ok(moment_and_time(config, time)),
            't' | 'T' => match adverb.as_str().chars().nth(2).expect("impossible string") {
                // today
                'd' | 'D' => Ok(moment_and_time(&config.period(Period::Day), time)),
                // tomorrow
                'm' | 'M' => Ok(moment_and_time(
                    &Config::new()
                        .now(now + Duration::days(1))
                        .period(Period::Day),
                    time,
                )),
                _ => unreachable!(),
            },
            // yesterday
            'y' | 'Y' => Ok(moment_and_time(
                &Config::new()
                    .now(now - Duration::days(1))
                    .period(Period::Day),
                time,
            )),
            _ => unreachable!(),
        };
    }
    if let Some(date) = m.name("date_with_year") {
        if let Some(date) = date.name("n_date") {
            return match n_date(date, config) {
                Err(s) => Err(s),
                Ok(d1) => {
                    let d1 = d1.and_hms(0, 0, 0);
                    Ok(moment_and_time(
                        &Config::new().now(d1).period(Period::Day),
                        time,
                    ))
                }
            };
        }
        if let Some(date) = date.name("a_date") {
            let year = year(date, &now);
            let month = a_month(date);
            let day = if date.has("n_day") {
                n_day(date)
            } else {
                o_day(date, month)
            };
            let d_opt = NaiveDate::from_ymd_opt(year, month, day);
            return match d_opt {
                None => Err(TimeError::ImpossibleDate(format!(
                    "cannot construct date with year {}, month {}, and day {}",
                    year, month, day
                ))),
                Some(d1) => {
                    if let Some(wd) = date.name("a_day") {
                        let wd = weekday(wd.as_str());
                        if wd == d1.weekday() {
                            let d1 = d1.and_hms(0, 0, 0);
                            Ok(moment_and_time(
                                &Config::new().now(d1).period(Period::Day),
                                time,
                            ))
                        } else {
                            Err(TimeError::Weekday(format!(
                                "the weekday of year {}, month {}, day {} is not {}",
                                year,
                                month,
                                day,
                                date.name("a_day").unwrap().as_str()
                            )))
                        }
                    } else {
                        let d1 = d1.and_hms(0, 0, 0);
                        Ok(moment_and_time(
                            &Config::new().now(d1).period(Period::Day),
                            time,
                        ))
                    }
                }
            };
        }
        unreachable!();
    }
    unimplemented!();
}

fn handle_specific_period(
    moment: &Match,
    config: &Config,
) -> Result<(NaiveDateTime, NaiveDateTime), TimeError> {
    if let Some(moment) = moment.name("relative_period") {
        let count = count(moment.name("count").unwrap()) as i64;
        let (displacement, period) = match moment
            .name("displacement")
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .unwrap()
        {
            'w' | 'W' => (Duration::weeks(count), Period::Week),
            'd' | 'D' => (Duration::days(count), Period::Day),
            'h' | 'H' => (Duration::hours(count), Period::Hour),
            'm' | 'M' => (Duration::minutes(count), Period::Minute),
            's' | 'S' => (Duration::seconds(count), Period::Second),
            _ => unreachable!(),
        };
        let d = match moment
            .name("from_now_or_ago")
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .unwrap()
        {
            'a' | 'A' => config.now - displacement,
            'f' | 'F' => config.now + displacement,
            _ => unreachable!(),
        };
        let span = match period {
            Period::Week => (d, d + Duration::weeks(1)),
            _ => moment_to_period(d, &period, config),
        };
        return Ok(span);
    }
    if let Some(moment) = moment.name("month_and_year") {
        let y = year(moment, &config.now);
        let m = a_month(moment);
        return match NaiveDate::from_ymd_opt(y, m, 1) {
            None => unreachable!(),
            Some(d1) => {
                let d1 = d1.and_hms(0, 0, 0);
                Ok(moment_and_time(
                    &Config::new().now(d1).period(Period::Month),
                    None,
                ))
            }
        };
    }
    if let Some(moment) = moment.name("modified_period") {
        let modifier = PeriodModifier::from_match(moment.name("modifier").unwrap());
        if let Some(month) = moment.name("a_month") {
            let d = config.now.with_month(a_month(month)).unwrap();
            let (d, _) = moment_to_period(d, &Period::Month, config);
            let d = match modifier {
                PeriodModifier::Next => d.with_year(d.year() + 1).unwrap(),
                PeriodModifier::Last => d.with_year(d.year() - 1).unwrap(),
                PeriodModifier::This => d,
            };
            return Ok(moment_to_period(d, &Period::Month, config));
        }
        if let Some(wd) = moment.name("a_day") {
            let wd = weekday(wd.as_str());
            let offset = config.now.weekday().num_days_from_monday() as i64
                - wd.num_days_from_monday() as i64;
            let d = config.now.date() - Duration::days(offset);
            let d = match modifier {
                PeriodModifier::Next => d + Duration::days(7),
                PeriodModifier::Last => d - Duration::days(7),
                PeriodModifier::This => d,
            };
            return Ok(moment_to_period(d.and_hms(0, 0, 0), &Period::Day, config));
        }
        return match ModifiablePeriod::from_match(moment.name("modifiable_period").unwrap()) {
            ModifiablePeriod::Week => {
                let (d, _) = moment_to_period(config.now, &Period::Week, config);
                let d = match modifier {
                    PeriodModifier::Next => d + Duration::days(7),
                    PeriodModifier::Last => d - Duration::days(7),
                    PeriodModifier::This => d,
                };
                Ok(moment_to_period(d, &Period::Week, config))
            }
            ModifiablePeriod::Weekend => {
                let (_, d2) =
                    moment_to_period(config.now, &Period::Week, &config.monday_starts_week(true));
                let d2 = match modifier {
                    PeriodModifier::Next => d2 + Duration::days(7),
                    PeriodModifier::Last => d2 - Duration::days(7),
                    PeriodModifier::This => d2,
                };
                let d1 = d2 - Duration::days(2);
                Ok((d1, d2))
            }
            ModifiablePeriod::Month => {
                let (d, _) = moment_to_period(config.now, &Period::Month, config);
                let d = match modifier {
                    PeriodModifier::Next => {
                        if d.month() == 12 {
                            d.with_year(d.year() + 1).unwrap().with_month(1).unwrap()
                        } else {
                            d.with_month(d.month() + 1).unwrap()
                        }
                    }
                    PeriodModifier::Last => {
                        if d.month() == 1 {
                            d.with_year(d.year() - 1).unwrap().with_month(12).unwrap()
                        } else {
                            d.with_month(d.month() - 1).unwrap()
                        }
                    }
                    PeriodModifier::This => d,
                };
                Ok(moment_to_period(d, &Period::Month, config))
            }
            ModifiablePeriod::Year => {
                let (d, _) = moment_to_period(config.now, &Period::Year, config);
                let d = match modifier {
                    PeriodModifier::Next => d.with_year(d.year() + 1).unwrap(),
                    PeriodModifier::Last => d.with_year(d.year() - 1).unwrap(),
                    PeriodModifier::This => d,
                };
                Ok(moment_to_period(d, &Period::Year, config))
            }
            ModifiablePeriod::PayPeriod => {
                if config.pay_period_start.is_some() {
                    let (d, _) = moment_to_period(config.now, &Period::PayPeriod, config);
                    let d = match modifier {
                        PeriodModifier::Next => d + Duration::days(config.pay_period_length as i64),
                        PeriodModifier::Last => d - Duration::days(config.pay_period_length as i64),
                        PeriodModifier::This => d,
                    };
                    Ok(moment_to_period(d, &Period::PayPeriod, config))
                } else {
                    Err(TimeError::NoPayPeriod(String::from(
                        "no pay period start date provided",
                    )))
                }
            }
        };
    }
    if let Some(moment) = moment.name("year") {
        let year = year(moment, &config.now);
        return Ok(moment_to_period(
            NaiveDate::from_ymd(year, 1, 1).and_hms(0, 0, 0),
            &Period::Year,
            config,
        ));
    }
    unreachable!()
}

enum ModifiablePeriod {
    Week,
    Month,
    Year,
    PayPeriod,
    Weekend,
}

impl ModifiablePeriod {
    fn from_match(m: &Match) -> ModifiablePeriod {
        match m.as_str().chars().nth(0).expect("unreachable") {
            'w' | 'W' => {
                if m.as_str().len() == 4 {
                    ModifiablePeriod::Week
                } else {
                    ModifiablePeriod::Weekend
                }
            }
            'm' | 'M' => ModifiablePeriod::Month,
            'y' | 'Y' => ModifiablePeriod::Year,
            'p' | 'P' => ModifiablePeriod::PayPeriod,
            _ => unreachable!(),
        }
    }
}

enum PeriodModifier {
    This,
    Next,
    Last,
}

impl PeriodModifier {
    fn from_match(m: &Match) -> PeriodModifier {
        match m.as_str().chars().nth(0).expect("unreachable") {
            't' | 'T' => PeriodModifier::This,
            'l' | 'L' => PeriodModifier::Last,
            'n' | 'N' => PeriodModifier::Next,
            _ => unreachable!(),
        }
    }
}

fn handle_specific_time(
    moment: &Match,
    config: &Config,
) -> Result<(NaiveDateTime, NaiveDateTime), TimeError> {
    if let Some(moment) = moment.name("precise_time") {
        return match n_date(moment, config) {
            Err(s) => Err(s),
            Ok(d) => {
                let (hour, minute, second, _) = time(moment);
                let period = if second.is_some() {
                    Period::Second
                } else if minute.is_some() {
                    Period::Minute
                } else {
                    Period::Hour
                };
                let m = d
                    .and_hms(0, 0, 0)
                    .with_hour(hour)
                    .unwrap()
                    .with_minute(minute.unwrap_or(0))
                    .unwrap()
                    .with_second(second.unwrap_or(0))
                    .unwrap();
                Ok(moment_to_period(m, &period, config))
            }
        };
    }
    return if moment.has("first_time") {
        Ok(moment_to_period(first_moment(), &config.period, config))
    } else {
        Ok((last_moment(), last_moment()))
    };
}

fn handle_one_time(
    moment: &Match,
    config: &Config,
) -> Result<(NaiveDateTime, NaiveDateTime, bool), TimeError> {
    let r = if moment.has("specific_day") {
        handle_specific_day(moment, config)
    } else if let Some(moment) = moment.name("specific_period") {
        handle_specific_period(moment, config)
    } else if let Some(moment) = moment.name("specific_time") {
        handle_specific_time(moment, config)
    } else {
        relative_moment(moment, config, &config.now, true)
    };
    match r {
        Ok((d1, d2)) => Ok((d1, d2, false)),
        Err(e) => Err(e),
    }
}

// add time to a date
fn moment_and_time(config: &Config, daytime: Option<&Match>) -> (NaiveDateTime, NaiveDateTime) {
    if let Some(daytime) = daytime {
        let (hour, minute, second, is_midnight) = time(daytime);
        let period = if second.is_some() {
            Period::Second
        } else if minute.is_some() {
            Period::Minute
        } else {
            Period::Hour
        };
        let mut m = config
            .now
            .with_hour(hour)
            .unwrap()
            .with_minute(minute.unwrap_or(0))
            .unwrap()
            .with_second(second.unwrap_or(0))
            .unwrap();
        if is_midnight {
            m = m + Duration::days(1); // midnight is second 0 *of the next day*
        }
        moment_to_period(m, &period, config)
    } else {
        moment_to_period(config.now, &config.period, config)
    }
}

fn relative_moment(
    m: &Match,
    config: &Config,
    other_time: &NaiveDateTime,
    before: bool,
) -> Result<(NaiveDateTime, NaiveDateTime), TimeError> {
    if let Some(a_month_and_a_day) = m.name("a_day_in_month") {
        return match month_and_a_day(a_month_and_a_day, config, other_time, before) {
            Ok(d) => Ok(moment_and_time(
                &config.now(d.and_hms(0, 0, 0)).period(Period::Day),
                m.name("time"),
            )),
            Err(e) => Err(e),
        };
    }
    if let Some(day) = m.name("a_day") {
        let wd = weekday(day.as_str());
        let mut delta =
            other_time.weekday().num_days_from_sunday() as i64 - wd.num_days_from_sunday() as i64;
        if delta <= 0 {
            delta += 7;
        }
        let mut d = other_time.date() - Duration::days(delta);
        if !before {
            d = d + Duration::days(7);
        }
        return Ok(moment_and_time(
            &config.now(d.and_hms(0, 0, 0)).period(Period::Day),
            m.name("time"),
        ));
    }
    if let Some(t) = m.name("time") {
        let (hour, minute, second, is_midnight) = time(t);
        let period = if second.is_some() {
            Period::Second
        } else if minute.is_some() {
            Period::Minute
        } else {
            Period::Hour
        };
        let mut t = other_time
            .with_hour(hour)
            .unwrap()
            .with_minute(minute.unwrap_or(0))
            .unwrap()
            .with_second(second.unwrap_or(0))
            .unwrap();
        if is_midnight {
            t = t + Duration::days(1); // midnight is second 0 *of the next day*
        }
        if before && t > *other_time {
            t = t - Duration::days(1);
        } else if !before && t < *other_time {
            t = t + Duration::days(1);
        }
        return Ok(moment_to_period(t, &period, config));
    }
    if let Some(month) = m.name("a_month") {
        let month = a_month(month);
        let year = if before {
            if month > other_time.month() {
                other_time.year() - 1
            } else {
                other_time.year()
            }
        } else {
            if month < other_time.month() {
                other_time.year() + 1
            } else {
                other_time.year()
            }
        };
        let d = NaiveDate::from_ymd(year, month, 1).and_hms(0, 0, 0);
        let (d1, d2) = moment_to_period(d, &Period::Month, config);
        if before && d1 >= *other_time {
            return Ok(moment_to_period(
                d1.with_year(d1.year() - 1).unwrap(),
                &Period::Month,
                config,
            ));
        } else if !before && d2 <= *other_time {
            return Ok(moment_to_period(
                d1.with_year(d1.year() + 1).unwrap(),
                &Period::Month,
                config,
            ));
        }
        return Ok((d1, d2));
    }
    unreachable!()
}

// for things like "the fifth", "March fifth", "5-6"
fn month_and_a_day(
    m: &Match,
    config: &Config,
    other_time: &NaiveDateTime,
    before: bool,
) -> Result<NaiveDate, TimeError> {
    if m.has("ordinal_day") {
        let mut year = config.now.year();
        let mut month = other_time.month();
        let day = o_day(m, month);
        let wd = if let Some(a_day) = m.name("a_day") {
            Some(weekday(a_day.as_str()))
        } else {
            None
        };
        // search backwards through the calendar for a possible day
        for _ in 0..4 * 7 * 12 {
            if let Some(d) = NaiveDate::from_ymd_opt(year, month, day) {
                if wd.is_none() || d.weekday() == wd.unwrap() {
                    return Ok(d);
                }
            }
            if month == 1 {
                month = 12;
                year -= 1;
            } else {
                month -= 1;
            }
        }
        return Err(TimeError::ImpossibleDate(format!(
            "there is no day {} in the year {}",
            m.as_str(),
            config.now.year()
        )));
    }
    let (month, day) = if let Some(month) = m.name("n_month") {
        let month = n_month(month);
        let day = m.name("n_day").unwrap();
        (month, n_day(day))
    } else {
        let month = a_month(m);
        let day = if let Some(day) = m.name("n_day") {
            n_day(day)
        } else {
            o_day(m, month)
        };
        (month, day)
    };
    let year = if before {
        config.now.year()
    } else {
        if month < other_time.month() {
            other_time.year() + 1
        } else {
            other_time.year()
        }
    };
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => Ok(d),
        None => Err(TimeError::ImpossibleDate(format!(
            "could not construct date from {} with year {}, month {}, and day {}",
            m.as_str(),
            year,
            month,
            day
        ))),
    }
}

fn specific_moment(
    m: &Match,
    config: &Config,
) -> Result<(NaiveDateTime, NaiveDateTime), TimeError> {
    if let Some(m) = m.name("specific_day") {
        return handle_specific_day(m, config);
    }
    if let Some(m) = m.name("specific_period") {
        return handle_specific_period(m, config);
    }
    if let Some(m) = m.name("specific_time") {
        return handle_specific_time(m, config);
    }
    unreachable!()
}

fn a_month(m: &Match) -> u32 {
    match m.name("a_month").unwrap().as_str()[0..3]
        .to_lowercase()
        .as_ref()
    {
        "jan" => 1,
        "feb" => 2,
        "mar" => 3,
        "apr" => 4,
        "may" => 5,
        "jun" => 6,
        "jul" => 7,
        "aug" => 8,
        "sep" => 9,
        "oct" => 10,
        "nov" => 11,
        "dec" => 12,
        _ => unreachable!(),
    }
}

// extract hour, minute, and second from time match
// last parameter is basically whether the value returned is for "midnight", which requires special handling
fn time(m: &Match) -> (u32, Option<u32>, Option<u32>, bool) {
    if let Some(m) = m.name("named_time") {
        return match m.as_str().chars().nth(0).unwrap() {
            'n' | 'N' => (12, None, None, false),
            _ => (0, None, None, true),
        };
    }
    let hour = if let Some(hour_24) = m.name("hour_24") {
        s_to_n(hour_24.name("h24").unwrap().as_str())
    } else if let Some(hour_12) = m.name("hour_12") {
        let mut hour = s_to_n(hour_12.name("h12").unwrap().as_str());
        hour = if let Some(am_pm) = m.name("am_pm") {
            match am_pm.as_str().chars().nth(0).expect("empty string") {
                'a' | 'A' => hour,
                _ => hour + 12,
            }
        } else {
            hour
        };
        if hour == 24 {
            0
        } else {
            hour
        }
    } else {
        unreachable!()
    };
    if let Some(minute) = m.name("minute") {
        let minute = s_to_n(minute.as_str());
        if let Some(second) = m.name("second") {
            let second = s_to_n(second.as_str());
            (hour, Some(minute), Some(second), false)
        } else {
            (hour, Some(minute), None, false)
        }
    } else {
        (hour, None, None, false)
    }
}

fn n_month(m: &Match) -> u32 {
    lazy_static! {
        static ref MONTH: Regex = Regex::new(r"\A0?(\d{1,2})\z").unwrap();
    }
    let cap = MONTH.captures(m.name("n_month").unwrap().as_str()).unwrap();
    cap[1].parse::<u32>().unwrap()
}

fn year(m: &Match, now: &NaiveDateTime) -> i32 {
    let year = m.name("year").unwrap();
    if let Some(sy) = year.name("short_year") {
        let y = s_to_n(sy.as_str()) as i32;
        let this_year = now.year() % 100;
        if this_year < y {
            now.year() - this_year - 100 + y
        } else {
            now.year() - this_year + y
        }
    } else if let Some(suffix) = year.name("year_suffix") {
        let y = s_to_n(year.name("suffix_year").unwrap().as_str()) as i32;
        if suffix.has("bce") {
            1 - y // there is no year 0
        } else {
            y
        }
    } else {
        let y = s_to_n(year.name("n_year").unwrap().as_str()) as i32;
        if year.as_str().chars().nth(0).expect("unreachable") == '-' {
            -y
        } else {
            y
        }
    }
}

fn s_to_n(s: &str) -> u32 {
    lazy_static! {
        static ref S_TO_N: Regex = Regex::new(r"\A[\D0]*(\d+)\z").unwrap();
    }
    S_TO_N.captures(s).unwrap()[1].parse::<u32>().unwrap()
}

fn n_day(m: &Match) -> u32 {
    m.name("n_day").unwrap().as_str().parse::<u32>().unwrap()
}

fn o_day(m: &Match, month: u32) -> u32 {
    let m = m.name("o_day").unwrap();
    let s = m.as_str();
    if m.has("a_ordinal") {
        ordinal(s)
    } else if m.has("n_ordinal") {
        s[0..s.len() - 2].parse::<u32>().unwrap()
    } else {
        // roman
        match s.chars().nth(0).expect("empty string") {
            'n' | 'N' => {
                // nones
                match month {
                    3 | 5 | 7 | 10 => 7, // March, May, July, October
                    _ => 5,
                }
            }
            'i' | 'I' => {
                // ides
                match month {
                    3 | 5 | 7 | 10 => 15, // March, May, July, October
                    _ => 13,
                }
            }
            _ => 1, // kalends
        }
    }
}

// converts the ordinals up to thirty-first
fn ordinal(s: &str) -> u32 {
    match s.chars().nth(0).expect("empty string") {
        'f' | 'F' => {
            match s.chars().nth(1).expect("too short") {
                'i' | 'I' => {
                    match s.chars().nth(2).expect("too short") {
                        'r' | 'R' => 1, // first
                        _ => {
                            if s.len() == 5 {
                                5 // fifth
                            } else {
                                15 // fifteenth
                            }
                        }
                    }
                }
                _ => {
                    if s.len() == 6 {
                        4 // fourth
                    } else {
                        14 // fourteenth
                    }
                }
            }
        }
        's' | 'S' => {
            match s.chars().nth(1).expect("too short") {
                'e' | 'E' => {
                    match s.len() {
                        6 => 2,  // second
                        7 => 7,  // seventh
                        _ => 17, // seventeenth
                    }
                }
                _ => {
                    if s.len() == 5 {
                        6 // sixth
                    } else {
                        16 // sixteenth
                    }
                }
            }
        }
        't' | 'T' => {
            match s.chars().nth(1).expect("too short") {
                'h' | 'H' => {
                    match s.chars().nth(4).expect("too short") {
                        'd' | 'D' => 3, //third
                        _ => {
                            match s.chars().nth(5).expect("too short") {
                                'e' | 'E' => 13, // thirteenth
                                'i' | 'I' => 30, // thirtieth
                                _ => 31,         // thirty-first
                            }
                        }
                    }
                }
                'e' | 'E' => 10, // tenth
                _ => {
                    match s.chars().nth(3).expect("too short") {
                        'l' | 'L' => 12, // twelfth
                        _ => {
                            if s.len() == 9 {
                                20 // twentiety
                            } else {
                                20 + ordinal(&s[7..s.len()]) // twenty-first...
                            }
                        }
                    }
                }
            }
        }
        'e' | 'E' => {
            match s.chars().nth(1).expect("too short") {
                'i' | 'I' => {
                    if s.len() == 6 {
                        8 // eight
                    } else {
                        18 // eighteen
                    }
                }
                _ => 11, // eleventh
            }
        }
        _ => {
            if s.len() == 5 {
                9 // ninth
            } else {
                19 // nineteenth
            }
        }
    }
}

/// expand a moment to the period containing it
fn moment_to_period(
    now: NaiveDateTime,
    period: &Period,
    config: &Config,
) -> (NaiveDateTime, NaiveDateTime) {
    match period {
        Period::Year => {
            let d1 = NaiveDate::from_ymd(now.year(), 1, 1).and_hms(0, 0, 0);
            let d2 = NaiveDate::from_ymd(now.year() + 1, 1, 1).and_hms(0, 0, 0);
            (d1, d2)
        }
        Period::Month => {
            let d1 = NaiveDate::from_ymd(now.year(), now.month(), 1).and_hms(0, 0, 0);
            let d2 = if now.month() == 12 {
                NaiveDate::from_ymd(now.year() + 1, 1, 1)
            } else {
                NaiveDate::from_ymd(now.year(), now.month() + 1, 1)
            }
            .and_hms(0, 0, 0);
            (d1, d2)
        }
        Period::Week => {
            let offset = if config.monday_starts_week {
                now.weekday().num_days_from_monday()
            } else {
                now.weekday().num_days_from_sunday()
            };
            let d1 = NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0)
                - Duration::days(offset as i64);
            (d1, d1 + Duration::days(7))
        }
        Period::Day => {
            let d1 = NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0);
            (d1, d1 + Duration::days(1))
        }
        Period::Hour => {
            let d1 =
                NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(now.hour(), 0, 0);
            (d1, d1 + Duration::hours(1))
        }
        Period::Minute => {
            let d1 = NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(
                now.hour(),
                now.minute(),
                0,
            );
            (d1, d1 + Duration::minutes(1))
        }
        Period::Second => {
            let d1 = NaiveDate::from_ymd(now.year(), now.month(), now.day()).and_hms(
                now.hour(),
                now.minute(),
                now.second(),
            );
            (d1, d1 + Duration::seconds(1))
        }
        Period::PayPeriod => {
            if let Some(pps) = config.pay_period_start {
                // find the current pay period start
                let offset = now.num_days_from_ce() - pps.num_days_from_ce();
                let ppl = config.pay_period_length as i32;
                let mut offset = (offset % ppl) as i64;
                if offset < 0 {
                    offset += config.pay_period_length as i64;
                }
                let d1 = (now.date() - Duration::days(offset)).and_hms(0, 0, 0);
                (d1, d1 + Duration::days(config.pay_period_length as i64))
            } else {
                unreachable!()
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Period {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    PayPeriod,
}

fn weekday(s: &str) -> Weekday {
    match s.chars().nth(0).expect("empty string") {
        'm' | 'M' => Weekday::Mon,
        't' | 'T' => {
            if s.len() == 1 {
                Weekday::Tue
            } else {
                match s.chars().nth(1).unwrap() {
                    'u' | 'U' => Weekday::Tue,
                    'h' | 'H' => Weekday::Thu,
                    _ => unreachable!(),
                }
            }
        }
        'w' | 'W' => Weekday::Wed,
        'H' => Weekday::Thu,
        'F' | 'f' => Weekday::Fri,
        'S' | 's' => {
            if s.len() == 1 {
                Weekday::Sat
            } else {
                match s.chars().nth(1).unwrap() {
                    'a' | 'A' => Weekday::Sat,
                    'u' | 'U' => Weekday::Sun,
                    _ => unreachable!(),
                }
            }
        }
        'U' => Weekday::Sun,
        _ => unreachable!(),
    }
}

// adjust a period relative to another period -- e.g., "one week before June" or "five minutes around 12:00 PM"
fn adjust(d1: NaiveDateTime, d2: NaiveDateTime, m: &Match) -> (NaiveDateTime, NaiveDateTime) {
    if let Some(adjustment) = m.name("adjustment") {
        let count = count(adjustment.name("count").unwrap()) as i64;
        let unit = match adjustment
            .name("unit")
            .unwrap()
            .as_str()
            .chars()
            .nth(0)
            .unwrap()
        {
            'w' | 'W' => Duration::weeks(count),
            'd' | 'D' => Duration::days(count),
            'h' | 'H' => Duration::hours(count),
            'm' | 'M' => Duration::minutes(count),
            _ => Duration::seconds(count),
        };
        let direction = adjustment.name("direction").unwrap().as_str();
        match direction.chars().nth(0).unwrap() {
            'b' | 'B' => {
                if direction.len() == 6 {
                    // before
                    let d = d1 - unit;
                    (d, d)
                } else {
                    // before and after
                    (d1 - unit, d1 + unit)
                }
            }
            _ => match direction.chars().nth(1).unwrap() {
                'f' | 'F' => {
                    let d = d2 + unit;
                    (d, d)
                }
                _ => {
                    let d1 = d1 - Duration::milliseconds(unit.num_milliseconds() / 2);
                    let d2 = d1 + unit;
                    (d1, d2)
                }
            },
        }
    } else {
        (d1, d2)
    }
}

// for converting a few cardinal numbers and integer expressions
fn count(m: &Match) -> u32 {
    let s = m.as_str();
    if m.has("a_count") {
        // cardinal numbers
        match s.chars().nth(0).expect("impossibly short") {
            'o' | 'O' => 1,
            't' | 'T' => match s.chars().nth(1).expect("impossibly short") {
                'w' | 'W' => 2,
                'h' | 'H' => 3,
                _ => 10,
            },
            'f' | 'F' => match s.chars().nth(1).expect("impossibly short") {
                'o' | 'O' => 4,
                _ => 5,
            },
            's' | 'S' => match s.chars().nth(1).expect("impossibly short") {
                'i' | 'I' => 6,
                _ => 7,
            },
            'e' | 'E' => 8,
            _ => 9,
        }
    } else {
        s.parse::<u32>().unwrap()
    }
}
