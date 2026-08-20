pub mod text_utils;
pub mod file_utils;

use std::collections::HashSet;
use walkdir::WalkDir;
use std::io;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FilePIIs {
    pub filename: String,
    pub email_accounts: HashSet<String>,
    pub phone_numbers: HashSet<String>,
    pub other_piis: HashSet<String>
}

fn search_text_for_pii(filename: &str, text: &str) -> Result<FilePIIs, io::Error> {
    let regex = text_utils::get_regexes("definitions.json");
    let rules = regex.unwrap();

    let pii = FilePIIs {
        filename: filename.to_string(),
        email_accounts: text_utils::detect_emails(rules.clone(), text).unwrap(),
        phone_numbers: text_utils::detect_phone_numbers(rules.clone(), text).unwrap(),
        other_piis: text_utils::detect_keywords(rules.clone(), text).unwrap()
    };

    Ok(pii)
}

pub fn search_directory(path: &str) -> Result<Vec<FilePIIs>, io::Error> {
    let mut piis: Vec<FilePIIs> = Vec::new(); 

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let filepath = entry.path().to_str().unwrap();
            let text = file_utils::extract_text(&filepath);

            let file_pii = search_text_for_pii(&filepath, &text).unwrap();
            piis.push(file_pii);
        }
    }

    Ok(piis)
}

pub fn search_file(path: &str) -> Result<FilePIIs, io::Error> {
    let filepath = Path::new(path);

    if filepath.is_file() {
        let text = file_utils::extract_text(&path);

        let file_pii = search_text_for_pii(&path, &text).unwrap();
        return Ok(file_pii);
    }

    Ok(FilePIIs::default())
}
