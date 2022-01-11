use std::path::Path;

use crate::read_to_string_sjis;

#[derive(Clone, Debug)]
pub struct Oto {
    pub filename: String,
    pub alias: Option<String>,
    pub offset: f64,
    pub consonant_length: f64,
    pub blank_length: f64,
    pub precede_length: f64,
    pub overlap_length: f64,
}

impl Oto {
    pub fn load(path: impl AsRef<Path>) -> Option<Vec<Self>> {
        let mut oto_list = Vec::new();

        let contents = read_to_string_sjis(path)?;
        for line in contents.lines() {
            if line.is_empty() || line.chars().all(char::is_whitespace) {
                continue;
            }

            let (filename, params) = line.split_once("=")?;
            let filename = filename.trim().to_string();

            let mut params = params.split(",");
            let alias = match params.next().unwrap() {
                "" => None,
                alias => Some(alias.trim().to_string()),
            };
            let offset = params.next().unwrap().parse().unwrap();
            let consonant_length = params.next().unwrap().parse().unwrap();
            let blank_length = params.next().unwrap().parse().unwrap();
            let precede_length = params.next().unwrap().parse().unwrap();
            let overlap_length = params.next().unwrap().parse().unwrap();

            oto_list.push(Self {
                filename,
                alias,
                offset,
                consonant_length,
                blank_length,
                precede_length,
                overlap_length,
            });
        }

        Some(oto_list)
    }
}
