#![recursion_limit = "1024"]
#[macro_use]
extern crate pidgin;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
use chrono::offset::LocalResult;
use chrono::{Date, DateTime, Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
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
        period => <named_period> | <specific_period>
        specific_period => <modified_period> | <month_and_year>
        month_and_year -> <a_month> <year>
        named_period => <a_day> | <a_month>
        modified_period -> <modifier> <modifiable_period>
        modifier => [["this", "last", "next"]]
        modifiable_period => [["week", "month", "year", "pay period", "pp"]] | <a_month> | <a_day>
        moment -> <at_time_on>? <some_day> <at_time>? | <specific_time>
        specific_time => <time> | <absolute_terminus>
        absolute_terminus => <first_time> | <last_time>
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

#[derive(Debug, Clone)]
pub struct Config {
    now: DateTime<Utc>,
    monday_starts_week: bool,
    period: Period,
    pay_period_length: u32,
    pay_period_start: Option<Date<Utc>>,
}

impl Config {
    pub fn default() -> Config {
        Config {
            now: Utc::now(),
            monday_starts_week: true,
            period: Period::Minute,
            pay_period_length: 7,
            pay_period_start: None,
        }
    }
    pub fn now(&self, n: DateTime<Utc>) -> Config {
        let mut c = self.clone();
        c.now = n;
        c
    }
    pub fn period(&self, period: Period) -> Config {
        let mut c = self.clone();
        c.period = period;
        c
    }
    pub fn monday_starts_week(&self, monday_starts_week: bool) -> Config {
        let mut c = self.clone();
        c.monday_starts_week = monday_starts_week;
        c
    }
    pub fn pay_period_length(&self, pay_period_length: u32) -> Config {
        let mut c = self.clone();
        c.pay_period_length = pay_period_length;
        c
    }
    pub fn pay_period_start(&self, pay_period_start: Option<Date<Utc>>) -> Config {
        let mut c = self.clone();
        c.pay_period_start = pay_period_start;
        c
    }
}

pub fn parse(
    phrase: &str,
    config: Option<Config>,
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
        return Ok((first_moment(), last_moment()));
    }
    let parse = parse.name("particular").unwrap();
    let config = config.unwrap_or(Config::default());
    if let Some(moment) = parse.name("one_time") {
        return handle_one_time(moment, &config);
    }
    if let Some(two_times) = parse.name("two_times") {
        let first = &two_times.children().unwrap()[0];
        let last = &two_times.children().unwrap()[2];
        if specific(first, true) {
            if specific(last, true) {
                return match specific_moment(first, &config) {
                    Ok((d1, _)) => match specific_moment(last, &config) {
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
                return match specific_moment(first, &config) {
                    Ok((d1, _)) => {
                        let (_, d2) = relative_moment(last, &config, &d1, false);
                        Ok((d1, d2))
                    }
                    Err(s) => Err(s),
                };
            }
        } else if specific(last, false) {
            return match specific_moment(last, &config) {
                Ok((_, d2)) => {
                    let (d1, _) = relative_moment(first, &config, &d2, true);
                    Ok((d1, d2))
                }
                Err(s) => Err(s),
            };
        } else {
            let (_, d2) = relative_moment(last, &config, &config.now, true);
            let (d1, _) = relative_moment(first, &config, &d2, true);
            return Ok((d1, d2));
        }
    }
    unreachable!();
}

fn first_moment() -> DateTime<Utc> {
    chrono::MIN_DATE.and_hms_milli(0, 0, 0, 0)
}

fn last_moment() -> DateTime<Utc> {
    chrono::MAX_DATE.and_hms_milli(23, 59, 59, 999)
}

fn specific(m: &Match, first: bool) -> bool {
    m.has("specific_day")
        || m.has("specific_period")
        || m.has("specific_time") && (first || m.has("absolute_terminus"))
}

fn handle_specific_day(
    m: &Match,
    config: &Config,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    let now = config.now.clone();
    let mut times = m.all_names("time");
    if times.len() > 1 {
        return Err(format!("more than one daytime specified in {}", m.as_str()));
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
                    &Config::default()
                        .now(now + Duration::days(1))
                        .period(Period::Day),
                    time,
                )),
                _ => unreachable!(),
            },
            // yesterday
            'y' | 'Y' => Ok(moment_and_time(
                &Config::default()
                    .now(now - Duration::days(1))
                    .period(Period::Day),
                time,
            )),
            _ => unreachable!(),
        };
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
                    Ok(moment_and_time(
                        &Config::default().now(d1).period(Period::Day),
                        time,
                    ))
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
                            Ok(moment_and_time(
                                &Config::default().now(d1).period(Period::Day),
                                time,
                            ))
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
                        Ok(moment_and_time(
                            &Config::default().now(d1).period(Period::Day),
                            time,
                        ))
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

fn handle_specific_period(
    moment: &Match,
    config: &Config,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    if let Some(moment) = moment.name("month_and_year") {
        let y = year(moment, &config.now);
        let m = a_month(moment);
        return match Utc.ymd_opt(y, m, 1) {
            LocalResult::None => unreachable!(),
            LocalResult::Single(d1) => {
                let d1 = d1.and_hms(0, 0, 0);
                Ok(moment_and_time(
                    &Config::default().now(d1).period(Period::Month),
                    None,
                ))
            }
            LocalResult::Ambiguous(_, _) => unreachable!(),
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
                    Err(String::from("no pay period start date provided"))
                }
            }
        };
    }
    unreachable!()
}

enum ModifiablePeriod {
    Week,
    Month,
    Year,
    PayPeriod,
}

impl ModifiablePeriod {
    fn from_match(m: &Match) -> ModifiablePeriod {
        match m.as_str().chars().nth(0).expect("unreachable") {
            'w' | 'W' => ModifiablePeriod::Week,
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
    other_time: &DateTime<Utc>,
    before: bool,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    if let Some(t) = moment.name("absolute_terminus") {
        return if t.has("first_time") {
            Ok(moment_to_period(first_moment(), &config.period, config))
        } else {
            Ok((last_moment(), last_moment()))
        };
    }
    if let Some(t) = moment.name("time") {
        let (hour, minute, second) = time(t);
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
        if before && t > *other_time {
            t = t - Duration::days(1);
        } else if !before && t < *other_time {
            t = t + Duration::days(1);
        }
        return Ok(moment_to_period(t, &period, config));
    } else {
        unreachable!();
    }
}

fn handle_one_time(
    moment: &Match,
    config: &Config,
) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    if moment.has("specific_day") {
        return handle_specific_day(moment, config);
    }
    if let Some(moment) = moment.name("specific_period") {
        return handle_specific_period(moment, config);
    }
    if let Some(moment) = moment.name("specific_time") {
        return handle_specific_time(moment, config, &config.now, true);
    }
    unimplemented!();
}

// add time to a date
fn moment_and_time(config: &Config, daytime: Option<&Match>) -> (DateTime<Utc>, DateTime<Utc>) {
    if let Some(daytime) = daytime {
        let (hour, minute, second) = time(daytime);
        let period = if second.is_some() {
            Period::Second
        } else if minute.is_some() {
            Period::Minute
        } else {
            Period::Hour
        };
        let m = config
            .now
            .with_hour(hour)
            .unwrap()
            .with_minute(minute.unwrap_or(0))
            .unwrap()
            .with_second(second.unwrap_or(0))
            .unwrap();
        moment_to_period(m, &period, config)
    } else {
        moment_to_period(config.now, &config.period, config)
    }
}

fn relative_moment(
    m: &Match,
    config: &Config,
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
            let mut t = other_time
                .with_hour(hour)
                .unwrap()
                .with_minute(minute.unwrap_or(0))
                .unwrap()
                .with_second(second.unwrap_or(0))
                .unwrap();
            if before && t > *other_time {
                t = t - Duration::days(1);
            } else if !before && t < *other_time {
                t = t + Duration::days(1);
            }
            return moment_to_period(t, &period, config);
        } else {
            unreachable!();
        }
    }
    unimplemented!();
}

fn specific_moment(m: &Match, config: &Config) -> Result<(DateTime<Utc>, DateTime<Utc>), String> {
    if let Some(m) = m.name("specific_day") {
        return handle_specific_day(m, config);
    }
    if let Some(m) = m.name("specific_period") {
        return handle_specific_period(m, config);
    }
    if let Some(m) = m.name("specific_time") {
        return handle_specific_time(m, config, &config.now, true);
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
fn moment_to_period(
    now: DateTime<Utc>,
    period: &Period,
    config: &Config,
) -> (DateTime<Utc>, DateTime<Utc>) {
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
            let offset = if config.monday_starts_week {
                now.weekday().num_days_from_monday()
            } else {
                now.weekday().num_days_from_sunday()
            };
            let d1 = Utc.ymd(now.year(), now.month(), now.day()).and_hms(0, 0, 0)
                - Duration::days(offset as i64);
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
pub enum Period {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Nanosecond,
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
