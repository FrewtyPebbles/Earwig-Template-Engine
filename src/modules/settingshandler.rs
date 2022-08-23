
//usages
use std::collections::HashMap;

pub fn parse_settings_arg(raw_settings:String) -> HashMap<String, String> {
    let mut ret_val:HashMap<String, String> = HashMap::new();
    let mut key_buffer:String = String::new();
    let mut value_buffer:String = String::new();
    let mut is_key:bool = true;
    for raw_char in raw_settings.chars() {
        match raw_char {
            ':' => {
                is_key = false;
            },
            ',' => {
                ret_val.insert(key_buffer.clone(), value_buffer.clone());
                is_key = true;
            },
            _ => {
                if is_key {
                    key_buffer.push(raw_char.clone());
                }
                else {
                    value_buffer.push(raw_char.clone());
                }
            }
        }
    }
    ret_val
}