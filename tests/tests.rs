extern crate two_timer;
use two_timer::{parsable, parse, Config, TimeError};
extern crate time;
use time::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime};

// a debugging method to print out the parse tree
// fn show_me(p: &str) {
//     println!("{}", two_timer::MATCHER.parse(p).unwrap());
// }

#[test]
fn can_use_parsable() {
    assert!(parsable("2019/1/1"));
}

#[test]
fn always() {
    let alpha = Date::MIN.midnight();
    let omega = Date::MAX.with_hms_milli(23, 59, 59, 999).unwrap();
    for phrase in [
        "always",
        "ever",
        "all time",
        "forever",
        "from beginning to end",
        "from the beginning to the end",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(alpha, start);
        assert_eq!(omega, end);
    }
}

fn datetime() -> PrimitiveDateTime {
    let now = OffsetDateTime::now_utc();
    now.date().with_time(now.time())
}

#[test]
fn yesterday() {
    let now = datetime();
    let (start, end, _) = parse("yesterday", Some(Config::new().now(now))).unwrap();
    assert!(start < now);
    assert!(end < now);
    let then = now - Duration::days(1);
    assert!(start < then);
    assert!(then < end);
    let then = then - Duration::days(1);
    assert!(then < start);
}

#[test]
fn tomorrow() {
    let now = datetime();
    let (start, end, _) = parse("tomorrow", Some(Config::new().now(now))).unwrap();
    assert!(start > now);
    assert!(end > now);
    let then = now + Duration::days(1);
    assert!(start < then);
    assert!(then < end);
    let then = then + Duration::days(1);
    assert!(then > end);
}

#[test]
fn today() {
    let now = datetime();
    let (start, end, _) = parse("today", Some(Config::new().now(now))).unwrap();
    assert!(start < now);
    assert!(end > now);
    let then = now + Duration::days(1);
    assert!(start < then);
    assert!(then > end);
    let then = now - Duration::days(1);
    assert!(then < start);
    assert!(then < end);
}

#[test]
fn day_5_6_69_at_3_30_pm() {
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 30, 0).unwrap();
    for phrase in [
        "at 3:30 PM on 5-6-69",
        "3:30 p.m. on 5-6-69",
        "at 15:30 on 5-6-69",
        "15:30 on 5-6-69",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_pm() {
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 0, 0).unwrap();
    for phrase in [
        "at 3 PM on 5-6-69",
        "3 p.m. on 5-6-69",
        "at 15 on 5-6-69",
        "15 on 5-6-69",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_00_pm() {
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 30, 0).unwrap();
    for phrase in [
        "at 3:30:00 PM on 5-6-69",
        "3:30:00 p.m. on 5-6-69",
        "at 15:30:00 on 5-6-69",
        "15:30:00 on 5-6-69",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_01_pm() {
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 30, 1).unwrap();
    for phrase in [
        "at 3:30:01 PM on 5-6-69",
        "3:30:01 p.m. on 5-6-69",
        "at 15:30:01 on 5-6-69",
        "15:30:01 on 5-6-69",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_01_am() {
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(3, 30, 1).unwrap();
    for phrase in [
        "at 3:30:01 AM on 5-6-69",
        "3:30:01 a.m. on 5-6-69",
        "at 3:30:01 on 5-6-69",
        "3:30:01 on 5-6-69",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(16, 0, 0).unwrap();
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 0, 0).unwrap();
    for phrase in ["3 PM", "3 pm", "15"].iter() {
        let (start, end, _) = parse(phrase, Some(Config::new().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm_default_to_future() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(14, 0, 0).unwrap();
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 0, 0).unwrap();
    for phrase in ["3 PM", "3 pm", "15"].iter() {
        let (start, end, _) =
            parse(phrase, Some(Config::new().now(now).default_to_past(false))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_00_pm() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(16, 0, 0).unwrap();
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 0, 0).unwrap();
    for phrase in ["3:00 PM", "3:00 pm", "15:00"].iter() {
        let (start, end, _) = parse(phrase, Some(Config::new().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_00_00_pm() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(16, 0, 0).unwrap();
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(15, 0, 0).unwrap();
    for phrase in ["3:00:00 PM", "3:00:00 pm", "15:00:00"].iter() {
        let (start, end, _) = parse(phrase, Some(Config::new().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm_yesterday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(14, 0, 0).unwrap();
    let then = Date::from_calendar_date(1969, Month::May, 5).unwrap().with_hms(15, 0, 0).unwrap();
    for phrase in ["3 PM yesterday", "3 pm yesterday", "15 yesterday"].iter() {
        let (start, end, _) = parse(phrase, Some(Config::new().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn alphabetic_5_6_69() {
    let then = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    for phrase in [
        "May 6, 1969",
        "May 6, '69",
        "May 6, 69",
        "6 May 1969",
        "6 May '69",
        "6 May 69",
        "Tuesday, May 6, 1969",
        "Tuesday, May 6, '69",
        "Tuesday, May 6, 69",
        "Tues, May 6, 1969",
        "Tues, May 6, '69",
        "Tues, May 6, 69",
        "Tue, May 6, 1969",
        "Tue, May 6, '69",
        "Tue, May 6, 69",
        "Tu, May 6, 1969",
        "Tu, May 6, '69",
        "Tu, May 6, 69",
        "Tues., May 6, 1969",
        "Tues., May 6, '69",
        "Tues., May 6, 69",
        "Tue., May 6, 1969",
        "Tue., May 6, '69",
        "Tue., May 6, 69",
        "Tu., May 6, 1969",
        "Tu., May 6, '69",
        "Tu., May 6, 69",
        "T, May 6, 1969",
        "T, May 6, '69",
        "T, May 6, 69",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
    }
}

#[test]
fn ymd_5_31_69() {
    let then = Date::from_calendar_date(1969, Month::May, 31).unwrap().midnight();
    for phrase in [
        "5-31-69",
        "5/31/69",
        "5.31.69",
        "5/31/1969",
        "5-31-1969",
        "5.31.1969",
        "69-5-31",
        "69/5/31",
        "69.5.31",
        "1969/5/31",
        "1969-5-31",
        "1969.5.31",
        "5-31-'69",
        "5/31/'69",
        "5.31.'69",
        "'69-5-31",
        "'69/5/31",
        "'69.5.31",
        "31-5-69",
        "31/5/69",
        "31.5.69",
        "31/5/1969",
        "31-5-1969",
        "31.5.1969",
        "69-31-5",
        "69/31/5",
        "69.31.5",
        "1969/31/5",
        "1969-31-5",
        "1969.31.5",
        "31-5-'69",
        "31/5/'69",
        "31.5.'69",
        "'69-31-5",
        "'69/31/5",
        "'69.31.5",
        "05-31-69",
        "05/31/69",
        "05.31.69",
        "05/31/1969",
        "05-31-1969",
        "05.31.1969",
        "69-05-31",
        "69/05/31",
        "69.05.31",
        "1969/05/31",
        "1969-05-31",
        "1969.05.31",
        "05-31-'69",
        "05/31/'69",
        "05.31.'69",
        "'69-05-31",
        "'69/05/31",
        "'69.05.31",
        "31-05-69",
        "31/05/69",
        "31.05.69",
        "31/05/1969",
        "31-05-1969",
        "31.05.1969",
        "69-31-05",
        "69/31/05",
        "69.31.05",
        "1969/31/05",
        "1969-31-05",
        "1969.31.05",
        "31-05-'69",
        "31/05/'69",
        "31.05.'69",
        "'69-31-05",
        "'69/31/05",
        "'69.31.05",
    ]
    .iter()
    {
        let (start, end, _) = parse(dbg!(phrase), None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
    }
}

#[test]
fn leap_day() {
    let rv = parse("2019-02-29", None);
    assert!(rv.is_err());
    let rv = parse("2020-02-29", None);
    assert!(rv.is_ok());
}

#[test]
fn may_1969() {
    let m1 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let m2 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    for phrase in ["May 1969", "May '69"].iter() {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(m1, start);
        assert_eq!(m2, end);
    }
}

#[test]
fn short_year_past_vs_future() {
    let m1 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let m2 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    let now = Date::from_calendar_date(2020, Month::May, 6).unwrap().midnight(); 
    let (start, end, _) = parse("May '69", Some(Config::new().now(now))).unwrap();
    assert_eq!(m1, start);
    assert_eq!(m2, end);
    let m1 = Date::from_calendar_date(2069, Month::May, 1).unwrap().midnight();
    let m2 = Date::from_calendar_date(2069, Month::June, 1).unwrap().midnight();
    let (start, end, _) = parse("May '69", Some(Config::new().now(now).default_to_past(false))).unwrap();
    assert_eq!(m1, start);
    assert_eq!(m2, end);
}

#[test]
fn this_month() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    let (start, end, _) = parse("this month", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_month() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::July, 1).unwrap().midnight();
    let (start, end, _) = parse("next month", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_month() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let (start, end, _) = parse("last month", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_year() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::January, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1970, Month::January, 1).unwrap().midnight();
    let (start, end, _) = parse("this year", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_year() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1970, Month::January, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1971, Month::January, 1).unwrap().midnight();
    let (start, end, _) = parse("next year", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_year() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1968, Month::January, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::January, 1).unwrap().midnight();
    let (start, end, _) = parse("last year", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_week() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse("this week", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn the_week() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse("the week", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_week() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 19).unwrap().midnight();
    let (start, end, _) = parse("next week", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_week() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 28).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse("last week", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_week_sunday_starts() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let (start, end, _) = parse(
        "this week",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_week_sunday_starts() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 18).unwrap().midnight();
    let (start, end, _) = parse(
        "next week",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_week_sunday_starts() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 27).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    let (start, end, _) = parse(
        "last week",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_pay_period() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1968, Month::May, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 18).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(format!("this {}", pp).as_ref(), Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}

#[test]
fn bare_pay_period() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1968, Month::May, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 18).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(pp, Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}
#[test]
fn next_pay_period() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1968, Month::May, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::May, 18).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(&format!("next {}", pp), Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}

#[test]
fn last_pay_period() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1968, Month::May, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::April, 20).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(&format!("last {}", pp), Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}

#[test]
fn this_pay_period_weird() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year *after* "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1970, Month::April, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 18).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(&format!("this {}", pp), Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}

#[test]
fn next_pay_period_weird() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year *after* "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1970, Month::April, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::May, 18).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(&format!("next {}", pp), Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}

#[test]
fn last_pay_period_weird() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    // two-week pay period beginning about a year *after* "now" on a Sunday
    let config = Config::new()
        .pay_period_start(Some(Date::from_calendar_date(1970, Month::April, 5).unwrap()))
        .pay_period_length(14)
        .now(now);
    let d1 = Date::from_calendar_date(1969, Month::April, 20).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 4).unwrap().midnight();
    for pp in ["pp", "pay period", "payperiod"].iter() {
        let (start, end, _) = parse(&format!("last {}", pp), Some(config.clone())).unwrap();
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    }
}

#[test]
fn this_april() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let (start, end, _) = parse("this april", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_april() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1970, Month::April, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1970, Month::May, 1).unwrap().midnight();
    let (start, end, _) = parse("next april", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_april() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1968, Month::April, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1968, Month::May, 1).unwrap().midnight();
    let (start, end, _) = parse("last april", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 9).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let (start, end, _) = parse("this friday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 16).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let (start, end, _) = parse("next friday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 2).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let (start, end, _) = parse("last friday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_monday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let (start, end, _) = parse("this monday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_monday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 13).unwrap().midnight();
    let (start, end, _) = parse("next monday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_monday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 28).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::April, 29).unwrap().midnight();
    let (start, end, _) = parse("last monday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn dawn_of_time() {
    let then = Date::MIN.midnight();
    for phrase in [
        "the beginning",
        "the beginning of time",
        "the first moment",
        "the start",
        "the very start",
        "the first instant",
        "the dawn of time",
        "the big bang",
        "the birth of the universe",
    ]
    .iter()
    {
        let (start, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::minutes(1), end);
    }
}

#[test]
fn the_crack_of_doom() {
    let then = Date::MAX.with_hms_milli(23, 59, 59, 999).unwrap();
    for phrase in [
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
    ]
    .iter()
    {
        let (_, end, _) = parse(phrase, None).unwrap();
        assert_eq!(then, end);
    }
}

#[test]
fn friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::May, 2).unwrap().midnight();
    let (start, end, _) = parse("Friday", Some(Config::new().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn tuesday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::April, 29).unwrap().midnight();
    let (start, end, _) = parse("Tuesday", Some(Config::new().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn monday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse("Monday", Some(Config::new().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn monday_default_to_future() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse(
        "Monday",
        Some(Config::new().now(now).default_to_past(false)),
    )
    .unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn friday_at_3_pm() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::May, 2).unwrap().with_hms(15, 0, 0).unwrap();
    let (start, end, _) = parse("Friday at 3 pm", Some(Config::new().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::seconds(1), end);
}

#[test]
fn tuesday_at_3_pm() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::April, 29).unwrap().with_hms(15, 0, 0).unwrap();
    let (start, end, _) = parse("Tuesday at 3 pm", Some(Config::new().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::seconds(1), end);
}

#[test]
fn monday_at_3_pm() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::May, 5).unwrap().with_hms(15, 0, 0).unwrap();
    let (start, end, _) = parse("Monday at 3 pm", Some(Config::new().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::seconds(1), end);
}

#[test]
fn monday_at_3_pm_default_to_future() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let then = Date::from_calendar_date(1969, Month::May, 12).unwrap().with_hms(15, 0, 0).unwrap();
    let (start, end, _) = parse(
        "Monday at 3 pm",
        Some(Config::new().now(now).default_to_past(false)),
    )
    .unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::seconds(1), end);
}

#[test]
fn just_may() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    let (start, end, _) = parse("May", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn just_april() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let (start, end, _) = parse("April", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn just_june() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1968, Month::June, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1968, Month::July, 1).unwrap().midnight();
    let (start, end, _) = parse("June", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn just_june_default_to_future() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::June, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::July, 1).unwrap().midnight();
    let (start, end, _) =
        parse("June", Some(Config::new().now(now).default_to_past(false))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn monday_through_friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let (start, end, _) = parse("Monday through Friday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn monday_through_friday_default_to_future() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let (start, end, _) = parse(
        "Monday through Friday",
        Some(Config::new().now(now).default_to_past(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn tuesday_through_friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 29).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let (start, end, _) = parse("Tuesday through Friday", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn tuesday_through_3_pm_on_friday() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::April, 29).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 2).unwrap().with_hms(15, 0, 1).unwrap();
    let (start, end, _) = parse(
        "Tuesday through 3 PM on Friday",
        Some(Config::new().now(now)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_year_through_today() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::January, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 7).unwrap().midnight();
    let (start, end, _) = parse("this year through today", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn noon_yesterday_through_midnight_today() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 5).unwrap().with_hms(12, 0, 0).unwrap();
    let d2 = Date::from_calendar_date(1969, Month::May, 7).unwrap().with_hms(0, 0, 1).unwrap();
    let (start, end, _) = parse(
        "noon yesterday through midnight today",
        Some(Config::new().now(now)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn very_specific_through_very_specific() {
    let d1 = Date::from_calendar_date(2014, Month::October, 6).unwrap().with_hms(8, 57, 29).unwrap();
    let d2 = Date::from_calendar_date(2020, Month::March, 6).unwrap().with_hms(17, 28, 34).unwrap();
    let (start, end, _) = parse("2014-10-06 08:57:29 - 2020-03-06 17:28:33", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn very_specific_up_to_very_specific() {
    let d1 = Date::from_calendar_date(2014, Month::October, 6).unwrap().with_hms(8, 57, 29).unwrap();
    let d2 = Date::from_calendar_date(2020, Month::March, 6).unwrap().with_hms(17, 28, 33).unwrap();
    let (start, end, _) = parse("2014-10-06 08:57:29 up to 2020-03-06 17:28:33", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn somewhat_specific_through_somewhat_specific() {
    let d1 = Date::from_calendar_date(2014, Month::October, 6).unwrap().with_hms(8, 57, 00).unwrap();
    let d2 = Date::from_calendar_date(2020, Month::March, 6).unwrap().with_hms(17, 28, 01).unwrap();
    let (start, end, _) = parse("2014-10-06 08:57 - 2020-03-06 17:28", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn somewhat_specific_up_to_somewhat_specific() {
    let d1 = Date::from_calendar_date(2014, Month::October, 6).unwrap().with_hms(8, 57, 00).unwrap();
    let d2 = Date::from_calendar_date(2020, Month::March, 6).unwrap().with_hms(17, 28, 00).unwrap();
    let (start, end, _) = parse("2014-10-06 08:57 up to 2020-03-06 17:28", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn april_3_25_bc() {
    let d1 = Date::from_calendar_date(-24, Month::April, 3).unwrap().midnight();
    let d2 = d1 + Duration::days(1);
    let (start, end, _) = parse("April 3, 25 BC", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn april_3_25_ad() {
    let d1 = Date::from_calendar_date(25, Month::April, 3).unwrap().midnight();
    let d2 = d1 + Duration::days(1);
    let (start, end, _) = parse("April 3, 25 AD", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn april_3_25bc() {
    let d1 = Date::from_calendar_date(-24, Month::April, 3).unwrap().midnight();
    let d2 = d1 + Duration::days(1);
    let (start, end, _) = parse("April 3, 25BC", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn april_3_25ad() {
    let d1 = Date::from_calendar_date(25, Month::April, 3).unwrap().midnight();
    let d2 = d1 + Duration::days(1);
    let (start, end, _) = parse("April 3, 25AD", None).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_weekend() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse("this weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_weekend() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse("last weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_weekend() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 19).unwrap().midnight();
    let (start, end, _) = parse("next weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_weekend_on_saturday() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse("this weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_weekend_on_saturday() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse("last weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_weekend_on_saturday() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 19).unwrap().midnight();
    let (start, end, _) = parse("next weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_weekend_on_sunday() {
    let now = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse("this weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_weekend_on_sunday() {
    let now = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse("last weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_weekend_on_sunday() {
    let now = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 19).unwrap().midnight();
    let (start, end, _) = parse("next weekend", Some(Config::new().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_weekend_on_sunday_when_sunday_starts_week() {
    let now = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse(
        "this weekend",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_weekend_on_sunday_when_sunday_starts_week() {
    let now = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse(
        "last weekend",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_weekend_on_sunday_when_sunday_starts_week() {
    let now = Date::from_calendar_date(1969, Month::May, 11).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 19).unwrap().midnight();
    let (start, end, _) = parse(
        "next weekend",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_weekend_on_saturday_when_sunday_starts_week() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 12).unwrap().midnight();
    let (start, end, _) = parse(
        "this weekend",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_weekend_on_saturday_when_sunday_starts_week() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 3).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 5).unwrap().midnight();
    let (start, end, _) = parse(
        "last weekend",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_weekend_on_saturday_when_sunday_starts_week() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 17).unwrap().midnight();
    let d2 = Date::from_calendar_date(1969, Month::May, 19).unwrap().midnight();
    let (start, end, _) = parse(
        "next weekend",
        Some(Config::new().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn regression_12pm() {
    let d1 = Date::from_calendar_date(2018, Month::May, 21).unwrap().midnight();
    let d2 = d1 + Duration::seconds(1);
    if let Ok((start, end, _)) = parse("12 pm on May 21, 2018", None) {
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    } else {
        assert!(false);
    }
}

#[test]
fn year_2000() {
    let d1 = Date::from_calendar_date(2000, Month::January, 1).unwrap().midnight();
    let d2 = Date::from_calendar_date(2001, Month::January, 1).unwrap().midnight();
    if let Ok((start, end, _)) = parse("2000", None) {
        assert_eq!(d1, start);
        assert_eq!(d2, end);
    } else {
        assert!(false);
    }
}

#[test]
fn ordinals() {
    let patterns = [
        (1, "1st", "first", "Monday"),
        (2, "2nd", "second", "Tuesday"),
        (3, "3rd", "third", "Wednesday"),
        (4, "4th", "fourth", "Thursday"),
        (5, "5th", "fifth", "Friday"),
        (6, "6th", "sixth", "Saturday"),
        (7, "7th", "seventh", "Sunday"),
        (8, "8th", "eighth", "Monday"),
        (9, "9th", "ninth", "Tuesday"),
        (10, "10th", "tenth", "Wednesday"),
        (11, "11th", "eleventh", "Thursday"),
        (12, "12th", "twelfth", "Friday"),
        (13, "13th", "thirteenth", "Saturday"),
        (14, "14th", "fourteenth", "Sunday"),
        (15, "15th", "fifteenth", "Monday"),
        (16, "16th", "sixteenth", "Tuesday"),
        (17, "17th", "seventeenth", "Wednesday"),
        (18, "18th", "eighteenth", "Thursday"),
        (19, "19th", "nineteenth", "Friday"),
        (20, "20th", "twentieth", "Saturday"),
        (21, "21st", "twenty-first", "Sunday"),
        (22, "22nd", "twenty-second", "Monday"),
        (23, "23rd", "twenty-third", "Tuesday"),
        (24, "24th", "twenty-fourth", "Wednesday"),
        (25, "25th", "twenty-fifth", "Thursday"),
        (26, "26th", "twenty-sixth", "Friday"),
        (27, "27th", "twenty-seventh", "Saturday"),
        (28, "28th", "twenty-eighth", "Sunday"),
        (29, "29th", "twenty-ninth", "Monday"),
        (30, "30th", "thirtieth", "Tuesday"),
        (31, "31st", "thirty-first", "Wednesday"),
    ];
    let base_date = Date::from_calendar_date(2018, Month::January, 1).unwrap().midnight();
    for (cardinal, abbv, ordinal, weekday) in patterns.iter() {
        let d1 = base_date + Duration::days(*cardinal as i64 - 1);
        let d2 = d1 + Duration::days(1);
        let subpatterns = [
            format!("January {}, 2018", abbv),
            format!("{}, January {}, 2018", weekday, abbv),
            format!("January {}, 2018", ordinal),
            format!("{}, January {}, 2018", weekday, ordinal),
            format!("the {} of January 2018", abbv),
            format!("{}, the {} of January 2018", weekday, abbv),
            format!("the {} of January 2018", ordinal),
            format!("{}, the {} of January 2018", weekday, ordinal),
        ];
        for p in subpatterns.iter() {
            match parse(p, None) {
                Ok((start, end, _)) => {
                    assert_eq!(d1, start);
                    assert_eq!(d2, end);
                }
                Err(e) => {
                    println!("{:?}", e);
                    assert!(false, "didn't match");
                }
            }
        }
    }
}

#[test]
fn kalends_nones_ids() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let mut i = Month::January;
    for m in months.iter() {
        let big_month = match i {
            Month::March | Month::May | Month::July | Month::October => true,
            _ => false,
        };
        // kalends
        let d1 = Date::from_calendar_date(2018, i, 1).unwrap().midnight();
        let d2 = d1 + Duration::days(1);
        let p = format!("the kalends of {} 2018", m);
        match parse(&p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
        // nones
        let d1 = Date::from_calendar_date(2018, i, if big_month { 7 } else { 5 }).unwrap().midnight();
        let d2 = d1 + Duration::days(1);
        let p = format!("the nones of {} 2018", m);
        match parse(&p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
        // ides
        let d1 = Date::from_calendar_date(2018, i, if big_month { 15 } else { 13 }).unwrap().midnight();
        let d2 = d1 + Duration::days(1);
        let p = format!("the ides of {} 2018", m);
        match parse(&p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
        i = i.next();
    }
}

#[test]
fn day_and_month() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 15).unwrap().midnight();
    let d2 = d1 + Duration::days(1);
    let patterns = [
        "the ides of May",
        "5-15",
        "the fifteenth",
        "May fifteenth",
        "May the 15th",
        "May the fifteenth",
    ];
    for p in patterns.iter() {
        match parse(p, Some(Config::new().now(now))) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn day_and_month_default_to_future() {
    let now = Date::from_calendar_date(1969, Month::June, 16).unwrap().midnight();
    let d1 = Date::from_calendar_date(1970, Month::May, 15).unwrap().midnight();
    let d2 = d1 + Duration::days(1);
    let patterns = [
        "the ides of May",
        "5-15",
        "May fifteenth",
        "May the 15th",
        "May the fifteenth",
    ];
    for p in patterns.iter() {
        match parse(p, Some(Config::new().now(now).default_to_past(false))) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn one_week_before_may_6_1969() {
    let d1 = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight() - Duration::days(7);
    let patterns = ["one week before May 6, 1969", "1 week before May 6, 1969"];
    for p in patterns.iter() {
        match parse(p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d1, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn one_week_after_may_6_1969() {
    let d1 = Date::from_calendar_date(1969, Month::May, 7).unwrap().midnight() + Duration::days(7);
    let patterns = ["one week after May 6, 1969", "1 week after May 6, 1969"];
    for p in patterns.iter() {
        match parse(p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d1, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn one_week_before_and_after_may_6_1969() {
    let d = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = d - Duration::days(7);
    let d2 = d + Duration::days(7);
    let patterns = [
        "one week before and after May 6, 1969",
        "1 week before and after May 6, 1969",
    ];
    for p in patterns.iter() {
        match parse(p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn one_week_around_may_6_1969() {
    let d1 = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight()
        - Duration::milliseconds(7 * 24 * 60 * 60 * 1000 / 2);
    let d2 = d1 + Duration::days(7);
    let patterns = ["one week around May 6, 1969", "1 week around May 6, 1969"];
    for p in patterns.iter() {
        match parse(p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d1, start);
                assert_eq!(d2, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn number_before_test() {
    let d = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(13, 0, 0).unwrap();
    let nums = [
        "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    for (i, p) in nums.iter().enumerate() {
        let d = d - Duration::seconds((i + 2) as i64);
        let p = format!("{} seconds before May 6, 1969 at 1:00 PM", p);
        match parse(&p, None) {
            Ok((start, end, _)) => {
                assert_eq!(d, start);
                assert_eq!(d, end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn noon() {
    let d1 = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(12, 0, 0).unwrap();
    let d2 = d1 + Duration::seconds(1);
    match parse("noon on May 6, 1969", None) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    match parse("noon on May 6, 1969", Some(Config::new().now(now))) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn simple_noon_past_and_future() {
    let now = Date::from_calendar_date(1969, Month::May, 6).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::May, 5).unwrap().with_hms(12, 0, 0).unwrap();
    let d2 = d1 + Duration::seconds(1);
    match parse("noon", Some(Config::new().now(now))) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
    let d1 = d1 + Duration::days(1);
    let d2 = d2 + Duration::days(1);
    match parse("noon", Some(Config::new().now(now).default_to_past(false))) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn midnight() {
    let d1 = Date::from_calendar_date(1969, Month::May, 7).unwrap().midnight();
    let d2 = d1 + Duration::seconds(1);
    match parse("midnight on May 6, 1969", None) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[derive(Debug)]
enum Period {
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

#[test]
fn displacement() {
    let displacements = [
        ("week", Period::Week),
        ("day", Period::Day),
        ("hour", Period::Hour),
        ("minute", Period::Minute),
        ("second", Period::Second),
    ];
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    for (phrase, period) in displacements.iter() {
        for n in [1, 2, 3].iter() {
            let phrase = if *n == 1 {
                String::from(*phrase)
            } else {
                String::from(*phrase) + "s"
            };
            let (displacement1, displacement2) = match period {
                Period::Week => (Duration::weeks(*n), Duration::weeks(1)),
                Period::Day => (Duration::days(*n), Duration::days(1)),
                Period::Hour => (Duration::hours(*n), Duration::hours(1)),
                Period::Minute => (Duration::minutes(*n), Duration::minutes(1)),
                _ => (Duration::seconds(*n), Duration::seconds(1)),
            };
            let d1 = now - displacement1;
            let d2 = d1 + displacement2;
            let expression = format!("{} {} ago", n, phrase);
            match parse(&expression, Some(Config::new().now(now))) {
                Ok((start, end, _)) => {
                    assert_eq!(d1, start);
                    assert_eq!(d2, end);
                }
                Err(e) => {
                    println!("{:?}", e);
                    assert!(false, "didn't match");
                }
            }
            let d1 = now + displacement1;
            let d2 = d1 + displacement2;
            let expression = format!("{} {} from now", n, phrase);
            match parse(&expression, Some(Config::new().now(now))) {
                Ok((start, end, _)) => {
                    assert_eq!(d1, start);
                    assert_eq!(d2, end);
                }
                Err(e) => {
                    println!("{:?}", e);
                    assert!(false, "didn't match");
                }
            }
        }
    }
}

#[test]
fn friday_the_13th() {
    let now = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1968, Month::December, 13).unwrap().midnight();
    match parse("Friday the 13th", Some(Config::new().now(now))) {
        Ok((start, _, _)) => {
            assert_eq!(d1, start);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn the_31st() {
    let now = Date::from_calendar_date(1969, Month::April, 10).unwrap().midnight();
    let d1 = Date::from_calendar_date(1969, Month::March, 31).unwrap().midnight();
    match parse("the 31st", Some(Config::new().now(now))) {
        Ok((start, _, _)) => {
            assert_eq!(d1, start);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn specific_time() {
    let d1 = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(12, 3, 5).unwrap();
    let d2 = d1 + Duration::seconds(1);
    match parse("1969-05-06 12:03:05", None) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn no_space_before_pm() {
    let d1 = Date::from_calendar_date(1969, Month::May, 6).unwrap().with_hms(13, 0, 0).unwrap();
    let d2 = d1 + Duration::seconds(1);
    match parse("1969-05-06 at 1PM", None) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
    match parse("1969-05-06 at 1:00PM", None) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
    match parse("1969-05-06 at 1:00:00PM", None) {
        Ok((start, end, _)) => {
            assert_eq!(d1, start);
            assert_eq!(d2, end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn relative_time_regression() {
    parse("24", None).unwrap();
    assert!(true, "'24' didn't cause a panic");
}

#[test]
fn since_yesterday() {
    let then = Date::from_calendar_date(1969, Month::May, 10).unwrap().midnight();
    let now = then + Duration::hours(5);
    match parse("since yesterday", Some(Config::new().now(now))) {
        Ok((start, end, two_times)) => {
            assert!(!two_times, "isn't a two-time expression");
            assert!(then == start);
            assert!(now == end);
        }
        Err(e) => {
            println!("{:?}", e);
            assert!(false, "didn't match");
        }
    }
}

#[test]
fn since_noon() {
    let then = Date::from_calendar_date(1969, Month::May, 10).unwrap().with_hms(12, 0, 0).unwrap();
    let now = then + Duration::hours(5);
    for expr in &[
        "since noon",
        "since noon today",
        "since 12",
        "since 12am",
        "since 12am today",
        "since 12:00",
        "since 12:00:00",
    ] {
        match parse(expr, Some(Config::new().now(now))) {
            Ok((start, end, two_times)) => {
                assert!(!two_times, "isn't a two-time expression");
                assert!(then == start);
                assert!(now == end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn since_may() {
    let then = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let now = then + Duration::hours(5);
    for expr in &[
        "since may",
        "since the start of may",
        "since the beginning of may",
        "after may",
        "after the start of may",
        "after the beginning of may",
    ] {
        match parse(expr, Some(Config::new().now(now))) {
            Ok((start, end, two_times)) => {
                assert!(!two_times, "isn't a two-time expression");
                assert!(then == start);
                assert!(now == end);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false, "didn't match");
            }
        }
    }
}

#[test]
fn since_the_end_of_may_misordered() {
    let then = Date::from_calendar_date(1969, Month::May, 1).unwrap().midnight();
    let now = then + Duration::hours(5);
    for expr in &["since the end of may", "after the end of may"] {
        match parse(expr, Some(Config::new().now(now))) {
            Ok((..)) => assert!(false, "this should not succeed"),
            Err(e) => match e {
                TimeError::Misordered(_) => assert!(true, "correct error"),
                _ => assert!(false, "unexpected error: {:?}", e),
            },
        }
    }
}
