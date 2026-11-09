# Fixed multi-column worksheet layout contract

**Status:** accepted product/layout decision; implementation not yet claimed  
**Owner:** Khwarizmi Core  
**Editor adapter:** AFFiNE / BlockSuite  
**Scope:** page-flow columns for the engineering worksheet; no free positioning

## 1. Decision

Khwarizmi uses a page-oriented, ordered-flow worksheet. A user may insert a multi-column container into the page flow and place ordinary or engineering blocks inside its columns.

This is a structured layout feature, not an infinite canvas. Blocks cannot be placed at arbitrary page coordinates, overlap, float freely, or acquire z-order. The editor presents columns as a side-by-side group and manages their contents as normal vertical block flows.

## 2. User-facing behavior

- Insert a columns container from the block insert menu.
- Choose one, two, three, or four columns.
- Add, edit, reorder, and delete blocks inside each column.
- Drag a block within a column or move it to another column.
- Resize column proportions with a divider, subject to minimum widths.
- Change the column gap.
- Convert a one-column container back to normal page flow.
- On narrow viewports, render columns stacked without changing the saved layout.

The initial useful preset is two equal columns. Three- and four-column layouts are supported by the document contract, but should not be prioritized over reliable two-column editing.

## 3. Canonical document representation

A page contains ordered flow regions. A region is either:

- `flow`: an ordered list of block IDs in one vertical stream; or
- `columns`: an ordered list of column records, each containing an ordered list of block IDs.

A columns region stores:

- stable `regionId`;
- `kind: "columns"`;
- stable `columnId` for each column;
- `blockIds[]` for each column;
- `widthBasisPoints` per column, with all widths summing to 10,000;
- `gapUm`, an integer micrometre gap;
- `flowOrder`, its position among page regions.

Constraints:

- Column count is 1–4; one column is normalized to ordinary flow.
- Each column width is at least 1,500 basis points (15%).
- Widths sum exactly to 10,000 basis points.
- Default two-column widths are 5,000 / 5,000.
- Default gap is 6,000 µm.
- IDs are stable across reordering, resizing, save/reload, and migration.
- Nested columns containers are not supported in v1.
- Rendered x/y/width/height are derived layout output, not authored coordinates.

This record is illustrative of the v1 model contract; exact JSON Schema field names and ID syntax remain part of schema publication.

## 4. Layout and pagination rules

- Regions flow from top to bottom in explicit page order.
- A columns region starts all of its columns at the same top edge.
- Each column lays out its own blocks vertically in its own order.
- The region's height is determined by its tallest column; the following region starts after that height.
- Blocks are not split internally by the column layout engine.
- When a columns region reaches a page boundary, pagination advances the region as a coordinated row; the layout engine preserves each column's block order and IDs.
- The layout engine derives column widths from the available page content width after margins and gaps.
- Resizing the application window or changing zoom may alter rendered pixel geometry, but never authored physical page geometry or column proportions.

A later pagination implementation may improve balancing or fragmentation, but MUST preserve the same authored order and must not introduce free positioning.

## 5. BlockSuite implementation boundary

Implement this as a Khwarizmi-owned custom container block (proposed type `kz:columns`) with child column containers (proposed type `kz:column`) registered through supported BlockSuite extension APIs.

BlockSuite owns:
- rendering the container and column views;
- drag handles, drop indicators, selection, focus, and keyboard navigation;
- resize-divider interaction;
- editor undo/redo and transaction mechanics.

Khwarizmi Core owns:
- stable region, column, and block identities;
- column membership and authored order;
- width/gap constraints and validation;
- persistence and migrations;
- the separation between visual order and calculation order.

All mutations pass through typed Khwarizmi commands, such as `InsertColumnsRegion`, `MoveBlockToColumn`, `SetColumnWidths`, `SetColumnGap`, and `ReorderBlocksInColumn`. A single user gesture is one atomic undoable transaction. The implementation MUST use public BlockSuite APIs and MUST NOT patch BlockSuite internals.

## 6. Calculation semantics

Columns are presentation only:

- A block's visual column does not create a variable namespace or local scope.
- Moving a calculation block between columns MUST NOT change calculation order.
- Reordering blocks within a column MUST NOT change calculation order.
- Only an explicit calculation-order command may change execution order.
- A visual layout edit may create a new authored document revision, but it must not invalidate or rewrite calculation meaning unless the edited content or an explicit semantic dependency changes.
- Stable block IDs, symbol IDs, assignment IDs, and evaluation history survive column moves and width changes.

## 7. Accessibility and interaction

- Every operation available by drag-and-drop also has a keyboard-accessible command.
- Focus order follows page flow, then column order, then block order.
- Screen-reader labels identify the column number and position.
- Resize controls expose keyboard increments and the current proportion.
- A block cannot be dropped into an invalid target; rejected moves leave the document unchanged.

## 8. Required tests

1. Create a two-column region, save, reload, and compare canonical document content.
2. Create three- and four-column regions and validate width constraints.
3. Reject widths that do not sum to 10,000 or violate the minimum.
4. Move a block between columns and preserve all stable IDs.
5. Reorder blocks visually and prove calculation order and evaluation results are unchanged.
6. Resize column widths and gap without changing calculation semantics.
7. Undo/redo insert, move, reorder, and resize operations atomically.
8. Render side-by-side at desktop width and stacked at narrow width without mutating authored data.
9. Paginate a columns region while preserving per-column order.
10. Verify keyboard-only movement, resizing, and accessible focus order.
11. Verify that no persisted arbitrary x/y/z or overlap fields are emitted.
12. Run save/reload and layout conformance on the mandatory Windows desktop target.

## 9. Explicit non-goals

- Free positioning or drag-anywhere page editing.
- Overlapping blocks, z-order, floating regions, or infinite-canvas worksheet authoring.
- Nested columns in v1.
- Calculation scope derived from columns or visual position.
- A general-purpose page-layout or desktop-publishing system.
