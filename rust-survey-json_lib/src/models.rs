#![allow(dead_code, unused_variables, unused_imports)]
use chrono;
use log::debug;
use serde_json::Value;
use std::ops::Add;
use std::time::SystemTime;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Survey {
    pub name: String,
    pub url: String,
    pub participant_count: i32,
    pub response_rate: f64,
    pub submitted_response_count: i32,
}
#[derive(PartialEq)]
pub enum Message {
    Update(usize, String, Value),
    Echo(String),
    IsCompleted(bool),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct State {
    pub survey: Survey,
    pub datetime: String,
    pub completed: bool,
    pub result: f32,
    pub description: String,
    pub user_id: u32,
}

impl Survey {
    pub fn new(
        name: String,
        url: String,
        participant_count: i32,
        response_rate: f64,
        submitted_response_count: i32,
    ) -> Self {
        Survey {
            name: name,
            url: url,
            participant_count: participant_count,
            response_rate: response_rate,
            submitted_response_count: submitted_response_count,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }
    pub fn set_response_rate(&mut self, response_rate: f64) {
        self.response_rate = response_rate;
    }
    pub fn set_participant_count(&mut self, participant_count: i32) {
        self.participant_count = participant_count;
    }
    pub fn set_submitted_response_count(&mut self, submitted_response_count: i32) {
        self.submitted_response_count = submitted_response_count;
    }
}

impl State {
    fn completed(&mut self) {
        self.completed = true;
        self.datetime = chrono::offset::Utc::now().to_string();
        self.read_survey();
    }

    fn incomplete(&mut self) {
        self.completed = false;
    }
    fn read_survey(&self) {
        dbg!("{:?}\n", &self);
    }
    fn echo(&self, s: String) {
        dbg!("{}", s);
    }

    /// serde_json has a issue to url keyword-sensetive word, so be careful
    fn particle_setter_survey(&mut self, key: Option<&str>, value: Value) {
        match key {
            Some("name") => &self.survey.set_name(value.to_string()),
            Some("participant_count") => &self
                .survey
                .set_participant_count(value.to_string().parse::<i32>().unwrap_or(-1)),
            Some("response_rate") => &self
                .survey
                .set_response_rate(value.to_string().parse::<f64>().unwrap_or(0.0)),
            Some("iurl") | Some("url") | Some("rl") => &self.survey.set_url(value.to_string()),
            Some("submitted_response_count") => &self
                .survey
                .set_submitted_response_count(value.to_string().parse::<i32>().unwrap_or(-1)),
            Some("themes") => &{ self.result = self.read_themes(value) },
            Some(&_) => &self.echo("Other".to_string()),
            None => &self.process(Message::IsCompleted(true)),
        };
    }
    pub fn read_themes(&mut self, themes: Value) -> f32 {
        let mut vec_total: Vec<u32> = Vec::default();
        let mut sum: u32 = 0;
        let mut count: u32 = 0;
        let mut is_found_user_in_file = false;
        const ONE: u32 = 1;
        const ZERO: u32 = 0;
        for item_type_survey in 0..themes.as_array().unwrap().len() {
            let name = themes[item_type_survey].get("name").unwrap();
            let questions = themes[item_type_survey].get("questions").unwrap();

            let mut vec_sum: Vec<u32> = Vec::default();

            questions
                .as_array()
                .unwrap()
                .into_iter()
                .enumerate()
                .for_each(|(_i, item_type_question)| {
                    let survey_responses = item_type_question.get("survey_responses").unwrap();

                    survey_responses
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .enumerate()
                        .for_each(|(_j, item_type_responses)| {
                            if item_type_responses["respondent_id"] == self.user_id {
                                is_found_user_in_file = true;
                                let verifed_response_content = item_type_responses
                                    ["response_content"]
                                    .as_str()
                                    .unwrap_or("0")
                                    .parse::<u32>()
                                    .unwrap_or(0);
                                let _ = &vec_sum.push(verifed_response_content.clone());
                                if !verifed_response_content.eq(&ZERO) {
                                    count = count.add(ONE);
                                }
                            }
                        });
                });

            if !is_found_user_in_file {
                self.description = format!(
                    "Failed, not found user in a current survey:{}",
                    &self.survey.name
                );
                break;
            } else {
                self.description = format!(
                    "Successed, found user in a current survey:{}",
                    &self.survey.name
                );
            }
            sum = vec_sum.iter().sum::<u32>();

            debug!(
                "Sum rates per question-type of:{} for user({}) is {:?}",
                name, &self.user_id, &sum
            );
            let _ = &vec_total.push(sum);
        }
        if is_found_user_in_file {
            let total = &vec_total.iter().sum::<u32>();
            debug!("For user({}) the total number is {}", &self.user_id, &total);
            debug!(
                "For user({}) the average number is {:.2}",
                &self.user_id,
                (*total as f32 / count as f32) as f32
            );
            return *total as f32 / count as f32;
        }
        return 0.0;
    }

    pub fn process(&mut self, message: Message) {
        match message {
            Message::IsCompleted(false) => self.incomplete(),
            Message::Update(index, key, value) => {
                let _ = index;
                self.particle_setter_survey(Some(key.as_str()), value);
            }
            Message::Echo(s) => self.echo(s),
            Message::IsCompleted(true) => self.completed(),
        }
    }
}
