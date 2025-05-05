const BASE: i32 = 36;

/*
    # Rust Bytes Challenge - Issue #65 From Rust Bytes
    # Credit: Silen1t
    # Github: https://github.com/Silen1t
*/

fn main() {
    // Examples

    println!("Output: {:?}", convert_to(46656, BASE));
    // Expected Output: "1000"
    
    println!("Output: {:?}", convert_to(2147483646, BASE));
    // Expected Output: zik0zi
    
    println!("Output: {:?}", convert_to(2147483647, BASE));
    // Expected Output: zik0zj

    println!("Output: {:?}", convert_to(1295, BASE));
    // Expected Output: zz
}

fn convert_to(mut num: i32, base:i32) -> String {
    let mut result = String::new();
    let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    while num > 0 {
        let char = match chars.chars().nth((num as usize) % (base as usize)) {
            Some(char) => char,
            None => {
                return String::new();
            }
        };

        result.insert(0, char);

        num /= base;
    }

    result.to_lowercase()
}
