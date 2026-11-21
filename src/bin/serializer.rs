// for constructing a serialized matcher
pub fn main() {
    println!(
        "serde_json::from_str(r#\"{}\"#).unwrap();",
        serde_json::to_string(&two_timer::GRAMMAR.matcher().unwrap()).unwrap()
    );
}
