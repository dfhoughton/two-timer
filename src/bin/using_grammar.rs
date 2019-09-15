extern crate two_timer;

// for timing the cost savings of using a serialized matcher
fn main() {
    two_timer::GRAMMAR.matcher().unwrap().parse("yesterday");
}