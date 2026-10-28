# Khwarizmi Studio

Khwarizmi Studio is a planned desktop engineering-calculation IDE built around the open-source [CalcpadCE](https://github.com/imartincei/CalcpadCE) calculation engine.

## Goals

- **Calculation editor:** Calcpad syntax highlighting, context-aware autocomplete, snippets, diagnostics, and symbol information.
- **Native Markdown:** editable Markdown documents alongside calculation worksheets, with live preview, math, tables, images, and references.
- **PDF publishing:** a replaceable PDF renderer and configurable engineering report templates, including page size, margins, title page, headers/footers, page numbering, and typography.
- **MCP server:** opt-in tools for project discovery, worksheet inspection, controlled calculation execution, validation, and report export.

## Architecture principles

1. Keep the Calcpad calculation engine as the source of calculation semantics; do not reimplement its parser or numerical behavior in the IDE.
2. Separate the editor UI, document/project model, calculation adapter, PDF publishing pipeline, and MCP transport.
3. Preserve compatibility with existing Calcpad worksheets wherever possible.
4. Keep MCP operations explicit and permission-scoped. File writes and execution must be constrained to the opened project and never silently overwrite user work.
5. Build features in small, testable steps and retain a reproducible baseline before changing the upstream engine.

## Planned milestones

1. **Upstream baseline:** import/fork CalcpadCE, record upstream commit and license notices, build and run existing tests.
2. **IDE foundation:** project/workspace model, editor panels, file operations, and baseline UI.
3. **Autocomplete:** syntax-aware completions, snippets, context help, and diagnostics based on the actual Calcpad language.
4. **Markdown:** native `.md` editing and preview, with a documented strategy for linking Markdown to calculation worksheets.
5. **PDF publishing:** renderer abstraction, print CSS/template system, regression fixtures, and export tests.
6. **MCP:** local stdio server first, explicit tool schemas, workspace sandboxing, and integration tests.
7. **Packaging:** Windows development build, installer, update strategy, and user documentation.

## Status

This repository is currently the project destination. The upstream CalcpadCE source has not yet been imported. The initial implementation should not be considered buildable until the upstream baseline and build/test instructions are verified.

## Upstream

- [CalcpadCE repository](https://github.com/imartincei/CalcpadCE)
- [CalcpadCE documentation](https://imartincei.github.io/CalcpadCE/)

## License

CalcpadCE is MIT-licensed. When its source is imported, preserve upstream license and copyright notices and include notices for any additional dependencies. Confirm the exact license and third-party notices before distributing packaged builds.
