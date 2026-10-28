# Khwarizmi Studio

Khwarizmi Studio is a Rust-first engineering-calculation IDE inspired by the focused, single-pane editing experience of [MarkText](https://github.com/marktext/marktext), with Calcpad-compatible worksheets and engineering report publishing.

## Product direction

- **Single-pane editor:** edit the document in one continuous workspace. Markdown headings, prose, equations, calculation blocks, tables, and images should feel like one document—not separate source and preview panes.
- **Rust parser and formatter:** progressively replace the C# syntax/parser and formatting layer with Rust, backed by compatibility fixtures and differential tests. Keep the numerical calculation engine separate until its semantics can be migrated safely.
- **Autocomplete and diagnostics:** context-aware completion, syntax diagnostics, symbols, units, functions, and snippets from the Rust syntax layer.
- **PDF publishing:** a dedicated renderer and engineering-report templates for A4/Letter, margins, title pages, headers/footers, page numbering, equations, tables, and references.
- **MCP:** local, permission-scoped tools for project discovery, worksheet inspection, validation, controlled calculation runs, and PDF export.

## Editor interaction

Use MarkText as a reference for the **single-pane/WYSIWYG interaction model**, not as a requirement to copy its code. The primary editor should be a continuous page-like surface. Toolbars and contextual popovers handle formatting; file navigation, outline, diagnostics, and calculation results are optional collapsible side panels rather than permanently split source/preview views. Keep a source-mode escape hatch for advanced users and debugging.

The document model should represent Markdown and Calcpad constructs explicitly so editing does not destroy calculation syntax. Embedded calculation blocks need clear visual boundaries, diagnostics, and a way to inspect the underlying source.

## Architecture

- `crates/calcpad-syntax`: Rust lexical/syntax foundation and conservative formatter (initial scaffold; not full language parity yet).
- `crates/calcpad-engine` (planned): calculation integration and, later, candidate Rust numerical components.
- `apps/desktop` (planned): desktop shell and single-pane editor UI.
- `crates/pdf-publisher` (planned): renderer abstraction and print templates.
- `crates/mcp-server` (planned): local MCP transport and permission-scoped tools.

### Migration rules

1. Preserve upstream Calcpad license and copyright notices when source is imported.
2. Build a corpus of real worksheets and use golden/differential tests before switching parser behavior.
3. Keep lexical parsing, syntax/formatting, semantic analysis, numerical evaluation, document rendering, and MCP transport as separate layers.
4. Never silently rewrite a worksheet. Formatting should be previewable and reversible.
5. Do not claim Rust parity until supported syntax, errors, units, directives, and rendered results pass the regression suite.

## Current status

- The Rust syntax crate has an initial lossless lexer, byte spans, a conservative expression formatter, and unit tests.
- The full CalcpadCE upstream source has **not yet been imported into this repository**.
- The Rust crate has **not been compiled or tested in this environment**. The initial code is a starting point, not a release-ready parser/formatter replacement.
- The MarkText repository is used as a UI reference only; its code has not been copied.

## Upstream references

- [CalcpadCE](https://github.com/imartincei/CalcpadCE)
- [MarkText](https://github.com/marktext/marktext)

## License

CalcpadCE is MIT-licensed. Preserve upstream notices when importing code and verify the licenses/notices of all dependencies before distributing binaries.
