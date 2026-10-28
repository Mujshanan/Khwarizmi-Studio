//! Small, deliberately limited arithmetic evaluator.
//!
//! This is the first executable slice of the calculation core. It supports
//! numeric literals, variables, parentheses, unary signs, and + - * / ^.
//! Units, functions, directives, and full Calcpad compatibility are not yet
//! implemented.

use std::collections::HashMap;
use std::fmt;

/// An expression evaluation error with a UTF-8 byte offset into the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    pub offset: usize,
    pub message: String,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.message, self.offset)
    }
}

impl std::error::Error for EvalError {}

/// Evaluates a basic arithmetic expression using the supplied symbol table.
///
/// Operator precedence follows conventional arithmetic; exponentiation is
/// right-associative, and exponentiation binds more tightly than unary signs.
/// This function intentionally does not guess unit semantics.
pub fn evaluate_expression(
    source: &str,
    variables: &HashMap<String, f64>,
) -> Result<f64, EvalError> {
    let mut parser = Parser { source, offset: 0, variables };
    let value = parser.parse_additive()?;
    parser.skip_whitespace();
    if parser.offset != source.len() {
        return Err(parser.error("Unexpected token"));
    }
    if !value.is_finite() {
        return Err(parser.error("Expression result is not finite"));
    }
    Ok(value)
}

struct Parser<'a> {
    source: &'a str,
    offset: usize,
    variables: &'a HashMap<String, f64>,
}

impl Parser<'_> {
    fn error(&self, message: &str) -> EvalError {
        EvalError { offset: self.offset, message: message.to_string() }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.offset..)?.chars().next()
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.offset += ch.len_utf8();
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.bump();
        }
    }

    fn eat(&mut self, expected: char) -> bool {
        self.skip_whitespace();
        if self.peek() == Some(expected) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn parse_additive(&mut self) -> Result<f64, EvalError> {
        let mut value = self.parse_multiplicative()?;
        loop {
            if self.eat('+') {
                value += self.parse_multiplicative()?;
            } else if self.eat('-') {
                value -= self.parse_multiplicative()?;
            } else {
                return Ok(value);
            }
        }
    }

    fn parse_multiplicative(&mut self) -> Result<f64, EvalError> {
        let mut value = self.parse_unary()?;
        loop {
            if self.eat('*') {
                value *= self.parse_unary()?;
            } else if self.eat('/') {
                let divisor = self.parse_unary()?;
                if divisor == 0.0 {
                    return Err(self.error("Division by zero"));
                }
                value /= divisor;
            } else {
                return Ok(value);
            }
        }
    }

    fn parse_unary(&mut self) -> Result<f64, EvalError> {
        if self.eat('+') {
            self.parse_unary()
        } else if self.eat('-') {
            Ok(-self.parse_unary()?)
        } else {
            self.parse_power()
        }
    }

    fn parse_power(&mut self) -> Result<f64, EvalError> {
        let base = self.parse_primary()?;
        if self.eat('^') {
            let exponent = self.parse_unary()?;
            Ok(base.powf(exponent))
        } else {
            Ok(base)
        }
    }

    fn parse_primary(&mut self) -> Result<f64, EvalError> {
        self.skip_whitespace();
        let start = self.offset;
        match self.peek() {
            Some('(') => {
                self.bump();
                let value = self.parse_additive()?;
                if !self.eat(')') {
                    return Err(self.error("Expected closing parenthesis"));
                }
                Ok(value)
            }
            Some(ch) if ch.is_ascii_digit() || ch == '.' => self.parse_number(),
            Some(ch) if ch == '_' || ch.is_alphabetic() => self.parse_identifier(),
            Some(_) => Err(self.error("Expected a number, variable, or parenthesized expression")),
            None => Err(EvalError { offset: start, message: "Expected an expression".to_string() }),
        }
    }

    fn parse_number(&mut self) -> Result<f64, EvalError> {
        let start = self.offset;
        let mut seen_exponent = false;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                self.bump();
            } else if (ch == 'e' || ch == 'E') && !seen_exponent {
                seen_exponent = true;
                self.bump();
                if matches!(self.peek(), Some('+' | '-')) {
                    self.bump();
                }
            } else {
                break;
            }
        }
        let literal = &self.source[start..self.offset];
        literal.parse::<f64>().map_err(|_| EvalError {
            offset: start,
            message: format!("Invalid numeric literal '{literal}'"),
        })
    }

    fn parse_identifier(&mut self) -> Result<f64, EvalError> {
        let start = self.offset;
        self.bump();
        while self.peek().is_some_and(|ch| ch == '_' || ch.is_alphanumeric()) {
            self.bump();
        }
        let name = &self.source[start..self.offset];
        self.variables.get(name).copied().ok_or_else(|| EvalError {
            offset: start,
            message: format!("Unknown variable '{name}'"),
        })
    }
}

#[cfg(test)]
mod eval_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn evaluates_precedence_and_parentheses() {
        let vars = HashMap::new();
        assert_eq!(evaluate_expression("2 + 3 * (4 - 1)", &vars), Ok(11.0));
    }

    #[test]
    fn exponentiation_is_right_associative_and_binds_over_unary_minus() {
        let vars = HashMap::new();
        assert_eq!(evaluate_expression("2^3^2", &vars), Ok(512.0));
        assert_eq!(evaluate_expression("-2^2", &vars), Ok(-4.0));
    }

    #[test]
    fn resolves_variables_and_scientific_notation() {
        let vars = HashMap::from([("L".to_string(), 6.0)]);
        assert_eq!(evaluate_expression("L / 2 + 1.5e1", &vars), Ok(18.0));
    }

    #[test]
    fn reports_unknown_variables_and_division_by_zero() {
        let vars = HashMap::new();
        assert!(evaluate_expression("missing + 1", &vars).unwrap_err().message.contains("Unknown variable"));
        assert!(evaluate_expression("4 / (2 - 2)", &vars).unwrap_err().message.contains("Division by zero"));
    }

    #[test]
    fn rejects_trailing_and_malformed_input() {
        let vars = HashMap::new();
        assert!(evaluate_expression("2 3", &vars).is_err());
        assert!(evaluate_expression("1.2.3", &vars).is_err());
        assert!(evaluate_expression("(2 + 3", &vars).is_err());
    }
}
