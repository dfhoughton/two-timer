#![recursion_limit = "1024"]
#[macro_use]
extern crate pidgin;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
use chrono::offset::LocalResult;
use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use pidgin::{Grammar, Match, Matcher};
use regex::Regex;

lazy_static! {
    static ref GRAMMAR: Grammar = grammar!{
        (?ibBw)

        TOP -> r(r"\A") <something> r(r"\z")

        something => <universal> | <particular>
        universal => [["always", "ever", "all time", "forever", "from beginning to end", "from the beginning to the end"]]
        particular => <one_time> | <two_times>
        one_time => <moment_or_period>
        two_times -> <moment_or_period> <to> <moment_or_period>
        to => [["to", "through", "until", "up to", "thru", "till"]] | r("-+")
        moment_or_period => <moment> | <period>
        period => <named_period> | <modified_period>
        named_period => <a_day> | <a_month> <year>?
        modified_period => <modifier> <modifiable_period>
        modifier => [["this", "last", "next"]]
        modifiable_period => [["week", "month", "year", "pay period", "pp"]] | <a_month> | <a_day>
        moment -> <at_time_on>? <some_day> <at_time>? | <time>
        some_day => <specific_day> | <relative_day>
        specific_day => <adverb> | <date_with_year>
        relative_day => ("bar")
        adverb => [["now", "today", "tomorrow", "yesterday"]]
        date_with_year => <n_date> | <a_date>
        at_time -> ("at") <time>
        at_time_on -> ("at")? <time> ("on")
        time -> <hour_12> <am_pm>? | <hour_24>
        hour_24 => <h24>
        hour_24 => <h24> (":") <minute>
        hour_24 => <h24> (":") <minute> (":") <second>
        hour_12 => <h12>
        hour_12 => <h12> (":") <minute>
        hour_12 => <h12> (":") <minute> (":") <second>
        minute => [ (0..60).into_iter().map(|i| format!("{:02}", i)).collect::<Vec<_>>() ]
        second => [ (0..60).into_iter().map(|i| format!("{:02}", i)).collect::<Vec<_>>() ]
        am_pm => (?-i) [["am", "AM", "pm", "PM", "a.m.", "A.M.", "p.m.", "P.M."]]
        h12 => [(1..=12).into_iter().collect::<Vec<_>>()]
        h24 => [(1..=24).into_iter().collect::<Vec<_>>()]
        n_date -> <year>    ("/") <n_month> ("/") <n_day>
        n_date -> <year>    ("-") <n_month> ("-") <n_day>
        n_date -> <year>    (".") <n_month> (".") <n_day>
        n_date -> <year>    ("/") <n_day>   ("/") <n_month>
        n_date -> <year>    ("-") <n_day>   ("-") <n_month>
        n_date -> <year>    (".") <n_day>   (".") <n_month>
        n_date -> <n_month> ("/") <n_day>   ("/") <year>
        n_date -> <n_month> ("-") <n_day>   ("-") <year>
        n_date -> <n_month> (".") <n_day>   (".") <year>
        n_date -> <n_day>   ("/") <n_month> ("/") <year>
        n_date -> <n_day>   ("-") <n_month> ("-") <year>
        n_date -> <n_day>   (".") <n_month> (".") <year>
        a_date -> <a_month> <n_day> (",") <year>
        a_date -> <n_day> <a_month> <year>
        a_date -> <a_day> (",") <a_month> <n_day> (",") <year>
        year => [
                (100..=3000)
                    .into_iter()
                    .collect::<Vec<_>>()
            ]
        year => [
                (0..=99)
                    .into_iter()
                    .flat_map(|i| vec![format!("'{:02}", i), format!("{:02}", i)])
                    .collect::<Vec<_>>()
            ]
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
        a_day => (?-i) [["M", "T", "W", "R", "F", "S", "U"]]
        a_month => [
                "January February March April May June July August September October November December"
                     .split(" ")
                     .into_iter()
                     .flat_map(|w| vec![w.to_string(), w[0..3].to_string()])
                     .collect::<Vec<_>>()
            ]
    };
}
lazy_static! {
    static ref MATCHER: Matcher = GRAMMAR.matcher().unwrap();
}

pub fn parse(
    phrase: &str,
    now: Option<&DateTime<Utc>>,
    period: Option<Period>,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    let parse = MATCHER.parse(phrase);
    if parse.is_none() {
        return Err(format!(
            "could not parse \"{}\" as a time expression",
            phrase
        ));
    }
    let parse = parse.unwrap();
    if parse.has("universal") {
        return Ok((
            chrono::MIN_DATE.and_hms_milli(0, 0, 0, 0),
            chrono::MAX_DATE.and_hms_milli(23, 59, 59, 999),
        ));
    }
    let parse = parse.name("particular").unwrap();
    let now = if now.is_some() {
        now.unwrap().clone()
    } else {
        Utc::now()
    };
    let period = if period.is_some() {
        period.unwrap()
    } else {
        Period::Minute
    };
    if let Some(moment) = parse.name("one_time") {
        if moment.has("specific_day") {
            return specific_moment(moment, &now, &period);
        }
        if moment.has("relative_day") || moment.has("time") {
            return Ok(relative_moment(moment, &now, &now, true));
        }
        unreachable!();
    }
    if let Some(two_times) = parse.name("two_times") {
        let first = &two_times.children().unwrap()[0];
        let last = &two_times.children().unwrap()[2];
        if first.has("specific_day") {
            if last.has("specific_day") {
                return match specific_moment(first, &now, &period) {
                    Ok((d1, _)) => match specific_moment(last, &now, &period) {
                        Ok((_, d2)) => {
                            if d1 <= d2 {
                                Ok((d1, d2))
                            } else {
                                Err(format!("{} is after {}", first.as_str(), last.as_str()))
                            }
                        }
                        Err(s) => Err(s),
                    },
                    Err(s) => Err(s),
                };
            } else {
                return match specific_moment(first, &now, &period) {
                    Ok((d1, _)) => {
                        let (_, d2) = relative_moment(last, &now, &d1, false);
                        Ok((d1, d2))
                    }
                    Err(s) => Err(s),
                };
            }
        } else if last.has("specific_day") {
            return match specific_moment(last, &now, &period) {
                Ok((_, d2)) => {
                    let (d1, _) = relative_moment(first, &now, &d2, true);
                    Ok((d1, d2))
                }
                Err(s) => Err(s),
            };
        } else {
            let (_, d2) = relative_moment(last, &now, &now, true);
            let (d1, _) = relative_moment(first, &now, &d2, true);
            return Ok((d1, d2));
        }
    }
    unreachable!();
}

// add time to a date
fn moment_and_time(
    m: DateTime<Utc>,
    default_period: &Period,
    daytime: Option<&Match>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    if let Some(daytime) = daytime {
        let (hour, minute, second) = time(daytime);
        let period = if second.is_some() {
            Period::Second
        } else if minute.is_some() {
            Period::Minute
        } else {
            Period::Hour
        };
        let m = m
            .with_hour(hour)
            .unwrap()
            .with_minute(minute.unwrap_or(0))
            .unwrap()
            .with_second(second.unwrap_or(0))
            .unwrap();
        moment_to_period(m, &period)
    } else {
        moment_to_period(m, default_period)
    }
}

fn relative_moment(
    m: &Match,
    now: &DateTime<Utc>,
    other_time: &DateTime<Utc>,
    before: bool,
) -> (DateTime<Utc>, DateTime<Utc>) {
    if !m.has("some_day") {
        // necessarily just time
        if let Some(t) = m.name("time") {
            let (hour, minute, second) = time(t);
            let period = if second.is_some() {
                Period::Second
            } else if minute.is_some() {
                Period::Minute
            } else {
                Period::Hour
            };
            let mut t = other_time.with_hour(hour).unwrap().with_minute(minute.unwrap_or(0)).unwrap().with_second(second.unwrap_or(0)).unwrap();
            if before && t > *other_time {
                t = t - Duration::days(1);
            } else if !before && t < *other_time {
                t = t + Duration::days(1);
            }
            return moment_to_period(t, &period)
        } else {
            unreachable!();
        }
    }
    unimplemented!();
}

fn specific_moment(
    m: &Match,
    now: &DateTime<Utc>,
    period: &Period,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    let now = now.clone();
    let mut times = m.all_names("time");
    if times.len() > 1 {
        return Err(format!("more than one daytime specified in {}", m.as_str()));
    }
    let time = times.pop();
    if let Some(adverb) = m.name("adverb") {
        return match adverb.as_str().chars().nth(0).expect("empty string") {
            // now
            'n' | 'N' => Ok(moment_and_time(now, period, time)),
            't' | 'T' => match adverb.as_str().chars().nth(2).expect("impossible string") {
                // today
                'd' | 'D' => Ok(moment_and_time(now, &Period::Day, time)),
                // tomorrow
                'm' | 'M' => Ok(moment_and_time(now + Duration::days(1), &Period::Day, time)),
                _ => unreachable!()
            },
            // yesterday
            'y' | 'Y' => Ok(moment_and_time(now - Duration::days(1), &Period::Day, time)),
            _ => unreachable!()
        }
    }
    if let Some(date) = m.name("date_with_year") {
        if let Some(date) = date.name("n_date") {
            let year = year(date, &now);
            let month = n_month(date);
            let day = n_day(date);
            let d_opt = Utc.ymd_opt(year, month, day);
            return match d_opt {
                LocalResult::None => Err(format!(
                    "cannot construct UTC date with year {}, month {}, and day {}",
                    year, month, day
                )),
                LocalResult::Single(d1) => {
                    let d1 = d1.and_hms(0, 0, 0);
                    Ok(moment_and_time(d1, &Period::Day, time))
                }
                LocalResult::Ambiguous(_, _) => Err(format!(
                    "cannot construct unambiguous UTC date with year {}, month {}, and day {}",
                    year, month, day
                )),
            };
        }
        if let Some(date) = date.name("a_date") {
            let year = year(date, &now);
            let month = a_month(date);
            let day = n_day(date);
            let d_opt = Utc.ymd_opt(year, month, day);
            return match d_opt {
                LocalResult::None => Err(format!(
                    "cannot construct UTC date with year {}, month {}, and day {}",
                    year, month, day
                )),
                LocalResult::Single(d1) => {
                    if let Some(wd) = date.name("a_day") {
                        let wd = weekday(wd.as_str());
                        if wd == d1.weekday() {
                            let d1 = d1.and_hms(0, 0, 0);
                            Ok(moment_and_time(d1, &Period::Day, time))
                        } else {
                            Err(format!(
                                "the weekday of year {}, month {}, day {} is not {}",
                                year,
                                month,
                                day,
                                date.name("a_day").unwrap().as_str()
                            ))
                        }
                    } else {
                        let d1 = d1.and_hms(0, 0, 0);
                        Ok(moment_and_time(d1, &Period::Day, time))
                    }
                }
                LocalResult::Ambiguous(_, _) => Err(format!(
                    "cannot construct unambiguous UTC date with year {}, month {}, and day {}",
                    year, month, day
                )),
            };
        }
        unreachable!();
    }
    unimplemented!();
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
fn time(m: &Match) -> (u32, Option<u32>, Option<u32>) {
    let hour = if let Some(hour_24) = m.name("hour_24") {
        s_to_n(hour_24.name("h24").unwrap().as_str())
    } else if let Some(hour_12) = m.name("hour_12") {
        let hour = s_to_n(hour_12.name("h12").unwrap().as_str());
        if let Some(am_pm) = m.name("am_pm") {
            match am_pm.as_str().chars().nth(0).expect("empty string") {
                'a' | 'A' => hour,
                _ => hour + 12,
            }
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
            (hour, Some(minute), Some(second))
        } else {
            (hour, Some(minute), None)
        }
    } else {
        (hour, None, None)
    }
}

fn n_month(m: &Match) -> u32 {
    lazy_static! {
        static ref MONTH: Regex = Regex::new(r"\A0?(\d{1,2})\z").unwrap();
    }
    let cap = MONTH.captures(m.name("n_month").unwrap().as_str()).unwrap();
    cap[1].parse::<u32>().unwrap()
}

fn year(m: &Match, now: &DateTime<Utc>) -> i32 {
    lazy_static! {
        static ref YEAR: Regex = Regex::new(r"\A(?:'0?|0)?(\d{1,2})\z").unwrap();
    }
    let year = m.name("year").unwrap().as_str();
    let cap = YEAR.captures(year);
    if let Some(cap) = cap {
        // year is assumed to be in the current century
        let y = cap[1].parse::<i32>().unwrap();
        let this_year = now.year() % 100;
        if this_year < y {
            now.year() - this_year - 100 + y
        } else {
            now.year() - this_year + y
        }
    } else {
        year.parse::<i32>().unwrap()
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

/// expand a moment to the period containing it
fn moment_to_period(now: DateTime<Utc>, period: &Period) -> (DateTime<Utc>, DateTime<Utc>) {
    match period {
        Period::Year => {
            let d1 = Utc.ymd(now.year(), 1, 1).and_hms(0, 0, 0);
            let d2 = Utc.ymd(now.year() + 1, 1, 1).and_hms(0, 0, 0);
            (d1, d2)
        }
        Period::Month => {
            let d1 = Utc.ymd(now.year(), now.month(), 1).and_hms(0, 0, 0);
            let d2 = if now.month() == 12 {
                Utc.ymd(now.year() + 1, 1, 1)
            } else {
                Utc.ymd(now.year(), now.month() + 1, 1)
            }
            .and_hms(0, 0, 0);
            (d1, d2)
        }
        Period::Week => {
            let d1 = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0)
                - Duration::days(now.weekday().num_days_from_monday() as i64);
            (d1, d1 + Duration::days(7))
        }
        Period::WeekStartingSunday => {
            let d1 = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0)
                - Duration::days(now.weekday().num_days_from_sunday() as i64);
            (d1, d1 + Duration::days(7))
        }
        Period::Day => {
            let d1 = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0);
            (d1, d1 + Duration::days(1))
        }
        Period::Hour => {
            let d1 = Utc
                .ymd(now.year(), now.month(), now.day())
                .and_hms(now.hour(), 0, 0);
            (d1, d1 + Duration::hours(1))
        }
        Period::Minute => {
            let d1 =
                Utc.ymd(now.year(), now.month(), now.day())
                    .and_hms(now.hour(), now.minute(), 0);
            (d1, d1 + Duration::minutes(1))
        }
        Period::Second => {
            let d1 = Utc.ymd(now.year(), now.month(), now.day()).and_hms(
                now.hour(),
                now.minute(),
                now.second(),
            );
            (d1, d1 + Duration::seconds(1))
        }
        Period::Nanosecond => (now, now + Duration::nanoseconds(1)),
    }
}

pub enum Period {
    Year,
    Month,
    Week,
    WeekStartingSunday,
    Day,
    Hour,
    Minute,
    Second,
    Nanosecond,
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
