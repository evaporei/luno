use crate::ast;
use pest::iterators::Pair;
use pest::Parser as _;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

pub fn parse(source: &str) -> Result<ast::Program, pest::error::Error<Rule>> {
    // List of Expressions.
    let pairs = Parser::parse(Rule::Program, source)?.next().unwrap();

    println!("pairs: {:?}", pairs);

    let ast = match parse_program(pairs) {
        Ok(bb) => bb,
        Err(e) => panic!("error: {e}"),
    };

    Ok(ast)
}

fn parse_program(program: Pair<Rule>) -> Result<ast::Program, String> {
    let mut ast = ast::Program::new();
    for child in program.into_inner() {
        if child.as_rule() == Rule::EOI {
            continue;
        }
        ast.push(parse_expr(child)?);
    }
    Ok(ast)
}

fn parse_expr(expr: Pair<Rule>) -> Result<ast::Expr, String> {
    match expr.as_rule() {
        Rule::Expr => parse_expr(expr.into_inner().next().unwrap()),
        Rule::Int => Ok(ast::Expr::Int(
            expr.as_str().parse::<i64>().map_err(|e| e.to_string())?,
        )),
        Rule::Float => Ok(ast::Expr::Float(
            expr.as_str().parse::<f64>().map_err(|e| e.to_string())?,
        )),
        Rule::Program | Rule::EOI | Rule::WHITESPACE => unreachable!(),
    }
}