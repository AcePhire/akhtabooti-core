use regex::Regex;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use strsim::jaro_winkler;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum RawCondition {
    Regex(String),
    Keywords(Vec<String>),
}

#[derive(Deserialize)]
struct RawDefinition {
    region: Option<String>,
    rules: Vec<Vec<RawCondition>>,
}

pub enum Condition {
    Regex(Regex),
    Keywords(Vec<Vec<String>>),
}

pub struct Definition {
    pub region: Option<String>,
    pub rules: Vec<Vec<Condition>>,
}

pub type Definitions = HashMap<String, Definition>;

pub fn load_definitions(file_path: &str) -> io::Result<Definitions> {
    let raw: HashMap<String, RawDefinition> = serde_json::from_str(&fs::read_to_string(file_path)?)?;

    raw.into_iter()
        .map(|(name, def)| {
            let rules = def
                .rules
                .into_iter()
                .map(|branch| branch.into_iter().map(compile).collect())
                .collect::<io::Result<_>>()?;
            Ok((name, Definition { region: def.region, rules }))
        })
        .collect()
}

fn compile(cond: RawCondition) -> io::Result<Condition> {
    Ok(match cond {
        RawCondition::Regex(pattern) => Condition::Regex(
            Regex::new(&pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?,
        ),
        RawCondition::Keywords(kws) => Condition::Keywords(
            kws.iter()
                .map(|kw| kw.split_whitespace().map(clean_word).collect())
                .filter(|kw: &Vec<String>| !kw.is_empty())
                .collect(),
        ),
    })
}

fn clean_word(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '.' | '\'' | '-' | '_' | ','))
        .collect::<String>()
        .to_lowercase()
}

fn tokenize(text: &str) -> Vec<(&str, String)> {
    text.split_whitespace()
        .map(|w| (w, clean_word(w)))
        .filter(|(_, c)| c.chars().count() >= 2)
        .collect()
}

fn similarity(a: &str, b: &str) -> f64 {
    jaro_winkler(a, b) * 100.0
}

fn find_keywords(kws: &[Vec<String>], words: &[(&str, String)]) -> HashSet<String> {
    let mut found = HashSet::new();

    for kw in kws {
        let target = kw.join(" ");
        for window in words.windows(kw.len()) {
            let candidate: Vec<&str> = window.iter().map(|(_, c)| c.as_str()).collect();
            if similarity(&candidate.join(" "), &target) > 80.0 {
                let original: Vec<&str> = window.iter().map(|(o, _)| *o).collect();
                found.insert(original.join(" "));
            }
        }
    }

    found
}

fn evaluate(cond: &Condition, text: &str, words: &[(&str, String)]) -> HashSet<String> {
    match cond {
        Condition::Regex(re) => re.find_iter(text).map(|m| m.as_str().to_string()).collect(),
        Condition::Keywords(kws) => find_keywords(kws, words),
    }
}

pub fn detect(defs: &Definitions, text: &str) -> HashMap<String, HashSet<String>> {
    let words = tokenize(text);
    let mut results = HashMap::new();

    for (name, def) in defs {
        let mut evidence = HashSet::new();

        for branch in &def.rules {
            let mut hits = HashSet::new();
            let passed = branch.iter().all(|cond| {
                let found = evaluate(cond, text, &words);
                let ok = !found.is_empty();
                hits.extend(found);
                ok
            });
            if passed {
                evidence.extend(hits);
            }
        }

        if !evidence.is_empty() {
            results.insert(name.clone(), evidence);
        }
    }

    results
}
