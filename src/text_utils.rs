use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::io;
use strsim::jaro_winkler;

fn text_to_wordlist(text: &str) -> Vec<&str> {
    let mut final_worldlist: Vec<&str> = Vec::new();
    let wordlist: Vec<&str> = text.split_whitespace().collect();

    for element in wordlist.iter() {
        if element.len() >= 2 {
            final_worldlist.push(element)
        }
    }

    final_worldlist
}

pub fn get_regexes(file_path: &str) -> Result<Value, io::Error> {
    let definitions = fs::read_to_string(file_path)?;
    let rules: serde_json::Value = serde_json::from_slice(definitions.as_bytes())?;

    Ok(rules)
}

fn clean_word(text: &str) -> String {
    let mut output = String::from(text);

    output.retain(|c| c != '.');
    output.retain(|c| c != '\'');
    output.retain(|c| c != '-');
    output.retain(|c| c != '_');
    output.retain(|c| c != ',');
    output.to_lowercase()
}

fn similarity(a: &str, b: &str) -> f64 {
    jaro_winkler(a, b) * 100.0
}

pub fn detect_emails(rules: Value, text: &str) -> Result<HashSet<String>, io::Error> {
    let email_rules = rules["Email"]["regex"].as_str();
    let email_regex = Regex::new(email_rules.expect("Email regex not found!")).unwrap();
    let email_list: HashSet<String> = email_regex
        .captures_iter(text)
        .filter_map(|cap| cap.get(0).map(|m| m.as_str().to_string()))
        .collect();

    Ok(email_list)
}

pub fn detect_phone_numbers(rules: Value, text: &str) -> Result<HashSet<String>, io::Error> {
    let phone_rules = rules["Phone Number"]["regex"].as_str();
    let phone_regex = Regex::new(phone_rules.expect("Email regex not found!")).unwrap();
    let phone_list: HashSet<String> = phone_regex
        .captures_iter(text)
        .filter_map(|cap| cap.get(0).map(|m| m.as_str().to_string()))
        .collect();

    Ok(phone_list)
}

pub fn detect_keywords(rules: Value, text: &str) -> Result<HashSet<String>, io::Error> {
    let wordlist = text_to_wordlist(text);

    let mut results: HashSet<String> = HashSet::new();

    if let Some(map) = rules.as_object() {
        for (key, rule) in map {
            let keywords: Value = rule["keywords"].clone();

            if let Some(kws) = keywords.as_array() {
                for word in &wordlist {
                    for kw in kws {
                        let sim = similarity(&clean_word(word), &clean_word(&kw.to_string()));
                        if sim > 80.0 {
                            results.insert(key.to_string());
                        }
                    }
                }
            }
        }
    }

    Ok(results)
}
