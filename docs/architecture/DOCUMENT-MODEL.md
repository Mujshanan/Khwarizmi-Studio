# Engineering document and worksheet model

**Status:** v1 proposal for schema review  
**Owner:** Khwarizmi Core  
**Not owned by:** BlockSuite internals or the renderer's transient state

## 1. Design goals

- Persist a durable, versioned engineering worksheet.
- Preserve authored material independently of calculation outputs.
- Support page-oriented A4 and custom-size documents.
- Support text, equations, variables, input data, tables, diagrams, references, and result regions.
- Support ordered page-flow regions and fixed one-to-four-column containers for side-by-side input/expression/result layouts; arbitrary free positioning is out of scope.
- Keep stable IDs across reorder, editing, save/reload, and migration.
- Make document migrations explicit and testable.

## 2. Document envelope

The native format is a JSON document (proposed extension: `.mpad.json`). The envelope contains:

- `format`: stable format identifier, `khwarizmi.document`;
- `schemaVersion`: integer schema version;
- `documentId`: stable UUID;
- `metadata`: title, authoring metadata, created/modified timestamps;
- `settings`: locale, display preferences, default unit system, page defaults;
- `pages[]`: ordered page records;
- `assets[]`: content-addressed references to embedded images/attachments;
- `extensions`: namespaced forward-compatible extension data.

Evaluation output is not authored content. It is stored separately as a derived snapshot keyed by document revision and calculation-engine contract/version. A stale snapshot MUST be detectable and MUST NOT silently replace authored values.

## 3. Page model and coordinates

Each page has a stable `pageId`, physical width and height, orientation, margins, and ordered flow regions.

**Accepted canonical geometry:** page dimensions, margins, and explicit layout gaps are integer micrometres (`um`). Use a page-local top-left origin; +X points right and +Y points down. Support named standard page presets and custom dimensions. Convert derived layout bounds to CSS pixels only at the rendering boundary. Zoom and device-pixel ratio never change persisted physical page geometry.

A region has a stable `regionId`, a `flowOrder`, and one of two v1 kinds:
- `flow`: an ordered `childBlockIds[]` vertical stream;
- `columns`: an ordered `columns[]` collection. Each column has a stable `columnId`, ordered `childBlockIds[]`, and `widthBasisPoints`; the region also stores `gapUm`.

Column count is 1–4 (one column normalizes to `flow`). Widths are integer basis points, sum exactly to 10,000, and are each at least 1,500. The default two-column layout is 5,000/5,000 with a 6,000 µm gap. Rendered region and column bounds are derived from page content width, margins, widths, and gaps. Arbitrary region `x/y`, z-order, overlap, and free positioning are not part of v1 authored data. Nested columns are deferred.

The layout engine MUST produce finite bounds within the page's content area or a deterministic pagination/overflow diagnostic. Renderer zoom/pan is view state and MUST NOT be persisted as page geometry.

## 4. Block model

Every block has a stable `blockId`, a `type`, a `contentVersion`, and authored `content`. Initial logical block types:

- `text`: rich text and headings;
- `math`: canonical expression AST plus optional display/editor interchange data;
- `variable`: explicit variable declaration/assignment;
- `input`: user-supplied scalar quantity input (structured/vector/matrix inputs are unsupported in v1);
- `table`: typed cells and optional calculated bindings;
- `diagram`: asset or structured diagram reference;
- `reference`: citation/standard/source metadata;
- `result`: reference to a calculated expression or named result;
- `group`: semantic grouping of child blocks.

A block's display style is not its semantic type. A result region cannot redefine a variable by virtue of being visually placed after another block.

## 5. Ordering and identity

Page order, region order, and block order are explicit. Executable statements also carry an explicit document-wide calculation order that is independent of visual page/region placement; moving a block visually MUST NOT change evaluation meaning. IDs are immutable once persisted. Reordering changes ordering metadata, not identity. References target stable IDs or explicit symbol IDs, never array indices or display labels.

## 6. Persistence and migration

- Schema versions are integer and monotonically increasing.
- Readers MUST reject unknown major versions with a clear compatibility error.
- Migrations are deterministic, version-to-version functions.
- Migrations MUST preserve stable IDs and authored values.
- Evaluation snapshots are invalidated when authored calculation inputs or semantic schema versions change.
- Unknown namespaced extension data SHOULD be preserved through read/write round trips.
- Save/load round-trip tests MUST compare canonicalized authored documents.

## 7. BlockSuite adapter boundary

BlockSuite may represent and edit visual blocks, selection, and ordering. An adapter maps BlockSuite transactions into Khwarizmi document commands. The adapter MUST:

- validate incoming commands against the Khwarizmi schema;
- preserve stable IDs;
- avoid storing calculation truth only in editor-specific properties;
- keep editor undo/redo synchronized with document commands;
- serialize the canonical Khwarizmi document, not a raw BlockSuite snapshot as the sole source of truth.

## 8. Accepted decisions and remaining schema work

Canonical page geometry is accepted as integer micrometres, top-left origin, +X right, +Y down, with standard presets and custom dimensions. V1 layout is ordered page flow with fixed one-to-four-column containers; it does not persist arbitrary region coordinates or support free positioning. Scalar-only v1 values and assignment/scope rules are defined in CALCULATION-SEMANTICS.md and DECISIONS.md.

The implementation team must still encode the accepted layout modes and asset policy in exact schema fields, define block content-version rules and stable ID syntax, and provide deterministic migration fixtures. V1 assets are embedded only with SHA-256 identity and the limits recorded in PR-3-DECISION-RESOLUTION.md. No remote asset is fetched implicitly during validation or evaluation.
