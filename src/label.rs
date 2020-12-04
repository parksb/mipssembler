use regex::Regex;

#[derive(Debug)]
pub struct Label {
    name: String,
    address: i32,
    new_address: i32,
}

impl Label {
    pub fn new(name: &str, address: i32, new_address: i32) -> Self {
        Self {
            name: name.to_string(),
            address,
            new_address,
        }
    }

    pub fn compare_name(&self, name: &str) -> bool {
        self.name == name.to_string()
    }

    pub fn get_address(&self) -> i32 {
        self.address
    }
}

pub fn find_label<'a>(name: &'a str, labels: &'a Vec<Label>) -> Option<&'a Label> {
    labels.iter().find(|label| label.compare_name(name))
}

pub fn is_label(code: &str) -> bool {
    let label_regex = Regex::new(r"^.*:").unwrap();
    label_regex.is_match(code)
}

pub fn resolve_labels(code: &str) -> Option<Label> {
    let label_regex = Regex::new(r"^.*:").unwrap();
    if let Some(cap) = label_regex.captures_iter(&code).next() {
        let name = cap[0].trim_end_matches(':');
        Some(Label::new(name, 0, 0))
    } else {
        None
    }
}
