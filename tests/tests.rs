#![feature(test)]
extern crate two_timer;
use two_timer::parse;
extern crate chrono;
use chrono::{Duration, TimeZone, Utc};

#[test]
fn always() {
    let alpha = chrono::MIN_DATE.and_hms_milli(0, 0, 0, 0);
    let omega = chrono::MAX_DATE.and_hms_milli(23, 59, 59, 999);
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
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(alpha, start);
        assert_eq!(omega, end);
    }
}

#[test]
fn yesterday() {
    let now = Utc::now();
    let (start, end) = parse("yesterday", Some(&now), None).unwrap();
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
    let now = Utc::now();
    let (start, end) = parse("tomorrow", Some(&now), None).unwrap();
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
    let now = Utc::now();
    let (start, end) = parse("today", Some(&now), None).unwrap();
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
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 30, 0);
    for phrase in [
        "at 3:30 PM on 5-6-69",
        "3:30 p.m. on 5-6-69",
        "at 15:30 on 5-6-69",
        "15:30 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::minutes(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in [
        "at 3 PM on 5-6-69",
        "3 p.m. on 5-6-69",
        "at 15 on 5-6-69",
        "15 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::hours(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_00_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 30, 0);
    for phrase in [
        "at 3:30:00 PM on 5-6-69",
        "3:30:00 p.m. on 5-6-69",
        "at 15:30:00 on 5-6-69",
        "15:30:00 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_01_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 30, 1);
    for phrase in [
        "at 3:30:01 PM on 5-6-69",
        "3:30:01 p.m. on 5-6-69",
        "at 15:30:01 on 5-6-69",
        "15:30:01 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_01_am() {
    let then = Utc.ymd(1969, 5, 6).and_hms(3, 30, 1);
    for phrase in [
        "at 3:30:01 AM on 5-6-69",
        "3:30:01 a.m. on 5-6-69",
        "at 3:30:01 on 5-6-69",
        "3:30:01 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(16, 0, 0);
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in ["3 PM", "3 pm", "15"].iter() {
        let (start, end) = parse(phrase, Some(&now), None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::hours(1), end);
    }
}

#[test]
fn at_3_00_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(16, 0, 0);
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in ["3:00 PM", "3:00 pm", "15:00"].iter() {
        let (start, end) = parse(phrase, Some(&now), None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::minutes(1), end);
    }
}

#[test]
fn at_3_00_00_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(16, 0, 0);
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in ["3:00:00 PM", "3:00:00 pm", "15:00:00"].iter() {
        let (start, end) = parse(phrase, Some(&now), None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm_yesterday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(14, 0, 0);
    let then = Utc.ymd(1969, 5, 5).and_hms(15, 0, 0);
    for phrase in ["3 PM", "3 pm", "15"].iter() {
        let (start, end) = parse(phrase, Some(&now), None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::hours(1), end);
    }
}

#[test]
fn alphabetic_5_6_69() {
    let then = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
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
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
    }
}

#[test]
fn ymd_5_31_69() {
    let then = Utc.ymd(1969, 5, 31).and_hms(0, 0, 0);
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
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
    }
}

#[test]
fn leap_day() {
    let rv = parse("2019-02-29", None, None);
    assert!(rv.is_err());
    let rv = parse("2020-02-29", None, None);
    assert!(rv.is_ok());
}
