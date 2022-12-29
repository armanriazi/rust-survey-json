#![allow(dead_code, unused_variables, unused_imports)]

use proptest::prelude::*;
//---
use super::{error::CustomError, *};
use crate::models::{Message, State, Survey};
use chrono;
use log::{debug, info};
extern crate serde_json;
use serde_json::Value;


#[allow(dead_code)]
#[allow(unused_mut)]
pub fn json_factory<F>(f: F, user_id: u32) -> Result<(), CustomError>
where
    F: FnOnce() -> Result<serde_json::Value, CustomError>,
{
    let serde_values_records: serde_json::Value = serde_json::from_value(f().unwrap())?;
    let value_fields: serde_json::Value = serde_values_records["survey_result_detail"].clone();
    let _count = value_fields
        .as_object()
        .unwrap()
        .into_iter()
        .enumerate()
        .count();
    debug!("\n Number of keys {:?}\n", _count);

    let mut state = State {
        completed: false,
        survey: Survey::new(String::default(), String::default(), 0, 0.0, 0),
        datetime: chrono::offset::Utc::now().to_string(),
        description: String::default(),
        result: 0.0,
        user_id: user_id,
    };

    value_fields
        .as_object()
        .unwrap()
        .into_iter()
        .enumerate()
        .for_each(|(_i, value_of_key)| {
            process_message_call(
                &mut state,
                _i,
                value_of_key.0.to_string(),
                value_of_key.1.to_owned(),
            );
            //state.process(Message::IsCompleted(true));
        });

    state.process(Message::IsCompleted(true));
    Ok(())
}

pub fn process_message_call(state: &mut State, index: usize, key: String, value: Value) {
    state.process(Message::Update(index, key.clone(), value));
    state.process(Message::Echo(String::from(format!(
        "Making model(updated key successfuly):{:?}",
        key
    ))));
    state.process(Message::IsCompleted(false));
}

