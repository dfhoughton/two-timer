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
fn ymd_5_6_69() {
    let then = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    for phrase in [
        "5-6-69", "5/6/69", "5.6.69", "5/6/1969", "5-6-1969", "5.6.1969", "69-5-6", "69/5/6",
        "69.5.6", "1969/5/6", "1969-5-6", "1969.5.6", "5-6-'69", "5/6/'69", "5.6.'69", "'69-5-6",
        "'69/5/6", "'69.5.6",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
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
