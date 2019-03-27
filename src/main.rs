use crate::options::parse_options;
use colored::Colorize;
use indexmap::IndexMap;
use regex::Captures;
use regex::Regex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

mod options;

fn format_match(regex: &Regex, color_output: bool, path: &str, extra: &str) {
    if !regex.is_match(&path) {
        return;
    }

    if color_output {
        let output = regex.replace_all(&path, |caps: &Captures| {
            format!("{}", &caps[0].red().bold())
        });
        println!("{}{}", output, extra);
    } else {
        println!("{}{}", path, extra);
    }
}

fn visit_obj(regex: &Regex, color_output: bool, path: &str, value: &Value) {
    let obj = value.as_object().unwrap();

    if obj.is_empty() {
        format_match(regex, color_output, &path, " = {{}}");
        return;
    }

    for e in obj {
        let fullstring = format!("{}/{}", path, e.0);
        if e.1.is_array() {
            visit_array(regex, color_output, &fullstring, e.1);
        } else if e.1.is_object() {
            visit_obj(regex, color_output, &fullstring, e.1);
        } else {
            format_match(regex, color_output, &fullstring, &format!(" = {}", e.1));
        }
    }
}

fn visit_array(regex: &Regex, color_output: bool, path: &str, value: &Value) {
    let a = value.as_array().unwrap();

    if a.is_empty() {
        format_match(regex, color_output, &path, " = []");
        return;
    }

    for e in a {
        if e.is_array() {
            visit_array(regex, color_output, path, e);
        } else if e.is_object() {
            visit_obj(regex, color_output, path, e);
        } else {
            format_match(regex, color_output, &path, &format!(" = {}", e));
        }
    }
}

fn search_file(regex: &str, f: &str, color_output: bool) -> std::io::Result<()> {
    let file = File::open(f)?;
    let reader = BufReader::new(file);

    let values: IndexMap<String, Value> = serde_json::from_reader(reader)?;
    let regex = Regex::new(&regex).unwrap();

    for v in values {
        if v.1.is_array() {
            visit_array(&regex, color_output, &v.0, &v.1);
        } else if v.1.is_object() {
            visit_obj(&regex, color_output, &v.0, &v.1);
        } else {
            format_match(&regex, color_output, &v.0, &format!(" = {}", v.1));
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let (regex, files, color_output) = parse_options();

    for f in files {
        search_file(&regex, &f, color_output)?;
    }

    Ok(())
}
