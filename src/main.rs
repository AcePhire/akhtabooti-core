mod text_utils;
mod file_utils;

use std::env;
use std::collections::HashSet;
use walkdir::WalkDir;
use std::io;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct FilePIIs {
    email_accounts: HashSet<String>,
    phone_numbers: HashSet<String>,
    other_piis: Vec<String>
}

fn main() {
    fn search_for_pii(text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let regex = text_utils::get_regexes("definitions.json");
        let rules = regex.unwrap();

        let pii = FilePIIs {
            email_accounts: text_utils::detect_emails(rules.clone(), text).unwrap(),
            phone_numbers: text_utils::detect_phone_numbers(rules.clone(), text).unwrap(),
            other_piis: text_utils::detect_keywords(rules.clone(), text).unwrap()
        };

        let json = serde_json::to_string(&pii)?;

        println!("{}", json);
        Ok(())
    }

    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let filename = entry.path().to_str().unwrap();
            let text = file_utils::extract_text(&filename);

            search_for_pii(&text);
        }
    }

    // for entry in WalkDir::new(path) {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //
    //     if path.is_file() {
    //         let filename = path.file_name().unwrap().to_string_lossy().into_owned();
    //         let text = file_utils::extract_text(&filename);
    //
    //         println!("{:?}", text);
    //
    //         let _piis = search_for_pii(&text);
    //     }
    // }
}
