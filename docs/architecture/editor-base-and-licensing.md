# Editor base and licensing strategy

## Decision

Khwarizmi Studio must remain MIT-licensed. Do not adopt or copy Taleno's application code because Taleno is licensed under AGPL-3.0-only and that license is incompatible with the project's required MIT-only distribution strategy.

Taleno may be evaluated as a reference for interaction design and architecture. Its license does not transfer to independent implementations of general ideas, but do not copy its source, assets, or other protectable material. Consult legal counsel for any borderline reuse question.

Primary references:
- Taleno source and license: https://github.com/BerryUIKI/Taleno
- Milkdown source and license: https://github.com/Milkdown/milkdown
- MarkText source and license: https://github.com/marktext/marktext

## MIT-compatible editor options

Evaluate these in order:

1. **Milkdown** as an editor framework, subject to verifying the exact version and all dependencies. Build a Khwarizmi-owned desktop shell and single-pane experience around it rather than forking Taleno.
2. **MarkText** as the lower-switching-cost fallback, since its upstream project is MIT-licensed and a Windows build already succeeded in this repository.
3. Other editor frameworks only after checking their actual code, assets, and dependency licenses.

Do not change Khwarizmi Studio's license from MIT. Do not copy Taleno's AGPL-licensed application code into this repository.

## Required compliance work

Before copying files or shipping a binary:

1. Preserve copyright and license notices for any reused MIT code.
2. Track reused files, upstream versions, and modifications in a third-party notices document.
3. Audit transitive npm/Rust dependencies, fonts, icons, themes, sample documents, and bundled runtime assets separately.
4. Keep Calcpad-derived code and behavior traceable. If any Calcpad source is copied, preserve its notices; a clean-room Rust reimplementation should use behavior tests without copying implementation text.
5. Generate a third-party license report as part of release packaging.

## Integration boundary

The editor should communicate with the Rust calculation core through a narrow API. The API should accept source text and return structured tokens, diagnostics, symbols, formatting edits, and calculation results. It must not expose UI-specific types from the old C# application.

## Compatibility and safety

- Maintain Markdown source faithfully unless the user invokes a formatting action.
- Represent edits as ranges where practical, so the editor can apply them without replacing the whole document.
- Keep calculation execution local by default.
- Do not execute arbitrary shell commands or general-purpose code from document content.
- Gate future MCP operations by explicit tool scope and project-root permissions.

## Acceptance criteria

- A Markdown file opens and saves without content loss.
- The editor is a continuous single-pane surface, with source mode available as a fallback.
- A `calcpad` fence is visually distinguishable and can show parser diagnostics.
- Ordinary code fences are never evaluated.
- Reused code retains its original license notices.
- A clean checkout can build the minimal desktop editor before any Calcpad compatibility claim is made.
