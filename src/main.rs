use serde_json;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.chars().next().unwrap().is_digit(10) {
        return decode_string(&encoded_value);
    } else if encoded_value.starts_with("i") && encoded_value.ends_with("e") {
        return decode_integer(&encoded_value);
    } else if encoded_value.starts_with("l") && encoded_value.ends_with("e") {
        return decode_list(&encoded_value);
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_string(encoded_string: &str) -> serde_json::Value {
    let colon_index = encoded_string.find(':').unwrap();

    let number_string = &encoded_string[..colon_index];
    let string_length = number_string.parse::<usize>().unwrap();

    let start_index = colon_index + 1;
    let string = &encoded_string[start_index..start_index + string_length];

    return serde_json::Value::String(string.to_string());
}

// Input should include starting 'i' and ending 'e'
fn decode_integer(encoded_integer: &str) -> serde_json::Value {
    let number_string = &encoded_integer[1..encoded_integer.len() - 1];

    return serde_json::Value::Number(number_string.parse::<i64>().unwrap().into());
}

// Input should include starting 'l' and ending 'e'
fn decode_list(encoded_list: &str) -> serde_json::Value {
    let mut result = Vec::new();

    let mut current_encoded_value = String::new();

    let mut encoded_string_length: isize = -1;

    let mut start_end_char_difference = 0;

    for i in 1..encoded_list.len() - 1 {
        let current_char = &encoded_list.chars().nth(i).unwrap();

        //println!("Current encoded value is {}", current_encoded_value);
        //println!("Current char is {}", current_char);

        current_encoded_value.push(*current_char);

        if current_encoded_value.chars().nth(0).unwrap().is_numeric() {
            if encoded_string_length == 1 {
                result.push(decode_string(&current_encoded_value));
                current_encoded_value = String::new();
                encoded_string_length = -1;
            } else if current_char == &':' {
                let length_string = &current_encoded_value[..current_encoded_value.len() - 1];
                encoded_string_length = length_string.parse::<isize>().unwrap();
            } else if encoded_string_length != -1 {
                encoded_string_length -= 1;
            }
        } else if current_encoded_value.chars().nth(0).unwrap() == 'i' && current_char == &'e' {
            result.push(decode_integer(&current_encoded_value));
            current_encoded_value = String::new();
        } else if current_encoded_value.chars().nth(0).unwrap() == 'l' {
            if current_char == &'i' || current_char == &'l' {
                start_end_char_difference += 1;
            } else if current_char == &'e' {
                start_end_char_difference -= 1;
            }

            if start_end_char_difference == 0 {
                result.push(decode_list(&current_encoded_value));
                current_encoded_value = String::new();
            }
        }
    }

    return serde_json::Value::Array(result);
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
