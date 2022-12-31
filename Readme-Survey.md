
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
     result: 4.6,
     description: `Successed, found user in a current survey:\`Simple Survey\``,
     user_id: 1,
 }
 
 ```
 


> Tags #json #published #survey


```
console.log("🦀 Rust + 🕸 Wasm = ❤");
```

> Author: Arman Riazi
