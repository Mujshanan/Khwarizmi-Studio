# Markdown + calculation document model

## Decision

Markdown is the canonical document format. Calculations are explicit Markdown fenced blocks, not a second document language and not a hidden preview-only representation. The editor renders prose and calculation blocks in one continuous editing surface.

## Authoring syntax

Use standard Markdown for prose and standard math delimiters for typeset-only mathematics:

- Inline typesetting: `$f(x) = x^2$`
- Display typesetting: `$$\int_0^L M(x)\,dx$$`

Use a `calcpad` fenced block when the contents must be parsed and evaluated by the Rust calculation engine:

```calcpad
# Simply supported beam example
L := 6 m
w := 12 kN/m
R_A := w * L / 2
R_B := w * L / 2
M_max := w * L^2 / 8
```

This is a proposed document syntax, not a claim that every line above is already supported by the current parser. The first implementation should define and test the exact assignment, unit, and output grammar before exposing it as stable.

## Rendering behavior

1. The editor treats each fenced `calcpad` block as a structured calculation node.
2. Source text remains authoritative and is preserved when the document is opened, edited, formatted, or saved.
3. Evaluation produces a separate result model: values, units, diagnostics, dependency links, and optional formatted output.
4. Results are rendered inline beneath or beside the block without being written back into the source unless the user explicitly inserts them.
5. A failed calculation displays a source-linked diagnostic; it must not erase the previous source or silently substitute a guessed result.
6. Ordinary Markdown code fences remain code fences. Only the `calcpad` info string invokes the calculation engine.
7. `$...$` and `$$...$$` are initially typesetting syntax. They must not be evaluated as Calcpad expressions automatically.

## Parser boundaries

Keep these layers separate:

- Markdown block parser: identifies paragraphs, headings, lists, tables, images, math, and fenced blocks.
- Calcpad lexer/parser: parses only the contents of a calculation block.
- Semantic layer: symbols, units, function resolution, and dependency analysis.
- Evaluation engine: calculates values and reports errors.
- Formatter: formats supported calculation syntax without changing semantics.
- Renderer/exporter: displays the document and produces paginated PDF output.

## Incremental implementation

1. Add Markdown document fixtures containing ordinary prose, tables, images, inline/display math, ordinary code fences, and `calcpad` blocks.
2. Implement lossless detection of fenced calculation blocks with original byte ranges.
3. Parse a deliberately small expression grammar and report source spans for errors.
4. Add symbol assignments and arithmetic with precedence, parentheses, unary operators, and exponentiation.
5. Add functions and units only after their semantics and tests are specified.
6. Add incremental evaluation and editor diagnostics.
7. Add PDF rendering from the same document model used by the editor.

## Non-goals

- Do not convert an entire Markdown document into a Calcpad worksheet.
- Do not evaluate arbitrary code fences.
- Do not store computed values as the only copy of a result.
- Do not promise full Calcpad compatibility until a licensed upstream behavior corpus and regression suite validate it.
