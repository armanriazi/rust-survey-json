use pretty_assertions::assert_eq;
use proptest::prelude::*;
use rust_survey_json_lib::models::{State, Survey, Message};
use chrono;
use log::{debug, info};
extern crate serde_json;
use serde_json::Value;

mod common;
/// process test
///
/// ## Commands
///
///
/// ```cargo test  -p rust-survey-json_lib -- --show-output```
///
/// ```cargo test  -p rust-survey-json_lib -- --show-output --ignore```
///             
/// ```cargo doc  --package rust-survey-json_bin --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-survey-json_lib```
///
/// ## What
/// `A finder on json data for calculate the rate of users- there are 3 mode gain access to the json content"`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the uniqe empty type main function
///
/// # Return
/// 
/// `passed`
/// 

#[cfg(test)]

#[test]
fn process(){        
    let  index=5;
    let key="url".to_string();       
    let j = "
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";
    let value:Value= serde_json::from_str(j).unwrap();
    let mut state= State {   
    survey: Survey { 
        name: String::from("Simple Survey"), url: String::from(""), participant_count: 6, response_rate: 0.8333333333333334, submitted_response_count: 5,
    },
    user_id:1,
        datetime: String::from("2022-12-29 13:03:06.128576526 UTC"), 
        completed: false, 
        result: 3.8, 
        description: String::from("Successed, found user in a current survey-Simple Survey"),
    };

    state.process(Message::Update(index, key.clone(), value));    

}
