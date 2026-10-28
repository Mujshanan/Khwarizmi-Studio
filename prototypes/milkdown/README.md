# Khwarizmi Studio — Milkdown prototype

An isolated, MIT-compatible prototype for evaluating Milkdown/Crepe as a custom single-pane Markdown editor. It does not replace the MarkText submodule or modify the root project's license.

## Run locally

Requires Node.js 20 or newer and npm.

```sh
npm install
npm run dev
```

Open the local URL printed by Vite. To validate a production bundle:

```sh
npm run build
npm run preview
```

## Prototype scope

- Continuous WYSIWYG Markdown editing using Milkdown Crepe.
- KaTeX-backed inline/display LaTeX via Crepe's built-in feature.
- Open a local Markdown/text file and export the current document as `.md`.
- Switch between visual editing and raw Markdown source.
- Count explicit `calcpad` fences and show a clear, non-executing engine placeholder.
- Responsive layout with a small calculation inspector.

## Important limitations

- The calculation panel is only a placeholder; it does not invoke Rust or evaluate document content.
- Markdown round-tripping through a rich-text editor may normalize unsupported or unusual source constructs. Verify source fidelity against a fixture corpus before considering replacement of MarkText.
- Browser file access is user-initiated only. Save downloads a Markdown file; it does not overwrite an arbitrary path silently.
- This is a prototype, not a signed or packaged desktop application.

## License

The prototype code is MIT-licensed under the parent project. Milkdown/Crepe is MIT-licensed; its full dependency tree and any packaged assets must still be audited and notices included before distribution. No Taleno application code or assets are used.
