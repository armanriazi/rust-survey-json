#![allow(dead_code, unused_variables, unused_imports)]
use rust_survey_json_lib::core::factory::json_factory;
use rust_survey_json_lib::{core::error::CustomError, core::sample::*};
use env_logger::{Builder, Target};
use log::{debug, error, info, log_enabled, Level};
use std::io::BufReader;
use std::process;
use std::{env, fs, path::Path};
use std::{env::args, fs::File};

/// Main
///
/// ## Commands
///
///
/// ```RUST_LOG=INFO cargo run  -p rust-survey-json_bin --bin rust-survey_main_bin 1 file /mnt/home/rust-all-in-one-projects/workspace/projects/survey/rust-survey/data/1.json /mnt/home/rust-all-in-one-projects/workspace/projects/survey/rust-survey/data/2.json```
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
/// ```no_run,compile_file
/// I'm running on a unix machine!
/// `Making model(updated key successfuly):\`name\``
/// `Making model(updated key successfuly):\`participant_count\``
/// `Making model(updated key successfuly):\`response_rate\``
/// `Making model(updated key successfuly):\`submitted_response_count\``
/// `Making model(updated key successfuly):\`themes\``
/// `Making model(updated key successfuly):\`url\``
/// 
/// survey: Survey {
///         name: `\`Simple Survey\``,
///         url: `\`/survey_results/1\``,
///         participant_count: 6,
///         response_rate: 0.8333333333333334,
///         submitted_response_count: 5,
///     },
///     datetime: `2022-12-29 18:49:27.644034577 UTC`,
///     completed: true,
///     result: 0.0,
///     description: `Failed, not found user in a current survey:\`Simple Survey\``,
///     user_id: 100,
/// }
/// 
/// survey: Survey {
///         name: `\`Simple Survey\``,
///         url: `\`/survey_results/1\``,
///         participant_count: 6,
///         response_rate: 0.8333333333333334,
///         submitted_response_count: 5,
///     },
///     datetime: `2022-12-29 18:23:14.971810866 UTC`,
///     completed: true,
///     result: 4.6,
///     description: `Successed, found user in a current survey:\`Simple Survey\``,
///     user_id: 1,
/// }
/// 
/// ```
/// 

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn main() -> Result<(), CustomError> {
    init_app();

    survey_init_env_logger(true);

    info!("Starting Up...");

    let mut args: Vec<String> = env::args().collect();

    let mut mode = String::default();
    let mut user_id: u32 = 0;
    let mut file_name = String::default();
     
    if (&args).len() <= 2 {
        info!("** Please select a runner mode\n Help(file path transaction_list, or macrojson transaction_list)\n Default is cargo run macrojson **\n");
        args.push("macrojson".to_owned());
    } else {        
        mode = (&args[2]).trim().to_lowercase();
    }
    user_id = (&args[1]).trim().parse::<u32>().unwrap_or(0);
        if &user_id <= &0 {
            dbg!("Select user id > 0");
            process::exit(1);
    }
    info!("UserId: {:?}\n", user_id.clone());
    info!("Mode: {:?}\n", mode.clone());

    if &mode == "file" {
        for (i, arg) in args.iter().enumerate() {
            if arg.find("json").is_some() {
                //file_name = arg.to_string();
                let json = fs::read_to_string(arg).unwrap();

                let file: serde_json::Value = serde_json::from_str(&json)?;

                json_factory(|| sample_json_data_from_file(file), user_id)?;
            }
        }
    } else if &mode == "macrojson" {
        info!("Selected mode is macrojson\n");
        json_factory(|| sample_json_data_from_module(), user_id)?;
    } else if &mode == "stringjson" {
        info!("Selected mode is stringjson\n");
        json_factory(|| sample_json_data_from_string(), user_id)?;
    } else {
        info!("The mode is not selected! Default is macrojson\n");
        json_factory(|| sample_json_data_from_module(), user_id)?;
    }
    Ok(())
}

pub fn survey_init_env_logger(is_enable: bool) {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    if is_enable {
        builder.init();
    }

    if log_enabled!(Level::Info) {
        info!("------------Welcome to env_logger------------");
    } else {
        info!("-------env_logger have not been activated-------");
    }
}

/// # Another way to read from file
/// ```rust,no_run
///let reader = BufReader::new(file);
///let serde_values = serde_json::from_reader(reader)?;
/// ```
pub fn sample_json_data_from_file(
    file: serde_json::Value,
) -> Result<serde_json::Value, CustomError> {
    info!("Selected mode is file!");

    return Ok(file);
}

fn init_app() -> impl std::process::Termination {
    let machine_kind = if cfg!(unix) {
        println!("I'm running on a {} machine!", "unix");
        std::process::ExitCode::SUCCESS
    } else if cfg!(windows) {
        println!("I'm running on a {} machine!", "windows");
        std::process::ExitCode::SUCCESS
    } else {
        println!("I'm running on a {} machine!", "unknown");
        std::process::ExitCode::FAILURE
    };
}
