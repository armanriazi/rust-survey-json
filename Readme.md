
# Rust-Survey-Json

 ## Commands

> Optional Command
> 
> RUST_LOG=INFO | DEBUG | TRACE cargo run ...
>
```
cd rust-survey-json_bin
```

 ```
 cargo run  -p rust-survey-json_bin --bin rust-survey_main_bin 1 file /mnt/home/rust-all-in-one-projects/workspace/projects/survey/rust-survey/data/1.json /mnt/home/rust-all-in-one-projects/workspace/projects/survey/rust-survey/data/2.json
 ```

 ```
 cargo doc  --package rust-survey-json_bin --message-format short --no-deps --open --color always
 ```

 ```
 cargo test --doc  --package rust-survey-json_lib
 ```

 ## What
 
 A finder on json data for calculate the rate of users- there are 3 mode gain access to the json content"

 ## How
 `Refered to the docs of codes`



 # Return
 
 ```
 I'm running on a unix machine!
 `Making model(updated key successfuly):\`name\``
 `Making model(updated key successfuly):\`participant_count\``
 `Making model(updated key successfuly):\`response_rate\``
 `Making model(updated key successfuly):\`submitted_response_count\``
 `Making model(updated key successfuly):\`themes\``
 `Making model(updated key successfuly):\`url\``
 
 survey: Survey {
         name: `\`Simple Survey\``,
         url: `\`/survey_results/1\``,
         participant_count: 6,
         response_rate: 0.8333333333333334,
         submitted_response_count: 5,
     },
     datetime: `2022-12-29 18:49:27.644034577 UTC`,
     completed: true,
     result: 0.0,
     description: `Failed, not found user in a current survey:\`Simple Survey\``,
     user_id: 100,
 }
 
 survey: Survey {
         name: `\`Simple Survey\``,
         url: `\`/survey_results/1\``,
         participant_count: 6,
         response_rate: 0.8333333333333334,
         submitted_response_count: 5,
     },
     datetime: `2022-12-29 18:23:14.971810866 UTC`,
     completed: true,
     result: 3.8,
     description: `Successed, found user in a current survey:\`Simple Survey\``,
     user_id: 1,
 }
 
 ```
 
# Verify Expectation Function 

> Change response_content to empty string and then take a calculated result

```
{
    "survey_result_detail": {
        "name": "Simple Survey",
        "url": "/survey_results/1",
        "participant_count": 6,
        "response_rate": 0.8333333333333334,
        "submitted_response_count": 5,
        "themes": [
            {
                "name": "The Work",
                "questions": [
                    {
                        "description": "I like the kind of work I do.",
                        "question_type": "ratingquestion",
                        "survey_responses": [
                            {
                                "id": 1,
                                "question_id": 1,
                                "respondent_id": 1,
                                "response_content": ""
                            },
```

> Result:
>
```
    survey: Survey {
        name: "\"Simple Survey\"",
        url: "\"/survey_results/1\"",
        participant_count: 6,
        response_rate: 0.8333333333333334,
        submitted_response_count: 5,
    },
    datetime: "2022-12-31 05:01:37.137603399 UTC",
    completed: true,
    result: 4.5,
    description: "Successed, found user in a current survey:\"Simple Survey\"",
    user_id: 1,
}
```

> Tags #json #published #survey


```
console.log("🦀 Rust + 🕸 Wasm = ❤");
```

> Author: Arman Riazi
