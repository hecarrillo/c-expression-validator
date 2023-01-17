use std::collections::VecDeque;
mod char_validations;

#[derive(Debug, PartialEq)]
enum State {
    Start,              // q0
    Variable,           // q1
    AfterAssignation,   // q2
    VariableExpression, // q3
    Operator,           // q4
    DigitExpression,    // q5
    Final,              // q6
    Reject,             // q7
}

// state machine that contains the current state and the stack
#[derive(Debug)]
struct StateMachine {
    state: State,
    stack: VecDeque<char>,
}

// State machine transitions

// Transition from q0 to q1 if the character is a valid variable identifier. Dont push to stack. Receive a state machine and edit it
fn start(c: char, mut state_machine: StateMachine) -> StateMachine {
    if char_validations::is_variable_identifier(c) {
        state_machine.state = State::Variable;
    } else {
        state_machine.state = State::Reject;
    }
    state_machine
}

// Transition from q1 to q2 if the character is an equal sign. Stay in q1 if the character is a valid variable identifier. Dont push to stack
// Recieve a state machine and edit it
fn variable(c: char, mut state_machine: StateMachine) -> StateMachine {
    if c == '=' {
        state_machine.state = State::AfterAssignation;
    } else if char_validations::is_variable_identifier(c) {
        state_machine.state = State::Variable;
    } else {
        state_machine.state = State::Reject;
    }
    state_machine
}

// Transition from q2 to q2 if the char is an opening parenthesis and push the parenthesis to the stack.
// Transition from q2 to q3 if the char is a valid variable identifier. Dont push to stack
// Transition from q2 to q5 if the char is a digit. Dont push to stack
// Receive a state machine and edit it
fn after_assignation(c: char, mut state_machine: StateMachine) -> StateMachine {
    if c == '(' {
        state_machine.stack.push_back(c);
        state_machine.state = State::AfterAssignation;
    } else if char_validations::is_variable_identifier(c) {
        state_machine.state = State::VariableExpression;
    } else if char_validations::is_digit(c) {
        state_machine.state = State::DigitExpression;
    } else {
        state_machine.state = State::Reject;
    }
    state_machine
}

// Transition from q3 to q3 if the char is a valid variable identifier or digit. Dont push to stack
// Transition from q3 to q3 if the char is a closing parenthesis and pop the stack. Dont push to stack
// Transition from q3 to q4 if the char is an operator. Dont push to stack
// Transition from q3 to q6 if the char is a semicolon. Dont push to stack
// Receive a state machine and edit it in place
fn variable_expression(c: char, mut state_machine: StateMachine) -> StateMachine {
    if char_validations::is_variable_identifier(c) || char_validations::is_digit(c) {
        state_machine.state = State::VariableExpression;
    } else if c == ')' {
        if state_machine.stack.is_empty() {
            state_machine.state = State::Reject;
            return state_machine;
        }
        state_machine.stack.pop_back();
        state_machine.state = State::VariableExpression;
    } else if char_validations::is_operator(c) {
        state_machine.state = State::Operator;
    } else if c == ';' && state_machine.stack.is_empty() {
        state_machine.state = State::Final;
    } else {
        state_machine.state = State::Reject;
    }
    state_machine
}

// Transition from q4 to q2 if the char is an opening parenthesis and push the parenthesis to the stack.
// Transition from q4 to q3 if the char is a valid variable identifier. Dont push to stack
// Transition from q4 to q5 if the char is a digit. Dont push to stack
// Receive a state machine and edit it in place
fn operator(c: char, mut state_machine: StateMachine) -> StateMachine {
    if c == '(' {
        state_machine.stack.push_back(c);
        state_machine.state = State::AfterAssignation;
    } else if char_validations::is_variable_identifier(c) {
        state_machine.state = State::VariableExpression;
    } else if char_validations::is_digit(c) {
        state_machine.state = State::DigitExpression;
    } else {
        state_machine.state = State::Reject;
    }
    state_machine
}

// Transition from q5 to q5 if the char is a digit. Dont push to stack
// Transition from q5 to q5 if the char is a closing parenthesis and pop the stack. Dont push to stack
// Transition from q5 to q4 if the char is an operator. Dont push to stack
// Transition from q5 to q6 if the char is a semicolon. Dont push to stack
// Receive a state machine and edit it in place
fn digit_expression(c: char, mut state_machine: StateMachine) -> StateMachine {
    if char_validations::is_digit(c) {
        state_machine.state = State::DigitExpression;
    } else if c == ')' {
        if state_machine.stack.is_empty() {
            state_machine.state = State::Reject;
            return state_machine;
        }
        state_machine.stack.pop_back();
        state_machine.state = State::DigitExpression;
    } else if char_validations::is_operator(c) {
        state_machine.state = State::Operator;
    } else if c == ';' && state_machine.stack.is_empty() {
        state_machine.state = State::Final;
    } else {
        state_machine.state = State::Reject;
    }
    state_machine
}

// transition function that receives a string, creates a state machine and iterates over the string to validate it
fn validate_expression(expression: &str) -> bool {
    let mut state_machine = StateMachine {
        state: State::Start,
        stack: VecDeque::new(),
    };
    for c in expression.chars() {
        if c == ' ' {
            continue;
        }
        match state_machine.state {
            State::Start => state_machine = start(c, state_machine),
            State::Variable => state_machine = variable(c, state_machine),
            State::AfterAssignation => state_machine = after_assignation(c, state_machine),
            State::VariableExpression => state_machine = variable_expression(c, state_machine),
            State::Operator => state_machine = operator(c, state_machine),
            State::DigitExpression => state_machine = digit_expression(c, state_machine),
            State::Final => return true,
            State::Reject => return false,
        }
    }
    if state_machine.state == State::Final {
        true
    } else {
        false
    }
}

fn main() {
    let expressions = [
        "A2 = A1 + 12 + C5;",
        "AB = A*B/100-59;",
        "ABC = (340 % 2) + (12-C);",
        "AC = 10 + 8 * (5+B);",
        "VAR = CatA + ( ( CatA + CatB ) * CatC );",
        "s2 = 10 + 8 * (5+B));",
        "sdfr4 = 10 + 8 * ((((5+B);",
    ];
    for expression in expressions.iter() {
        println!(
            "Expression '{}' is valid: {}",
            expression,
            validate_expression(expression)
        );
    }
}
