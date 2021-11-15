#[derive(Clone, PartialEq)]
pub enum Section {
    NONE,
    DATA,
    TEXT,
}

pub fn resolve_section(code: &str) -> Option<Section> {
    match code {
        "\t.data" => Some(Section::DATA),
        "\t.text" => Some(Section::TEXT),
        _ => None,
    }
}
