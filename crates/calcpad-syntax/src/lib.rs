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
                if !out.is_empty() && !(out.ends_with(' ') || out.ends_with('\n')) { out.push(' '); }
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


/// A fenced Markdown block whose info string starts with `calcpad`.
/// Spans are byte offsets into the original Markdown source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalculationBlock {
    /// Includes the opening fence, body, and closing fence when present.
    pub span: Range<usize>,
    /// Contains only the block body, excluding both fences.
    pub content_span: Range<usize>,
}

/// Finds explicit `calcpad` fenced blocks in Markdown without rewriting the source.
///
/// This deliberately handles only fenced blocks, not inline code or indented code.
/// Ordinary fences (for example ```rust) are ignored. Unclosed calculation fences
/// extend to end-of-file, matching Markdown's fenced-code behavior.
pub fn find_calculation_blocks(markdown: &str) -> Vec<CalculationBlock> {
    #[derive(Clone)]
    struct Fence {
        marker: char,
        length: usize,
        start: usize,
        content_start: usize,
    }

    let mut blocks = Vec::new();
    let mut active: Option<Fence> = None;
    let mut offset = 0usize;

    for line_with_newline in markdown.split_inclusive('\n') {
        let line = line_with_newline.trim_end_matches('\n').trim_end_matches('\r');
        let leading = line.chars().take_while(|c| *c == ' ').count();
        let rest = if leading <= 3 { &line[leading..] } else { "" };
        let marker = rest.chars().next();
        let marker_count = marker
            .filter(|c| *c == '`' || *c == '~')
            .map(|c| rest.chars().take_while(|x| *x == c).count())
            .unwrap_or(0);

        if let Some(fence) = active.as_ref() {
            let is_closing = leading <= 3
                && marker == Some(fence.marker)
                && marker_count >= fence.length
                && rest[marker_count..].trim().is_empty();

            if is_closing {
                let content_end = offset;
                let block_end = offset + line_with_newline.len();
                blocks.push(CalculationBlock {
                    span: fence.start..block_end,
                    content_span: fence.content_start..content_end,
                });
                active = None;
            }
        } else if leading <= 3 && marker_count >= 3 {
            let info = rest[marker_count..].trim();
            let is_calcpad = marker != Some('`') || !info.contains('`');
            let language = info.split_whitespace().next().unwrap_or("");
            if is_calcpad && language.eq_ignore_ascii_case("calcpad") {
                active = Some(Fence {
                    marker: marker.unwrap(),
                    length: marker_count,
                    start: offset,
                    content_start: offset + line_with_newline.len(),
                });
            }
        }

        offset += line_with_newline.len();
    }

    if let Some(fence) = active {
        blocks.push(CalculationBlock {
            span: fence.start..markdown.len(),
            content_span: fence.content_start..markdown.len(),
        });
    }
    blocks
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
    #[test]
    fn finds_calcpad_blocks_and_preserves_byte_spans() {
        let markdown = "Text\n\n```rust\nlet x = 1;\n```\n\n```calcpad\nL := 6 m\nR := L / 2\n```\n";
        let blocks = find_calculation_blocks(markdown);
        assert_eq!(blocks.len(), 1);
        assert_eq!(&markdown[blocks[0].content_span.clone()], "L := 6 m\nR := L / 2\n");
        assert_eq!(&markdown[blocks[0].span.clone()], "```calcpad\nL := 6 m\nR := L / 2\n```\n");
    }

    #[test]
    fn supports_tilde_fences_and_unclosed_blocks() {
        let markdown = "~~~Calcpad\nx := 2\n";
        let blocks = find_calculation_blocks(markdown);
        assert_eq!(blocks.len(), 1);
        assert_eq!(&markdown[blocks[0].content_span.clone()], "x := 2\n");
        assert_eq!(blocks[0].span, 0..markdown.len());
    }

    #[test]
    fn ignores_non_calcpad_and_indented_fences() {
        let markdown = "    ```calcpad\nignored\n    ```\n```text\nnot a calculation\n```\n";
        assert!(find_calculation_blocks(markdown).is_empty());
    }

}
