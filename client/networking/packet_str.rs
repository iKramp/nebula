use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum ValueType {
    STRING(String),
    LIST(HashMap<String, ValueType>),
}

impl PartialEq for ValueType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ValueType::STRING(str1) => {
                if let ValueType::STRING(str2) = other {
                    str2 == str1
                } else {
                    false
                }
            }
            ValueType::LIST(map1) => {
                if let ValueType::LIST(map2) = other {
                    map1 == map2
                } else {
                    false
                }
            }
        }
    }
}

pub fn from_packet(data: Vec<u8>) -> Result<ValueType> {
    let data = String::from_utf8(data)?;
    let data: Vec<&str> = data.split(' ').collect();
    let (_, map) = parse_list(&data.get(1..).unwrap())?;

    Ok(ValueType::LIST(map))
}

fn parse_list(data: &[&str]) -> Result<(usize, HashMap<String, ValueType>)> {
    let mut map = HashMap::new();
    let mut index = 0;
    while index < data.len() {
        let key = *data.get(index).unwrap();
        if key == "]" {
            return Ok((index, map));
        }
        index += 1;
        let value = *data.get(index).unwrap();
        if value == "[" {
            let (i, val) = parse_list(data.get(index + 1..).unwrap())?;
            map.insert(key.to_owned(), ValueType::LIST(val));
            index += 1 + i;
        } else {
            map.insert(key.to_owned(), ValueType::STRING(value.to_owned()));
        }
        index += 1;
    }

    Ok((index, map))
}

pub fn to_packet(data: ValueType) -> Vec<u8> {
    if let ValueType::LIST(map) = data {
        let string = write_list(map);
        string.into_bytes()
    } else {
        panic!("Root ValueType must be a LIST variant")
    }
}

fn write_list(map: HashMap<String, ValueType>) -> String {
    let mut string = "[".to_owned();
    for element in map {
        string.push(' ');
        string.push_str(&element.0);
        string.push(' ');
        match element.1 {
            ValueType::STRING(value) => {
                string.push_str(&value);
            }
            ValueType::LIST(map) => string.push_str(&write_list(map)),
        }
    }
    string.push_str(" ]");
    string
}
