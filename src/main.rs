use serde_json;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    if encoded_value.chars().next().unwrap().is_digit(10) {
        // Example: "5:hello" -> "5"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    // If encoded_value starts with 'i' and ends with 'e', then it is an integer
    } else if encoded_value.starts_with("i") && encoded_value.ends_with("e") {
        let number_string = &encoded_value[1..encoded_value.len() - 1];
        return serde_json::Value::Number(number_string.parse::<i64>().unwrap().into());
    } else if encoded_value.starts_with("l") && encoded_value.ends_with("e") {
        let array_string = "";
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

fn decode_string(encoded_string: &str) -> serde_json::Value::String {
    let colon_index = encoded_string.find(":").unwrap();

    let number_string = &encoded_string[..colon_index];
    let string_length = number_string.parse::<i64>().unwrap();

    let start_index = colon_index + 1;
    let string = &encoded_string[start_index..start_index + string_length];

    return serde_json::Value::String(string.to_string())
}

// Input should include starting 'i' and ending 'e'
fn decode_integer(encoded_integer: &str) -> serde_json::Value::Number {
    let number_string = &encoded_integer[1..encoded_integer.len() - 1];
    
    return serde_json::Value::Number(number_string.parse::<i64>().unwrap().into());
}

// Input should include starting 'l' and ending 'e'
fn decode_list(encoded_list: &str) -> serde_json::Value::Array {
    let result = Vec::new();

    let current_encoded_value: &str = ""; 
    let current_encoded_value_type: u8 = 0; // 0 for string, 1 for int, 2 for list

    let encoded_string_length: usize = 0;

    for i in 1..encoded_list.len() - 1 {
        let current_char = &encoded_list[i];

        if &current_char != "" {
            if current_encoded_value_type == 0 {
                if encoded_string_length == 0 {
                    let decoded = decode_string(&current_encoded_value);
                    result.push(decoded);
                    current_encoded_value = "";
                } else if current_char == ":" {
                    let length_string = &current_encoded_value;
                    encoded_string_length = length_string.parse::<i64>().unwrap();
                    current_encoded_value += &current_char;
                } else {
                    encoded_string_length -= 1;
                }
            } else if current_char == "e" {
                current_encoded_value += &current_char;
                
                let decoded;

                if current_encoded_value_type == 1 {
                    decoded = decode_integer(&current_encoded_value);
                } else if current_encoded_value_type == 2 {
                    decoded = decode_list(&current_encoded_value);
                }

                result.push(decoded);
                
                current_encoded_value = "";
            }
        } else if &current_char.trim().parse::<i64>().is_ok() {
            current_encoded_value += &current_char;
            current_encoded_value_type = 0;
            encoded_string_length += 1;
        } else if &current_char == "i" {
            current_encoded_value += &current_char;
            current_encoded_value_type = 1;
        } else if &current_char == "l" {
            current_encoded_value += &current_char;
            current_encoded_value_type = 2;
        }
    }

    //  Loop over each char in encoded value, apart from first and last
    //  if current_encoded_value is not empty
    //      if type = 0
    //          if encoded string length = 0
    //              add char to encoded value
    //              decode
    //              set current encoded value to empty string 
    //          else if current char is :
    //              get string in currently encoded value as number (this is the length)
    //              set encoded string length
    //              add colon to encoded value
    //          else decrement string length
    //      else current char is e
    //          add char to encoded_value
    //          decode value depending on type
    //          set current_encoded_value to empty string
    //  else if current_value is a number
    //      add number to encoded value and set type to 0
    //      increment encoded string length
    //  else if current value is i
    //      add char to encoded value and set type to 1
    //  else current value is l
    //      add char to encoded value and set type to 2
    
    return serde_json:Value::Array(result)
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
