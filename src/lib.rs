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

//detect piis within a certain text
//detection definitions are in 'definitions.json'
fn search_text_for_pii(filename: &str, text: &str) -> Result<FilePIIs, io::Error> {
    let defs = text_utils::load_definitions()?;
    let mut results = text_utils::detect(&defs, text);

    let email_accounts: HashSet<String> = results.remove("Email").unwrap_or_default();
    let phone_numbers: HashSet<String> = results.remove("Phone Number").unwrap_or_default();
    let other_piis: HashSet<String> = results.into_keys().collect();

    let pii = FilePIIs {
        filename: filename.to_string(),
        email_accounts,
        phone_numbers,
        other_piis,
    };

    Ok(pii)
}

//scan a directory (recursively) and return an list of objects of type FilePIIs
pub fn search_directory(path: &str) -> Result<Vec<FilePIIs>, io::Error> {
    let mut piis: Vec<FilePIIs> = Vec::new(); 

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filepath = &entry.path().to_string_lossy();
            let text = file_utils::extract_text(filepath)?;

            let file_pii = search_text_for_pii(filepath, &text)?;
            piis.push(file_pii);
        }
    }

    Ok(piis)
}

// scan a singular file and return an object of FilePIIs
pub fn search_file(path: &str) -> Result<FilePIIs, io::Error> {
    let filepath = Path::new(path);

    if filepath.is_file() {
        let text = file_utils::extract_text(&path)?;

        let file_pii = search_text_for_pii(&path, &text)?;
        return Ok(file_pii);
    }

    Ok(FilePIIs::default())
}
