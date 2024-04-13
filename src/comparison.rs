use serde_json::Value;
use std::fs;

pub fn test() {
    // Read the contents of the JSON files
    let file1 = fs::read_to_string("./output/1.json").expect("Failed to read file 1.json");
    let file2 = fs::read_to_string("./output/2.json").expect("Failed to read file 2.json");

    // Parse the JSON contents into arrays
    let array1: Vec<Value> = serde_json::from_str(&file1).expect("Failed to parse file 1.json");
    let array2: Vec<Value> = serde_json::from_str(&file2).expect("Failed to parse file 2.json");

    // Compare the arrays and print differing indices
    for (i, (item1, item2)) in array1.iter().zip(array2.iter()).enumerate() {
        if item1 == item2 {
            println!("\x1b[32m {}\x1b[0m", i + 1); // Print in green color
        } else {
            let index = array2.iter().position(|x| x == item1);
            match index {
                Some(idx) => {
                    if idx > i {
                        println!("\x1b[31{}: {} ↓\x1b[0m", i + 1, idx + 1);
                    } else {
                        println!("{}:\x1b[31m {}\x1b[32m ↑\x1b[0m", i + 1, idx + 1);
                    }
                }
                None => println!("\x1b[31m{}: out\x1b[0m", i + 1),
            }
        }
    }
}
