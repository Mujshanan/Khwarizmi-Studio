//! Rust-first syntax layer for Khwarizmi Studio.
//!
//! This crate intentionally starts with lossless lexical scanning and conservative
//! formatting. It is not yet a replacement for the complete Calcpad semantic parser;
//! compatibility fixtures from upstream must be ported before claiming parity.

use std::ops::Range;

/// A source token with a byte range into the original UTF-8 source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    Newline,
    Number,
    Identifier,
    Operator,
    OpenDelimiter,
    CloseDelimiter,
    Comment,
    Punctuation,
    Unknown,
}

/// Lexes a worksheet while preserving all source text and byte spans.
/// This first pass recognizes common mathematical tokens and keeps unknown syntax intact.
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = source.char_indices().peekable();

    while let Some((start, ch)) = iter.next() {
        let mut end = start + ch.len_utf8();
        let kind = match ch {
            '\n' => TokenKind::Newline,
            '\r' => {
                if let Some(&(next_pos, '\n')) = iter.peek() {
                    iter.next();
                    end = next_pos + 1;
                }
                TokenKind::Newline
            }
            c if c.is_whitespace() => {
                while let Some(&(pos, next)) = iter.peek() {
                    if next.is_whitespace() && next != '\n' && next != '\r' {
                        iter.next();
                        end = pos + next.len_utf8();
                    } else {
                        break;
                    }
                }
                TokenKind::Whitespace
            }
            '#' => {
                while let Some(&(pos, next)) = iter.peek() {
                    if next == '\n' || next == '\r' { break; }
                    iter.next();
                    end = pos + next.len_utf8();
                }
                TokenKind::Comment
            }
            c if c.is_ascii_digit() || (c == '.' && iter.peek().is_some_and(|(_, n)| n.is_ascii_digit())) => {
                let mut seen_exp = false;
                while let Some(&(pos, next)) = iter.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        iter.next();
                        end = pos + next.len_utf8();
                    } else if (next == 'e' || next == 'E') && !seen_exp {
                        seen_exp = true;
                        iter.next();
                        end = pos + 1;
                        if let Some(&(sign_pos, sign @ ('+' | '-'))) = iter.peek() {
                            let _ = sign;
                            iter.next();
                            end = sign_pos + 1;
                        }
                    } else { break; }
                }
                TokenKind::Number
            }
            c if is_ident_start(c) => {
                while let Some(&(pos, next)) = iter.peek() {
                    if is_ident_continue(next) {
                        iter.next();
                        end = pos + next.len_utf8();
                    } else { break; }
                }
                TokenKind::Identifier
            }
            '(' | '[' | '{' => TokenKind::OpenDelimiter,
            ')' | ']' | '}' => TokenKind::CloseDelimiter,
            '+' | '-' | '*' | '/' | '^' | '=' | '<' | '>' | '!' | '&' | '|' | '%' | '±' | '×' | '÷' | '≤' | '≥' | '≠' => {
                // Group adjacent ASCII operator characters (<=, >=, !=, ==, etc.).
                while let Some(&(pos, next)) = iter.peek() {
                    if next.is_ascii_punctuation() && "+-*/^=<>!&|%".contains(next) {
                        iter.next();
                        end = pos + next.len_utf8();
                    } else { break; }
                }
                TokenKind::Operator
            }
            ',' | ';' | ':' => TokenKind::Punctuation,
            _ => TokenKind::Unknown,
        };

        tokens.push(Token { kind, text: source[start..end].to_string(), span: start..end });
    }
    tokens
}

fn is_ident_start(c: char) -> bool {
    c == '_' || c.is_alphabetic() || c == 'µ' || c == 'π' || c == 'Δ' || c == 'δ'
}
fn is_ident_continue(c: char) -> bool {
    is_ident_start(c) || c.is_ascii_digit() || c == '₀' || c == '₁' || c == '₂' || c == '₃'
}

/// Returns a conservative normalized representation for simple expressions.
/// It deliberately preserves comments, line breaks, unknown symbols, and directive text.
/// Full Calcpad formatting rules will be added against upstream golden fixtures.
pub fn format_expression(source: &str) -> String {
    let tokens = lex(source);
    let mut out = String::new();
    let mut previous: Option<TokenKind> = None;

    for token in tokens {
        match token.kind {
            TokenKind::Whitespace => {
                // Whitespace is emitted only where it separates word-like tokens.
                let next_is_word = token.span.end < source.len()
                    && source[token.span.end..].chars().next().is_some_and(is_ident_start);
                let prev_is_word = matches!(previous, Some(TokenKind::Identifier | TokenKind::Number));
                if prev_is_word && next_is_word && !out.ends_with(' ') {
                    out.push(' ');
                }
            }
            TokenKind::Newline => {
                while out.ends_with(' ') { out.pop(); }
                out.push('\n');
            }
            TokenKind::Comment => {
                if !out.is_empty() && !out.ends_with([' ', '\n']) { out.push(' '); }
                out.push_str(&token.text);
            }
            TokenKind::Operator => {
                while out.ends_with(' ') { out.pop(); }
                if !out.is_empty() && !out.ends_with('\n') { out.push(' '); }
                out.push_str(&token.text);
                out.push(' ');
            }
            TokenKind::Punctuation => {
                while out.ends_with(' ') { out.pop(); }
                out.push_str(&token.text);
                if token.text == "," || token.text == ";" { out.push(' '); }
            }
            _ => {
                if matches!(token.kind, TokenKind::Identifier | TokenKind::Number)
                    && matches!(previous, Some(TokenKind::Identifier | TokenKind::Number))
                    && !out.ends_with([' ', '\n'])
                {
                    out.push(' ');
                }
                out.push_str(&token.text);
            }
        }
        previous = Some(token.kind);
    }
    out.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_preserves_source_when_tokens_are_joined() {
        let source = "a = 2.5 * π # note\n";
        let joined: String = lex(source).into_iter().map(|t| t.text).collect();
        assert_eq!(joined, source);
    }

    #[test]
    fn lexer_tracks_utf8_byte_spans() {
        let tokens = lex("π = 3");
        assert_eq!(&"π = 3"[tokens[0].span.clone()], "π");
    }

    #[test]
    fn formatter_normalizes_basic_operator_spacing() {
        assert_eq!(format_expression("x=2+3"), "x = 2 + 3");
    }

    #[test]
    fn formatter_keeps_newlines_and_comments() {
        assert_eq!(format_expression("x=2\ny=3 # note"), "x = 2\ny = 3 # note");
    }
}
