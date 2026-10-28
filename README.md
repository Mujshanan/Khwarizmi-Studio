# Khwarizmi Studio

Khwarizmi Studio is a Markdown-first engineering-calculation IDE. It combines a MarkText-inspired single-pane editor with a Rust parser, formatter, and calculation core informed by Calcpad's worksheet behavior.

## Product direction

- **Markdown-first editor:** prose, tables, figures, typeset equations, and calculation blocks live in one continuous document surface.
- **Rust calculation core:** reimplement the required parser, formatter, semantic analysis, and calculation behavior in Rust. Do not import CalcpadCE's WPF or ASP.NET application UI.
- **Explicit calculation blocks:** a \`calcpad\` Markdown fence invokes the calculation engine. Ordinary code fences and typesetting-only math are not evaluated.
- **Autocomplete and diagnostics:** completion for symbols, units, functions, and snippets; source-linked syntax and calculation errors.
- **PDF publishing:** a separate renderer for page templates, equations, figures, tables, numbering, headers/footers, and references.
- **MCP:** local, permission-scoped tools for project discovery, validation, controlled calculation runs, and PDF export.

## Editor base

MarkText is the proposed editor starting point because its MIT license permits reuse and modification subject to retaining required notices. We should adapt its editor shell rather than blindly copy every feature. Before release, audit its dependency licenses and bundled assets, retain applicable notices, and maintain a third-party notices file. See [the editor reuse and licensing decision](docs/architecture/editor-base-and-licensing.md).

## Document model

Markdown remains the canonical source. The proposed \`calcpad\` fenced block is the boundary between regular Markdown and evaluated engineering calculations. Inline and display math (\`$...$\`, \`$$...$$\`) are initially typesetting-only. Evaluation results and diagnostics are separate from the source and must never silently overwrite it.

See [Markdown calculation blocks](docs/architecture/markdown-calculations.md).

## Architecture

- \`crates/calcpad-syntax\`: Rust lexical/syntax foundation and conservative formatter (initial scaffold; not full language parity).
- \`crates/calcpad-engine\` (planned): semantic analysis, evaluation, units, and calculation result model.
- \`apps/desktop\` (planned): adapted MarkText-based desktop editor.
- \`crates/pdf-publisher\` (planned): document renderer and print templates.
- \`crates/mcp-server\` (planned): local MCP transport and permission-scoped tools.

### Migration rules

1. Import only the minimal editor code and calculation behavior required; do not import Calcpad's legacy UI projects.
2. Preserve MIT license and copyright notices for reused code, and audit transitive dependencies/assets.
3. Use real worksheet fixtures and golden/differential tests before claiming Calcpad compatibility.
4. Keep parsing, formatting, semantic analysis, evaluation, rendering, and MCP transport separate.
5. Never silently rewrite a worksheet; formatting must be explicit and reversible.
6. Do not claim Rust parity until supported syntax, errors, units, directives, and results pass regression tests.

## Current status

- The Rust syntax crate has an initial lossless lexer, byte spans, a conservative expression formatter, and unit tests.
- The \`calcpad\` Markdown fence is a proposed syntax documented for implementation; the parser does not yet evaluate it.
- MarkText's MIT license has been checked, and a reuse/compliance plan is documented. The MarkText code itself has **not yet been copied or integrated**.
- No Rust build/test or desktop build has been run in this environment. The current Rust crate is a scaffold, not a release-ready parser/formatter.

## Upstream references

- [CalcpadCE](https://github.com/imartincei/CalcpadCE) — reference behavior and source for targeted compatibility research.
- [MarkText](https://github.com/marktext/marktext) — proposed Markdown editor base.

## License

The project should retain its own license and third-party notices consistent with the code actually included. CalcpadCE and MarkText are MIT-licensed; retain their required notices for any reused code and audit all dependencies before distributing binaries.
