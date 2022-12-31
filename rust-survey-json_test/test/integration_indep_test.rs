use self::common::adder;

/// process test
///
/// ## Commands
///
///
/// ```cd rust-survey-json_test```
/// 
/// ```cargo test  -p rust-survey-json_test -- --show-output```
///
/// ```cargo test  -p rust-survey-json_test -- --show-output --ignore```
///             
/// ```cargo doc  --package rust-survey-json_test --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-survey-json_test```
///
/// ## What
/// `TODO`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `TODO`
///
/// # Return
/// 
/// `passed`
/// 

#[cfg(test)]

use pretty_assertions::assert_eq;
use proptest::prelude::*;
mod common;


#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}


#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}