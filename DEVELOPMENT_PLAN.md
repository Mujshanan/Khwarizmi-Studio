# Khwarizmi Studio — Development Plan

This roadmap describes intended product direction and sequencing. Planned items are not claims of implemented functionality.

## Foundation

Khwarizmi Studio retains the useful **AFFiNE-derived React/TypeScript and BlockSuite frontend packages** as its editor/application foundation. **Tauri 2 is the selected desktop host**, replacing the inherited Electron desktop runtime. This does not commit the project to retaining every inherited AFFiNE desktop capability. Preserve upstream licenses and notices.

## Product outcome

A visual engineering worksheet where engineers define inputs, write equations, see symbolic and numerical substitutions, reuse intermediate results, track units, explain assumptions, and produce reviewable technical documents.

## Phase 0 — Repository and upstream baseline

- [x] Establish an AFFiNE-derived repository baseline.
- [x] Document the AFFiNE and BlockSuite foundation.
- [x] Identify the AFFiNE baseline commit and document inherited repository/toolchain structure.
- [ ] Complete the nested license/notice inventory.
- [ ] Verify a clean Windows install, build, and unmodified desktop launch.
- [ ] Complete Windows ZIP/Squirrel/NSIS install-launch-uninstall smoke tests (PR #1 remains blocked).
- [ ] Establish a routine for reviewing upstream changes.

Baseline evidence and remaining gates: [Repository baseline](./docs/BASELINE.md).

**Exit criteria:** reproducible baseline, clear provenance, documented build steps, and separation of inherited versus Khwarizmi-owned code. The audit is documented; clean runtime and packaging validation remain open.

## Phase 1 — Architecture and engineering document contracts

- [x] Record accepted architecture direction and responsibility boundaries.
- [x] Draft the versioned worksheet/document model and page geometry contract.
- [x] Draft canonical AST, variable, dependency, substitution, unit, and error semantics.
- [ ] Reconcile legacy TypeScript/CortexJS calculation wording with the approved CalcpadCE calculation-host boundary; defer the Python worker from v1.
- [x] Specify the initial JSON-RPC 2.0 named-pipe topology and framing proposal.
- [x] Resolve owner decisions A1–A9 and select the T1–T6 implementation contract.
- [ ] Publish and freeze versioned JSON schemas.
- [ ] Implement language-neutral contract fixtures and executable conformance tests.
- [ ] Verify RPC transport/security behavior and cross-implementation conformance.

Architecture documents: [Contract index](./docs/architecture/README.md).

**Exit criteria:** approved normative schemas, stable fixtures, passing contract tests, and verified transport/security behavior before broad editor or production calculation-host implementation. Owner-level decisions are resolved; executable contracts and verification remain outstanding.

## Phase 2 — Worksheet interaction foundation

- [ ] Establish a page-oriented worksheet experience on BlockSuite.
- [ ] Support fixed page-flow layout with one to four columns; do not introduce arbitrary/free positioning.
- [ ] Define selection, editing, keyboard, and navigation behavior.
- [ ] Add equation, text, table, and result content blocks.
- [ ] Support side-by-side input/expression and result presentation.
- [ ] Preserve document structure through save, reload, and migration.

**Exit criteria:** users can create, arrange, save, reopen, and edit a representative worksheet.

## Phase 3 — Calculation engine and traceability

- [ ] Implement the calculation-host process and RPC v1.
- [ ] Implement expression parsing and evaluation behind the contract.
- [ ] Resolve variables and references to earlier definitions/results.
- [ ] Show expressions, substitutions, and evaluated results.
- [ ] Support reusable intermediate results and dependency updates.
- [ ] Implement units, conversions, and dimensional validation.
- [ ] Surface calculation errors and invalid states clearly.
- [ ] Build deterministic contract and regression tests.

**Exit criteria:** representative calculations evaluate reproducibly with visible substitutions, reusable results, units, and meaningful errors.

## Phase 4 — Engineering authoring and verification

- [ ] Create representative structural-engineering worksheet examples.
- [ ] Document assumptions, inputs, references, and conclusions.
- [ ] Add verified calculation modules only with explicit code edition and scope.
- [ ] Build benchmark examples and independently checked expected results.
- [ ] Add regression tests for edge cases, units, and numerical tolerances.
- [ ] Distinguish verified functionality from experimental features.

**Exit criteria:** each released engineering capability has documented scope, assumptions, and independent validation evidence.

## Phase 5 — Document output and release readiness

- [ ] Define paginated A4 layout and print behavior.
- [ ] Add PDF export for equations, tables, diagrams, and page numbering.
- [ ] Verify export fidelity against on-screen worksheets.
- [ ] Establish Windows packaging and installation checks.
- [ ] Add backup, recovery, and document compatibility tests.
- [ ] Publish release notes, limitations, and migration guidance.

**Exit criteria:** stable authoring-to-review-to-PDF workflow, repeatable installation, and documented limitations.

## T1 — Electron-to-Tauri 2 migration inventory

T1 established the initial source-backed migration inventory. The selected desktop target is **Tauri 2**, replacing Electron because avoiding Electron packaging and maintenance complexity is the primary motivation. T1 does not repair Electron packaging, remove Electron dependencies, or rewrite the Electron runtime. See [Desktop platform migration inventory](./docs/architecture/DESKTOP-PLATFORM-MIGRATION.md).

**T1 verification status: not yet closed.** The live recursive Git tree was inspected (12,828 paths), the Electron host/package surface and the renderer API package dependency were identified, and the native Rust workspace manifests were reviewed. The connector's code-search endpoint returned incomplete/empty results, so a complete symbol-level scan of all renderer consumers cannot be verified from this session. The exact consumer map, final inherited-feature dispositions, and Windows-targeted native-link feasibility remain T1 closure gates. Do not mark these checks complete based on path inventory alone.

Windows x64 remains mandatory. Retain the AFFiNE-derived React/TypeScript and BlockSuite frontend packages, use Tauri 2 as the desktop host, preserve the approved CalcpadCE calculation-engine boundary, and preserve fixed one-to-four-column page-flow layout. Do not change existing PR dependencies or silently retire inherited features.

## Next development milestone — T2: Framework-neutral desktop host contract

T2 defines the typed, framework-neutral boundary between the renderer and desktop capabilities/services. The first draft is [Desktop host contract](./docs/architecture/DESKTOP-HOST-CONTRACT.md). It is a contract-design artifact only: no Tauri runtime, Electron rewrite, packaging change, dependency deletion, or calculation-engine implementation is included. T2 must reconcile the API consumer audit and product-scope decisions before freezing service methods.

### Earlier contract milestone reference — PR #3

The earlier PR #3 contract-freeze milestone remains a relevant contract/conformance workstream, but is no longer the next desktop-development milestone. Its acceptance criteria and existing status are retained below for traceability; this T1 roadmap update does not claim that the PR #3 requirements are complete.

1. Publish versioned document, expression, evaluation, diagnostic, and RPC JSON Schemas from the accepted ADRs.
2. Add language-neutral fixtures with stable IDs and independently reviewed expected outcomes.
3. Implement TypeScript validators and deterministic schema/semantic/unit tests.
4. Test RPC framing independently and add fake-host integration coverage.
5. Run the contract suite in CI without Electron or the production calculation host.
6. Defer independent second-implementation interoperability and Windows named-pipe security verification to their separately tracked milestones.

PR #1's Windows packaging and launch/install/uninstall verification remains a separate release-readiness track. PR #2's accepted architecture contracts are merged into `canary`. PR #3 records the resolved decisions and implementation contract. Its merge does not claim executable conformance is complete. Do not start broad worksheet UI or production calculation-host implementation until PR #3's acceptance criteria pass.

## Foundation-stage non-goals

- Replacing AFFiNE or BlockSuite wholesale without evidence that they block a required capability.
- Broad UI implementation before document and calculation contracts are reviewed.
- Claiming structural-code compliance before independent validation.
- Treating a successful desktop build as evidence that engineering calculations are correct.

## Status conventions

- [x] Completed and verified.
- [ ] Planned or not yet verified as complete.

Update this roadmap as decisions are approved and work is delivered. Link completed phases to implementation PRs, tests, and architecture records.
