# calcpad-syntax

Rust-first syntax crate for Khwarizmi Studio.

## Current scope

- Lossless tokenization with UTF-8 byte spans.
- Basic token categories for numbers, identifiers, operators, delimiters, comments, and whitespace.
- Conservative expression spacing formatter.
- Unit tests for source preservation and core token behavior.

## Important limitation

This is an initial foundation, **not yet a full Calcpad parser or formatter replacement**. Calcpad has domain-specific syntax and semantic behavior. Before switching the IDE or calculation runtime to this crate, import the upstream source, collect representative worksheets, and add differential/golden tests. Unknown constructs must remain preserved until their syntax is understood.
