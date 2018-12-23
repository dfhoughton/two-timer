#![recursion_limit="512"]
#[macro_use]
extern crate pidgin;

fn main() {
    let g = grammar!{
        (?ibBw)

        TOP => <universal> | <existential>
        existential => <specific> | <relative>
        specific => ("foo")
        relative => ("bar")
        universal => [["always", "ever", "all time"]]
        existential => <date> | <two_times>
        two_times => <two_dates> | <on_date>
        two_dates -> <date> <date_separator> <date>
        on_date -> [["on"]]? <date> [["from"]] <time> [["to"]] <time>
        date_separator => [["-", "through", "to", "until", "till", "til", "thru"]]
        date => <specific> | <relative>
        month => <a_month> | <n_month>
        time -> <hour_12> <am_pm> | <hour_24>
        hour_24 => <h24>
        hour_24 => <h24> (":") <minute>
        hour_24 => <h24> (":") <minute> (":") <second>
        hour_12 => <h12>
        hour_12 => <h12> (":") <minute>
        hour_12 => <h12> (":") <minute> (":") <second>
        minute => [ (0..60).into_iter().map(|i| format!("'{:02}", i)).collect::<Vec<_>>() ]
        second => [ (0..60).into_iter().map(|i| format!("'{:02}", i)).collect::<Vec<_>>() ]
        am_pm => (?-i) [["am", "AM", "pm", "PM", "a.m.", "A.M.", "p.m.", "P.M."]]
        h12 => [(1..=12).into_iter().collect::<Vec<_>>()]
        h24 => [(1..=24).into_iter().collect::<Vec<_>>()]
        day => <a_day> | <n_date>
        a_day => [
                "Sunday Monday Tuesday Wednesday Thursday Friday Saturday"
                    .split(" ")
                    .into_iter()
                    .flat_map(|w| vec![w.to_string(), w[0..2].to_string(), w[0..3].to_string()])
                    .collect::<Vec<_>>()
            ]
        a_day => (?-i) [["M", "T", "W", "R", "F", "S", "U"]]
        n_date -> <year> ("/") <n_month> ("/") <n_day>
        n_date -> <year> ("-") <n_month> ("-") <n_day>
        n_date -> <year> (".") <n_month> (".") <n_day>
        n_date -> <year> ("/") <n_day>   ("/") <n_month>
        n_date -> <year> ("-") <n_day>   ("-") <n_month>
        n_date -> <year> (".") <n_day>   (".") <n_month>
        year => [
                (1..=3000)
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
        n_month => [(1..12).into_iter().collect::<Vec<_>>()]
        a_month => [
                "January February March April May June July August September October November December"
                     .split(" ")
                     .into_iter()
                     .flat_map(|w| vec![w.to_string(), w[0..3].to_string()])
                     .collect::<Vec<_>>()
            ]
    };
    println!("Hello, world!");
}
