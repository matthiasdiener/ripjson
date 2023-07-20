use crate::options::parse_options;
use colored::Colorize;
use indexmap::IndexMap;
use regex::Captures;
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::io::Read;

mod options;

fn format_match(regex: &Regex, color_output: bool, path: &str, extra: &str, res: &mut String) {
    if !regex.is_match(&path) {
        return;
    }

    if color_output {
        let output = regex.replace_all(&path, |caps: &Captures| {
            format!("{}", &caps[0].red().bold())
        });
        res.push_str(&output);
        res.push_str(extra);
        res.push('\n');
    } else {
        res.push_str(path);
        res.push_str(extra);
        res.push('\n');
    }
}

fn visit_obj(regex: &Regex, color_output: bool, path: &str, value: &Value, res: &mut String) {
    let obj = value.as_object().unwrap();

    if obj.is_empty() {
        format_match(regex, color_output, &path, " = {{}}", res);
        return;
    }

    for e in obj {
        let fullstring = format!("{}/{}", path, e.0);
        if e.1.is_array() {
            visit_array(regex, color_output, &fullstring, e.1, res);
        } else if e.1.is_object() {
            visit_obj(regex, color_output, &fullstring, e.1, res);
        } else {
            format_match(
                regex,
                color_output,
                &fullstring,
                &format!(" = {}", e.1),
                res,
            );
        }
    }
}

fn visit_array(regex: &Regex, color_output: bool, path: &str, value: &Value, res: &mut String) {
    let a = value.as_array().unwrap();

    if a.is_empty() {
        format_match(regex, color_output, &path, " = []", res);
        return;
    }

    for e in a {
        if e.is_array() {
            visit_array(regex, color_output, path, e, res);
        } else if e.is_object() {
            visit_obj(regex, color_output, path, e, res);
        } else {
            format_match(regex, color_output, &path, &format!(" = {}", e), res);
        }
    }
}

fn search_string(regex: &str, content: &str, color_output: bool) -> String {
    let values: IndexMap<String, Value> = serde_json::from_str(content).unwrap();
    let regex = Regex::new(&regex).unwrap();

    let mut res = "".to_string();

    for v in values {
        if v.1.is_array() {
            visit_array(&regex, color_output, &v.0, &v.1, &mut res);
        } else if v.1.is_object() {
            visit_obj(&regex, color_output, &v.0, &v.1, &mut res);
        } else {
            format_match(&regex, color_output, &v.0, &format!(" = {}", v.1), &mut res);
        }
    }

    res
}

fn search_file(regex: &str, filename: &str, color_output: bool) -> std::io::Result<()> {
    let mut contents = String::new();

    if filename == "-" {
        std::io::stdin().read_to_string(&mut contents)?;
    } else {
        contents = fs::read_to_string(filename).unwrap();
    }

    print!("{}", search_string(regex, &contents, color_output));
    Ok(())
}

fn main() -> std::io::Result<()> {
    let (regex, files, color_output) = parse_options();

    for filename in files {
        search_file(&regex, &filename, color_output)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_simple_string() {
        use crate::search_string;
        let mystr = "{\"name\": \"John Doe\",\"age\": 43,\"address\":
        {\"street\": \"10 Downing Street\",\"city\": \"London\"
        },\"phones\": [\"+44 1234567\",\"+44 2345678\"]}";
        let regex = ".*es.*/cit";
        assert_eq!(
            &search_string(regex, mystr, false),
            "address/city = \"London\"\n"
        );
    }
}
