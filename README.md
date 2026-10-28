# Khwarizmi Studio

Khwarizmi Studio is a Markdown-first engineering-calculation IDE. It combines a MarkText-inspired single-pane editor with a Rust parser, formatter, and calculation core informed by Calcpad's worksheet behavior.

## Product direction

- **Markdown-first editor:** prose, tables, figures, typeset equations, and calculation blocks live in one continuous document surface.
- **Rust calculation core:** reimplement the required parser, formatter, semantic analysis, and calculation behavior in Rust. Do not import CalcpadCE's WPF or ASP.NET application UI.
- **Explicit calculation blocks:** a \`calcpad\` Markdown fence invokes the calculation engine. Ordinary code fences and typesetting-only math are not evaluated.
- **Autocomplete and diagnostics:** completion for symbols, units, functions, and snippets; source-linked syntax and calculation errors.
- **PDF publishing:** a separate renderer for page templates, equations, figures, tables, numbering, headers/footers, and references.
- **MCP:** local, permission-scoped tools for project discovery, validation, controlled calculation runs, and PDF export.

## Editor source

The MarkText upstream repository is connected at \`vendor/marktext\` as a Git submodule, pinned to a specific upstream commit. This brings the real editor source into a recursive checkout without copying the entire source tree into Khwarizmi's own commits. A repeatable PowerShell patch now changes installer metadata, executable identity, and startup branding; this is an initial adaptation, not a full rebrand of all editor UI strings.

Clone with the submodule:

\`\`\`bash
git clone --recurse-submodules https://github.com/Mujshanan/Khwarizmi-Studio.git
cd Khwarizmi-Studio
\`\`\`

On Windows, run the included launcher from PowerShell. It initializes the submodule if needed, installs dependencies on first run, and starts the upstream editor:

\`\`\`powershell
.\\scripts\\run-editor.ps1
\`\`\`

Requirements: Node.js 24 or newer, pnpm 10 or newer, Python 3.12, and Visual Studio 2022 Build Tools with the C++ workload for native dependencies. You can also run `pnpm install` and `pnpm run dev` directly from `vendor/marktext`.

For a repeatable Windows build, use the **Actions → Khwarizmi Studio Windows build** workflow. It checks out the submodule, installs dependencies, runs unit tests, builds the Windows x64 installer, and uploads the installer/ZIP as a 14-day workflow artifact. The workflow is configured to run on relevant pushes and pull requests; it can also be started manually from the Actions tab. The workflow applies the tracked Khwarizmi branding patch before building; the editor UI still contains upstream MarkText strings. For platform-specific build details, see [MarkText's build instructions](https://github.com/marktext/marktext/blob/develop/packages/website/content/docs/dev/BUILD.md).

MarkText's repository is MIT-licensed. Keep its license and copyright notices with the submodule and audit dependency and asset licenses before redistribution. See [the editor reuse and licensing decision](docs/architecture/editor-base-and-licensing.md).

## Document model

Markdown remains the canonical source. The proposed \`calcpad\` fenced block is the boundary between regular Markdown and evaluated engineering calculations. Inline and display math (\`$...$\`, \`$$...$$\`) are initially typesetting-only. Evaluation results and diagnostics are separate from the source and must never silently overwrite it.

See [Markdown calculation blocks](docs/architecture/markdown-calculations.md).

## Architecture

- \`vendor/marktext\`: upstream MarkText editor source, linked as a Git submodule.
- \`crates/calcpad-syntax\`: Rust lexical/syntax foundation and conservative formatter (initial scaffold; not full language parity).
- \`crates/calcpad-engine\` (planned): semantic analysis, evaluation, units, and calculation result model.
- \`apps/desktop\` (planned): Khwarizmi integration/adaptation of the MarkText desktop editor.
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

- MarkText is now linked as a submodule at \`vendor/marktext\`; it is not copied into the Khwarizmi Git history as ordinary files.
- The Rust syntax crate has an initial lossless lexer, byte spans, a conservative expression formatter, and a lossless detector for explicit Markdown `calcpad` fenced blocks, with unit tests.
- The \`calcpad\` Markdown fence is a proposed syntax documented for implementation; the parser does not yet evaluate it.
- A Windows build/test workflow has been added, but its first run must complete successfully before we can claim the editor builds. The current Rust crate is a scaffold, and MarkText integration is not yet verified.

## Upstream references

- [CalcpadCE](https://github.com/imartincei/CalcpadCE) — reference behavior and source for targeted compatibility research.
- [MarkText](https://github.com/marktext/marktext) — Markdown editor base.

## License

The project should retain its own license and third-party notices consistent with the code actually included. CalcpadCE and MarkText are MIT-licensed; retain their required notices for any reused code and audit all dependencies before distributing binaries.
