use std::{collections::HashMap, path::Path};

use crate::read_to_string_sjis;

#[derive(Clone, Debug)]
pub struct Character {
    attributes: HashMap<String, String>,
}

impl Character {
    pub fn load(path: impl AsRef<Path>) -> Option<Self> {
        let mut attributes = HashMap::new();

        let contents = read_to_string_sjis(path)?;
        for line in contents.lines() {
            if line.is_empty() || line.chars().all(char::is_whitespace) {
                continue;
            }

            let (key, value) = line
                .split_once("=")
                .or_else(|| line.split_once(":"))
                .or_else(|| line.split_once("："))?;

            let key = match key {
                "名前" => "name",
                "画像" => "image",
                key => key,
            };

            let key = key.trim().to_string();
            let value = value.trim().to_string();
            attributes.insert(key, value);
        }

        Some(Self { attributes })
    }

    pub fn name(&self) -> Option<&str> {
        self.get_attribute("name")
    }

    pub fn image(&self) -> Option<&str> {
        self.get_attribute("image")
    }

    pub fn sample(&self) -> Option<&str> {
        self.get_attribute("sample")
    }

    pub fn author(&self) -> Option<&str> {
        self.get_attribute("author")
    }

    pub fn web(&self) -> Option<&str> {
        self.get_attribute("web")
    }

    pub fn get_attribute(&self, key: &str) -> Option<&str> {
        let value = self.attributes.get(key)?;
        Some(value.as_str())
    }
}
