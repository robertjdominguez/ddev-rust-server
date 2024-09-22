use pulldown_cmark;
use regex::Regex;
use serde::Deserialize;
use std::fs;

/**
* Simply enough, this function finds the file by name in the /posts directory and then
* uses cmark to transform it into html.
*/
pub fn transform_markdown_to_html(filename: String) -> String {
    let path_to_file = format!("posts/{}.md", filename);
    let template = fs::read_to_string("templates/post.html");
    let contents = fs::read_to_string(&path_to_file);

    // Handle error if template or content cannot be read
    let template = match template {
        Ok(template) => template,
        Err(error) => return format!("Error getting the post template: {}", error),
    };

    let contents = match contents {
        Ok(contents) => contents,
        Err(error) => return format!("Error reading the file {}: {}", &path_to_file, error),
    };

    let split_contents = split_frontmatter_from_content(contents);
    match split_contents {
        Some((frontmatter_str, main_content)) => {
            let parser = pulldown_cmark::Parser::new(&main_content);

            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);

            // Parse frontmatter into struct
            let frontmatter = match parse_frontmatter(&frontmatter_str) {
                Some(fm) => fm,
                None => return "Error parsing frontmatter.".to_string(),
            };

            // Replace placeholders in the template
            let combined_html = template
                .replace("{page_title}", &frontmatter.title)
                .replace("{og_title}", &frontmatter.title)
                .replace("{hook}", &frontmatter.hook)
                .replace("{og_description}", &frontmatter.hook)
                .replace("{og_image}", &frontmatter.image)
                .replace("{slug}", &frontmatter.slug)
                .replace("{post_image}", &frontmatter.image)
                .replace("{post_content}", &html_output);

            combined_html.to_string()
        }
        None => "No frontmatter found.".to_string(),
    }
}

/**
* Struct to hold frontmatter information.
*/
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    hook: String,
    slug: String,
    created_at: String,
    image: String,
}

/**
* Parse frontmatter YAML into a Frontmatter struct.
*/
fn parse_frontmatter(frontmatter: &str) -> Option<Frontmatter> {
    serde_yaml::from_str(frontmatter).ok()
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
