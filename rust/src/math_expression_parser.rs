/*
References:
 - https://lukaszwrobel.pl/blog/math-parser-part-1-introduction/
 - https://itnext.io/writing-a-mathematical-expression-parser-35b0b78f869e
 - https://www.youtube.com/watch?v=88lmIMHhYNs
 - https://youtu.be/4m7ubrdbWQU?si=aou9r58PCiDH4v-5
 - https://mathworld.wolfram.com/Precedence.html
 - https://craftinginterpreters.com/parsing-expressions.html
 - https://en.wikipedia.org/wiki/Shunting_yard_algorithm
*/

use std::str::{CharIndices, FromStr};

#[derive(Debug, Clone, Copy)]
enum Token {
  Number(f64),
  AddOperator,
  SubOperator,
  MulOperator,
  DivOperator,
  LeftParenthesis,
  RightParenthesis,
  SqrtOperator,
}

//math expression parser
pub fn parse_and_eval(expression: &str) -> f64 {
  let tokens = tokenize(expression);
  println!("tokens: {tokens:?}");

  let tree = shunting_yard_algorithm(&tokens);
  println!("shunting_yard_algorithm: {tree:?}");

  execute_shunting_yard_algorithm(&tree)
}

fn shunting_yard_algorithm(tokens: &Vec<Token>) -> Vec<Token> {
  let mut output_stack = Vec::new();
  let mut operator_stack = Vec::new();

  for token in tokens {
    match token {
      Token::Number(num) => output_stack.push(Token::Number(*num)),
      Token::AddOperator | Token::SubOperator => {
        if let Some(last_operator) = operator_stack.last() {          
          match last_operator {
            Token::MulOperator | Token::DivOperator => {
              while let Some(operator) = operator_stack.pop() {
                output_stack.push(operator)
              }
            },
            _ => {},
          }
        }

        operator_stack.push(*token);
      }
      Token::MulOperator | Token::DivOperator => {
        operator_stack.push(*token);
      }
      Token::SqrtOperator => {
        operator_stack.push(*token);
      }
      Token::LeftParenthesis => {
        operator_stack.push(*token);
      }
      Token::RightParenthesis => {
        while let Some(operator) = operator_stack.pop() {
          match operator {
            Token::LeftParenthesis => break,
            _ => output_stack.push(operator)
          }
        }
      }
    }
  }

  while let Some(operator) = operator_stack.pop() {
    output_stack.push(operator)
  }

  output_stack
}

fn execute_shunting_yard_algorithm(stack: &Vec<Token>) -> f64 {
  let mut result_stack = Vec::new();
  for el in stack {
    match el {
      Token::AddOperator => {
        let right = result_stack.pop().unwrap();
        let left = result_stack.pop().unwrap();

        result_stack.push(left + right);
      },
      Token::SubOperator => {
        let right = result_stack.pop().unwrap();
        let left = result_stack.pop().unwrap();

        result_stack.push(left - right);
      },
      Token::MulOperator => {
        let right = result_stack.pop().unwrap();
        let left = result_stack.pop().unwrap();

        result_stack.push(left * right);
      },
      Token::DivOperator => {
        let right = result_stack.pop().unwrap();
        let left = result_stack.pop().unwrap();

        result_stack.push(left / right);
      },
      Token::Number(num) => {
        result_stack.push(*num);
      },
      Token::SqrtOperator => {
        let num = result_stack.pop().unwrap();

        result_stack.push( f64::sqrt(num) );
      }
      Token::LeftParenthesis | Token::RightParenthesis => panic!("Parenthesis should not be in the execution step."),
    }
  }

  *result_stack.last().unwrap()
}

static VALID_NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
static VALID_NUMBERS_PLUS_DECIMAL_POINT: [char; 11] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];
fn tokenize(input: &str) -> Vec<Token> {
    let mut iterator = input.char_indices();
    let mut tokens = Vec::new();

    while let Some((idx, mut char)) = iterator.next() {
        if VALID_NUMBERS.contains(&char) {
            tokens.push(generate_number(idx, &mut char, &mut iterator, input));
        } 
        
        if '+' == char {
          tokens.push(Token::AddOperator);
        } else if '-' == char {
          tokens.push(Token::SubOperator);
        } else if '*' == char {
          tokens.push(Token::MulOperator);
        } else if '/' == char {
          tokens.push(Token::DivOperator);
        } else if '(' == char {
          tokens.push(Token::LeftParenthesis);
        } else if ')' == char {
          tokens.push(Token::RightParenthesis);
        } else if 's' == char {
          if "sqrt" == &input[idx..=3] {
            tokens.push(Token::SqrtOperator);
            iterator.next();
            iterator.next();
            iterator.next();
          }
        }
    }

    tokens
}

fn generate_number(
    idx: usize,
    current_char: &mut char,
    iterator: &mut CharIndices,
    input: &str,
) -> Token {
    let mut end_idx = idx;
    while let Some((idx, char)) = iterator.next() {
        *current_char = char;
        if !VALID_NUMBERS_PLUS_DECIMAL_POINT.contains(&char) {
            break;
        }
        end_idx = idx;
    }

    println!("start: {idx}, end: {end_idx}, input: {input}");
    Token::Number(f64::from_str(&input[idx..=end_idx]).unwrap())
}

mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(2.0, parse_and_eval("1+1"));
    assert_eq!(0.0, parse_and_eval("1-1"));
    assert_eq!(2.3, parse_and_eval("1.1+1.2"));
    assert_eq!(85.0, parse_and_eval("5+80"));
    assert_eq!(2.0, parse_and_eval("1*2"));
    assert_eq!(400.0, parse_and_eval("5*80"));
    assert_eq!(3.0, parse_and_eval("1+1*2"));
    assert_eq!(4.0, parse_and_eval("1*2+2"));
    assert_eq!(12.0, parse_and_eval("1*10+2"));

    assert_eq!(4.0, parse_and_eval("(1+1)*2"));

    assert_eq!(5.0, parse_and_eval("10/2"));
    assert_eq!(11.0, parse_and_eval("10/2+6"));
    assert_eq!(0.6, parse_and_eval("0.30*2/1"));
    assert_eq!(0.75, parse_and_eval("0.30*10/4"));
    
    assert_eq!(2.0, parse_and_eval("2"));
    assert_eq!(2.0, parse_and_eval("sqrt(4)"));
    assert_eq!(3.16227766016838.to_string()[..11], parse_and_eval("sqrt(10)").to_string()[..11]); //NOTE: a way to test if the precision is correct
  }
}
