mod stack_automaton;

fn main() {
    let expressions = [
        "A2 = A1 + 12 + C5;",
        "AB = A*B/100-59;",
        "sdf = (340 % 2) + (12-C);",
        "AC = 10 + 8 * (5+B);",
        "VAR = CatA + ( ( CatA + CatB * CatC );",
        "s2 = 10 + 8 * (5+B));",
        "sdfr4 = 10 + 8 * ((((5+B);",
    ];
    for expression in expressions.iter() {
        println!(
            "Expression '{}' is valid: {}",
            expression,
            stack_automaton::validate_expression(expression)
        );
    }
}
