use pulldown_cmark;
use regex::Regex;
use std::fs;

/**
* Simply enough, this function finds the file by name in the /posts directory and then
* uses cmark to transform it into html.
*/
pub fn transform_markdown_to_html(filename: String) -> String {
    let path_to_file = format!("posts/{}.md", filename);

    let contents = fs::read_to_string(&path_to_file);
    let contents = match contents {
        Ok(contents) => contents,

        Err(error) => return format!("Error reading the file {}: {}", &path_to_file, error),
    };

    let split_contents = split_frontmatter_from_content(contents);
    match split_contents {
        Some((_frontmatter, main_content)) => {
            let parser = pulldown_cmark::Parser::new(&main_content);

            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);

            html_output
        }
        None => r#"No frontmatter found."#.to_string(),
    }
}

/**
* We'll also create a function that strips away frontmatter. This will be useful for our
* /posts route so that we can use that frontmatter to create that content!
*/
pub fn split_frontmatter_from_content(file_contents: String) -> Option<(String, String)> {
    let re = Regex::new(r"\A\s*---\s*([\s\S]*?)\s*---\s*([\s\S]*)\z").unwrap();

    re.captures(&file_contents).map(|caps| {
        let frontmatter = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let main_content = caps.get(2).map_or("", |m| m.as_str()).to_string();
        (frontmatter, main_content)
    })
}