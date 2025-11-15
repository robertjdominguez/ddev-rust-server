use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

pub fn split_frontmatter_from_content(file_contents: String) -> Option<(String, String)> {
    let re = Regex::new(r"\A\s*---\s*([\s\S]*?)\s*---\s*([\s\S]*)\z").unwrap();

    re.captures(&file_contents).map(|caps| {
        let frontmatter = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let main_content = caps.get(2).map_or("", |m| m.as_str()).to_string();
        (frontmatter, main_content)
    })
}

pub fn parse_yaml_frontmatter(frontmatter: &str) -> Result<HashMap<String, serde_yaml::Value>, serde_yaml::Error> {
    serde_yaml::from_str(frontmatter)
}

pub fn extract_string_field(map: &HashMap<String, serde_yaml::Value>, key: &str) -> Option<String> {
    map.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
}

pub fn extract_bool_field(map: &HashMap<String, serde_yaml::Value>, key: &str) -> bool {
    map.get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub fn extract_string_array_field(map: &HashMap<String, serde_yaml::Value>, key: &str) -> Vec<String> {
    map.get(key)
        .and_then(|v| v.as_sequence())
        .map(|seq| {
            seq.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.trim().to_string())
                .collect()
        })
        .unwrap_or_default()
}