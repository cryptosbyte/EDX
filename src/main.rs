use std::*;
use std::env;
use std::fs;
use cmd_lib::*;
use colored::*;
use serde_json;

fn main() {

    let edx_file = "./edx.json";

    if path::Path::new(edx_file).exists() {

        let data = fs::read_to_string(edx_file).expect("[EDX Error] Unable to read JSON");
        let res: serde_json::Value = serde_json::from_str(&data).expect("[EDX Error] Unable to parse JSON");

        let map = res["scripts"].as_object().expect("scripts isnt an object!");
        let arg = env::args().nth(1).expect("missing argument!");

        if map.contains_key(&arg) {
            
            println!("{} \t{} {}",
                "[EDX]".green(),
                "Running".clear(),
                res["scripts"][&arg]
            );

            let mut script_to_be_run: String = res["scripts"][&arg].to_string();
            script_to_be_run.remove(0);
            script_to_be_run.pop();

            if run_cmd! {
                ${script_to_be_run}
            }.is_err() {
                return println!("{} \t{}", 
                    "[EDX Error]".red(), 
                    "Error given when executing command".clear()
                );
            }

        } else {

            return println!("{} \t{}", 
                "[EDX Error]".red(),
                "Command not found in 'edx.json'".clear()
            );

        }

    } else {
        
        let current_dir = env::current_dir().unwrap();
        return println!("{} \t{} {:?}", 
            "[EDX Error]".red(), 
            "Cannot find 'edx.json' in".clear(), 
            current_dir
        );

    };

}