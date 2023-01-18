// This mod calls ./derivation_tree.py and passes on a string as an argument
// to the python script. The python script should print something on the terminal and returns nothing

// The python script should be in the same directory as this file
// The python script should be named derivation_tree.py

use std::process::Command;

pub fn validate_expression(expression: &str) -> bool {
    // From the expression, remove the semicolon at the end and add spaces between tokens if needed
    let expression = expression.replace(";", "");

    let output = Command::new("python3")
        .arg("derivation_tree.py")
        .arg(expression)
        .output()
        .expect("Failed to execute python script");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    true
}