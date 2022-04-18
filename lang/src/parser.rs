use crate::ast;
use ordered_float::OrderedFloat;
use pest::iterators::Pair;
use pest::Parser as _;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

pub fn parse(source: &str) -> Result<ast::Program, pest::error::Error<Rule>> {
    let pairs = Parser::parse(Rule::Program, source)?.next().unwrap();

    // println!("pairs: {:?}", pairs);

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
        Rule::Float => Ok(ast::Expr::Float(OrderedFloat(
            expr.as_str().parse::<f64>().map_err(|e| e.to_string())?,
        ))),
        Rule::Int => Ok(ast::Expr::Int(
            expr.as_str().parse::<i64>().map_err(|e| e.to_string())?,
        )),
        Rule::Bool => {
            let expr_str = expr.as_str();
            let b = match expr_str {
                "true" => true,
                "false" => false,
                _ => return Err(format!("this ain't boolean: {expr_str}")),
            };

            Ok(ast::Expr::Bool(b))
        }
        Rule::Character => Ok(ast::Expr::Character(
            expr.as_str().chars().skip(1).next().unwrap(),
        )),
        Rule::Keyword => {
            let mut expr_str = expr.as_str().to_string();
            expr_str.remove(0); // :

            Ok(ast::Expr::Keyword(expr_str))
        }
        Rule::Symbol => Ok(ast::Expr::Symbol(expr.as_str().into())),
        Rule::String => {
            let mut expr_str = expr.as_str().to_string();
            expr_str.pop(); // remove ending "
            expr_str.remove(0); // remove starting "

            Ok(ast::Expr::String(expr_str))
        }
        Rule::Map => {
            let mut map = ast::Map::new();

            for child in expr.into_inner() {
                let c = child.as_str();
                if c == "{" || c == "}" {
                    continue;
                }

                if child.as_rule() == Rule::Pair {
                    let mut map_pair = child.into_inner();

                    let expr1 = map_pair.next().unwrap();
                    let expr2 = map_pair.next().unwrap();

                    map.insert(parse_expr(expr1)?, parse_expr(expr2)?);
                }
            }

            Ok(ast::Expr::Map(map))
        }
        Rule::List => {
            let mut list = ast::List::new();

            for child in expr.into_inner() {
                let c = child.as_str();
                if c == "[" || c == "]" {
                    continue;
                }
                list.push(parse_expr(child)?);
            }

            Ok(ast::Expr::List(list))
        }
        Rule::SExpr => {
            let mut sexpr = ast::SExpr::new();

            for child in expr.into_inner() {
                let c = child.as_str();
                if c == "(" || c == ")" {
                    continue;
                }
                sexpr.push(parse_expr(child)?);
            }

            Ok(ast::Expr::SExpr(sexpr))
        }
        Rule::Program
        | Rule::EOI
        | Rule::COMMENT
        | Rule::WHITESPACE
        | Rule::Inner
        | Rule::Char
        | Rule::Letter
        | Rule::ArithmeticOps
        | Rule::ComparisonOps
        | Rule::Pair => unreachable!(),
    }
}
