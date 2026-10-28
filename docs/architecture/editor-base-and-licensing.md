# Editor base and licensing strategy

## Decision

Use MarkText as the starting point for the Markdown editing experience where its existing architecture is useful, then replace or remove features that conflict with Khwarizmi Studio's engineering-document model. Do not import CalcpadCE's WPF or ASP.NET UI projects.

MarkText's repository license is MIT. MIT generally permits reuse, modification, redistribution, and commercial use, subject to retaining the required copyright and license notice. This is a project-level decision record, not legal advice.

Reference: https://github.com/marktext/marktext/blob/master/LICENSE

## Required compliance work

Before copying files or shipping a binary:

1. Keep the MarkText MIT license and applicable copyright notices for any reused code.
2. Track reused files and modifications in a third-party notices document.
3. Audit the licenses of transitive npm packages and any bundled runtime assets; MarkText's license does not override dependency licenses.
4. Keep Calcpad-derived code and behavior traceable separately. If any Calcpad source is copied, preserve its copyright and MIT license notices; a clean-room Rust reimplementation should still use upstream behavior tests without copying implementation text.
5. Verify fonts, icons, themes, sample documents, and other assets independently.
6. Generate a third-party license report as part of release packaging.

## What to reuse

Potentially reuse the editor shell and interaction patterns: single-pane editing, Markdown-aware rendering, keyboard shortcuts, document navigation, themes, and file open/save workflows.

Do not blindly retain the whole application. Remove unrelated publishing, website, or application features where they add complexity. Establish a minimal desktop build before integrating the calculation engine.

## Integration boundary

The editor should communicate with the Rust core through a narrow API. Initially this can be a local process or native binding; choose the bridge after the editor shell and packaging target are verified. The API should accept source text and return structured tokens, diagnostics, symbols, formatting edits, and calculation results. It should not expose UI-specific types from the old C# application.

## Compatibility and safety

- Maintain source text exactly unless the user invokes a formatting action.
- Represent edits as ranges where practical, so the editor can apply them without replacing the whole document.
- Keep calculation execution local by default.
- Do not execute arbitrary shell commands or general-purpose code from document content.
- Gate future MCP operations by explicit tool scope and project-root permissions.

## Acceptance criteria

- A Markdown file opens and saves without content loss.
- The editor is a continuous single-pane surface, with source mode available as a fallback.
- A `calcpad` fence is visually distinguishable and can show parser diagnostics.
- Ordinary code fences are never evaluated.
- Reused MarkText code retains its license notices.
- A clean checkout can build the minimal desktop editor before any Calcpad compatibility claim is made.
