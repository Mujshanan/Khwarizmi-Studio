# Single-pane editor interaction model

## Goal

Provide a MarkText-like continuous editing surface for engineering calculations. The default experience is one document surface, not a permanently split Markdown source and rendered preview.

## Main surface

- A single scrollable page with a readable content width and page-like margins.
- Markdown paragraphs and headings render as formatted content while remaining directly editable.
- Equations, tables, images, and Calcpad calculation blocks are first-class document nodes.
- Calculation blocks show source-aware syntax coloring, completion, diagnostics, and a compact run/result affordance.
- Clicking a formatted node places the caret in the relevant source range; selection and undo must remain predictable.
- A source mode is available for debugging and advanced editing, but is not the default.

## Chrome

- Top bar: document title, save state, undo/redo, run, and export.
- Compact formatting toolbar appears on selection or through keyboard shortcuts.
- Outline, project files, diagnostics, and calculation output are optional collapsible panels.
- Panels overlay or dock temporarily; they must not force a second editor/preview pane into the default layout.
- Status bar shows cursor position, document mode, diagnostics count, and calculation state.

## Document and source integrity

Use a structured document model with stable source ranges. Rendering must not normalize or rewrite the source merely because a node is displayed in rich form. Edits are explicit source transformations with undo support. Unknown Calcpad syntax must remain editable and round-trip unchanged.

## Implementation order

1. Define document nodes and source-range mapping.
2. Add a static UI shell and single-pane editing interactions.
3. Implement Markdown block/inline editing and selection mapping.
4. Embed Calcpad blocks using the Rust syntax crate.
5. Add diagnostics/autocomplete and safe calculation execution.
6. Add regression tests for round-tripping, cursor mapping, undo, and mixed Markdown/calculation documents.

## Reference

[MarkText](https://github.com/marktext/marktext) is a behavioral and visual reference for focused single-pane/WYSIWYG Markdown editing. Do not copy its source code unless the project explicitly chooses and satisfies the applicable license and attribution requirements.
