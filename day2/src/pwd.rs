use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap()
});

pub struct Rules {
    pub min: u8,
    pub max: u8,
    pub letter: char,
}

pub fn from_string(data: &str) -> (Rules, String) {
    let captures = REGEX.captures(data).unwrap();
    (
        Rules {
            min: u8::from_str(captures.get(1).unwrap().as_str()).unwrap(),
            max: u8::from_str(captures.get(2).unwrap().as_str()).unwrap(),
            letter: captures.get(3).unwrap().as_str().chars().next().unwrap(),
        },
        String::from_str(captures.get(4).unwrap().as_str()).unwrap(),
    )
}
