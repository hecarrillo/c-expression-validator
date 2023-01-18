import nltk
from nltk import CFG

# Define the grammar
grammar = CFG.fromstring("""
    S -> VAR '=' E ';'
    E -> T | T '+' E | T '-' E
    T -> F | F '*' T | F '/' T
    F -> VAR | CONST | '(' E ')'
    VAR -> 'A' | 'B' | 'C' | 'CatA' | 'CatB' | 'CatC' | 'VAR'
    CONST -> '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    """)


# Define the function
def parse_expression(expression):
    parser = nltk.ChartParser(grammar)
    # print the tree on the terminal or a file
    with open("tree.txt", "w") as f:
        for tree in parser.parse(expression.split()):
            tree.pretty_print(stream=f)


# Test the function with an example expression
expression = "VAR = CatA + ( ( CatA + CatB ) * CatC )"
parse_expression(expression)
