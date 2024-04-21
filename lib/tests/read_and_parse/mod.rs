use lib::parser::parse_string;

use regex::Regex;
use std::fs;

pub fn test(filename: &str, test_name: &str) {
    // We need to find the unique string of the following form:
    // =============|||
    // ${test_name}
    // =============|||
    //
    // ---- MODULE Test ----
    // op(x) == lbl :: x
    // ====
    //
    // -------------|||
    //
    // Everything between the second set of =============||| and -------------||| is the test case.

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // Forgive me father for I have sinned.
    // TODO capture up until the first ----||| instead of greedily.
    let rx = Regex::new(&format!(
        "=+\\|\\|\\|\\n{}\\n=+\\|\\|\\|\\n((.|\\n)*)\\n-+\\|\\|\\|",
        test_name
    ))
    .unwrap();
    let captures = rx.captures(&contents).unwrap();
    let contents = captures.get(1).unwrap().as_str().to_string();
    parse_string(&contents).unwrap();
}
