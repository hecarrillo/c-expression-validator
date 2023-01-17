// Function to check if a character is an operator
pub fn is_operator(c: char) -> bool {
    match c {
        '=' | '+' | '-' | '*' | '/' | '%' => true,
        _ => false,
    }
}

// Function to check if a character is a digit
pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

// Function to check if a character is a valid variable identifier
pub fn is_variable_identifier(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '_'
}