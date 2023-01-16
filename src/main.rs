use std::collections::HashMap;

struct StackAutomaton {
    stack: Vec<State>,
    transition_table: HashMap<(State, Token), State>,
}

#[derive(Hash)]
enum State {
    Initial,
    StartParenthesis,
    AfterIdentifier,
    AfterOperator,
    AfterDigit,
    // other states
    Final,
    Invalid,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (State::Initial, State::Initial) => true,
            (State::StartParenthesis, State::StartParenthesis) => true,
            (State::AfterIdentifier, State::AfterIdentifier) => true,
            (State::AfterOperator, State::AfterOperator) => true,
            (State::AfterDigit, State::AfterDigit) => true,
            (State::Final, State::Final) => true,
            (State::Invalid, State::Invalid) => true,
            _ => false,
        }
    }
}

impl Eq for State {}

#[derive(Hash)]
enum Token {
    Operator(char),
    Parenthesis(char),
    Digit(char),
    Identifier(String),
    // other tokens
    Invalid,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Operator(c1), Token::Operator(c2)) => c1 == c2,
            (Token::Parenthesis(c1), Token::Parenthesis(c2)) => c1 == c2,
            (Token::Digit(c1), Token::Digit(c2)) => c1 == c2,
            (Token::Identifier(s1), Token::Identifier(s2)) => s1 == s2,
            (Token::Invalid, Token::Invalid) => true,
            _ => false,
        }
    }
}

impl Eq for Token {}

impl StackAutomaton {
    fn new() -> StackAutomaton {
        let mut transition_table = HashMap::new();
    
        transition_table.insert((State::Initial, Token::Parenthesis('(')), State::StartParenthesis);
        transition_table.insert((State::StartParenthesis, Token::Identifier(_)), State::AfterIdentifier);
        transition_table.insert((State::AfterIdentifier, Token::Operator(_)), State::AfterOperator);
        transition_table.insert((State::AfterOperator, Token::Identifier(_)), State::AfterIdentifier);
        transition_table.insert((State::AfterIdentifier, Token::Parenthesis(')')), State::Final);
        transition_table.insert((State::AfterOperator, Token::Digit(_)), State::AfterDigit);
        transition_table.insert((State::AfterDigit, Token::Parenthesis(')')), State::Final);
        transition_table.insert((State::Initial, Token::Invalid), State::Invalid);
        transition_table.insert((State::StartParenthesis, Token::Invalid), State::Invalid);
        transition_table.insert((State::AfterIdentifier, Token::Invalid), State::Invalid);
        transition_table.insert((State::AfterOperator, Token::Invalid), State::Invalid);
        transition_table.insert((State::AfterDigit, Token::Invalid), State::Invalid);
        
        StackAutomaton {
            stack: vec![State::Initial],
            transition_table,
        }
    }

    fn transition(&mut self, token: Token) {
        let current_state = *self.stack.last().unwrap();
        match self.transition_table.get(&(current_state, token)) {
            Some(next_state) => self.stack.push(*next_state),
            None => self.stack.push(State::Invalid),
        }
    }

    fn is_final_state(&self) -> bool {
        match self.stack.last() {
            Some(State::Final) => true,
            _ => false,
        }
    }
}

fn parse(input: &str) -> bool {
    let mut automaton = StackAutomaton::new();
    for c in input.chars() {
        match c {
            '=' | '+' | '-' | '*' | '/' | '%' => automaton.transition(Token::Operator(c)),
            '(' | ')' => automaton.transition(Token::Parenthesis(c)),
            '0'..='9' => automaton.transition(Token::Digit(c)),
            'A'..='Z' | 'a'..='z' | '_' => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(c) = input.chars().nth(identifier.len()) {
                    if c.is_alphanumeric() || c == '_' {
                        identifier.push(c);
                    } else {
                        break;
                    }
                }
                automaton.transition(Token::Identifier(identifier))
            }
            _ => automaton.transition(Token::Invalid),
        }
    }
    automaton.is_final_state()
}
