# Desktop platform migration inventory — Electron to Tauri 2

**Status:** T1 — migration inventory in progress; static repository inspection recorded  
**Target desktop framework:** Tauri 2  
**Current desktop framework:** Electron  
**Primary motivation:** avoid Electron packaging and maintenance complexity  
**Target platform:** Windows x64 is mandatory  
**Implementation status:** not started  
**Scope boundary:** inventory and migration planning only. This document does not authorize or perform Electron packaging repairs, Electron dependency removal, runtime rewrites, or release changes.

## 1. Purpose and decision

The approved direction is to replace the Electron desktop host with Tauri 2. The migration is a desktop-host replacement, not a wholesale rewrite of the frontend, editor, document model, or calculation system.

The intended separation is:

- React and TypeScript remain responsible for the frontend.
- BlockSuite remains the editor foundation, subject to the existing integration contract.
- MathLive and MathJSON remain the mathematical authoring/expression technologies.
- Khwarizmi owns engineering document semantics, calculation order, variable/version semantics, units, provenance, and evaluation snapshots.
- The calculation service remains independent of the desktop shell and must retain the approved CalcpadCE boundary.
- Tauri 2/Rust replaces Electron as the native desktop host.
- Windows x64 installation, launch, update, recovery, and uninstall remain mandatory release gates.

The fixed page-flow worksheet layout is unchanged: one to four columns, no arbitrary/free positioning. Desktop migration must not change document meaning or calculation order.

## 2. Inspection baseline and confidence

This inventory is based on the live `canary` branch inspected for T1, including:

- `packages/frontend/apps/electron/package.json`
- Electron main, preload, helper, worker, window-manager, updater, protocol, security, storage, and packaging source files listed below
- Root `package.json` and `DEVELOPMENT_PLAN.md`
- Current calculation and document architecture documents
- Current open pull-request list and branch list

The inspected `canary` head reported by repository commit search is `7e85e7324dfcb533b8555eb8476cc376a99c51ff`. The repository search returned 13 open PRs and no open issues at inspection time.

**Confidence limitation:** this is a source-backed inventory of the identified Electron application package and its principal integration points. GitHub code search did not return usable repository-wide import matches in this inspection environment. Before implementation, a local full-repository import/reference scan is required to find renderer-side globals, package aliases, build-time defines, scripts, tests, and generated artifacts that refer to Electron. Do not treat this document as proof that every Electron reference in the monorepo has been enumerated.

## 3. Current desktop package and build topology

The existing desktop package is `packages/frontend/apps/electron`, package name `@affine/electron`, with description `AFFiNE Desktop App`.

Its scripts include:

- `start: electron .`
- `dev: r ./scripts/dev.ts`
- `build: cross-env NODE_ENV=production r ./scripts/build-layers.ts`
- `package: electron-forge package`
- `make: electron-forge make`
- `make-squirrel`, `make-nsis`, and Windows signing scripts

The current build-layer script uses esbuild to bundle the main, preload, and helper entry points and explicitly externalizes Electron runtime packages. Electron Forge then packages the application and invokes makers/plugins. This is a separate build topology from the frontend's existing build and must be mapped explicitly into Tauri's frontend build/dev URL and Rust build lifecycle.

### Electron-specific package dependencies

Direct desktop/runtime and packaging dependencies identified in the package manifest include:

- `electron`
- `electron-updater`
- `electron-log`
- `electron-squirrel-startup`
- `electron-window-state`
- `@sentry/electron`
- `@electron-forge/*` CLI, core, makers, plugins, and shared types
- `@electron/fuses` and Electron signing/notarization packages
- `electron-winstaller` at the root dependency/override level
- `app-builder-lib`, `builder-util-runtime`, and Electron-oriented maker utilities

Other dependencies in the desktop package are not automatically removable: `async-call-rpc`, `rxjs`, `yjs`, `@affine/native`, `@affine/nbstore`, `@toeverything/infra`, `link-preview-js`, and related packages may support application or domain behavior independently of Electron. Their use must be classified by import graph and ownership, not by package location alone.

## 4. Desktop API and process-boundary inventory

| Area | Current source evidence | Current responsibility | Tauri migration disposition |
|---|---|---|---|
| Application lifecycle | `src/main/index.ts` | Startup, readiness, quit, single-instance lock, activation, protocol setup, handler/event registration, updater and telemetry startup | Replace with Rust/Tauri lifecycle. Preserve only required behaviors. |
| Preload bootstrap | `src/preload/bootstrap.ts`, `src/preload/index.ts` | Conditional exposure of app metadata, APIs, events, shared storage, and worker APIs into trusted renderer pages | Remove preload model; expose a typed, narrow frontend adapter backed by Tauri commands/events. |
| Main IPC API | `src/preload/electron-api.ts`, `src/main/exposed.ts`, `src/main/type.ts`, `src/shared/type.ts` | Dynamic namespace/method discovery, invoke handlers, event subscription/ref-counting, RPC proxy | Replace with explicit versioned command/event contracts. Do not mechanically translate every IPC channel without classifying it. |
| Helper process | `src/main/helper-process.ts`, `src/helper/index.ts`, `src/helper/exposed.ts` | Separate Node utility process; RPC bridge between renderer and helper; lifecycle/error handling | Reassess as a managed sidecar or service. Keep database/native service behavior out of the Rust host unless there is a justified rewrite. |
| Helper database/storage | `src/helper/nbstore/handlers.ts`, `src/helper/nbstore/index.ts` | `DocStoragePool`, document/blob/index/clock operations via `@affine/native` and `@affine/nbstore` | Preserve storage contracts and on-disk compatibility. Decide whether native Rust functionality can be called as Rust crates or remains a packaged service. |
| Disk synchronization | `src/helper/disk-sync/handlers.ts`, `src/helper/disk-sync/index.ts` | Native `DiskSync` sessions, serialized session operations, normalized events | Preserve session lifecycle, serialization, event ordering, and cancellation/recovery semantics. |
| Workspace services | `src/helper/workspace/index.ts` and `src/helper/workspace/*` | Local workspace listing, trash, backup, recovery, deletion | Carry forward only if retained in Khwarizmi product scope; otherwise mark as inherited functionality for retirement after product review. |
| Native dialogs/import | `src/helper/dialog/index.ts`, `src/preload/electron-api.ts` | Database file open/save and Markdown sync-folder selection; Electron `webUtils.getPathForFile` converts dropped files to local paths | Replace with Tauri dialog/filesystem APIs and a reviewed drag/drop path contract. Validate paths and permissions. |
| Preview rendering | `src/helper/preview/index.ts` | Mermaid and Typst SVG rendering via `@affine/native`; font discovery currently uses Electron resource paths | Preserve render request/result contracts; redesign resource/font path resolution for Tauri bundle layout. |
| Renderer worker windows | `src/main/worker/pool.ts`, `src/preload/worker.ts` | Hidden BrowserWindow worker instances, per-worker preload, helper connection and failure handling | High-risk redesign. Evaluate Web Workers, Rust-managed processes, or sidecars. Do not assume a Tauri webview window is equivalent to Electron's hidden BrowserWindow worker. |
| Main window | `src/main/windows-manager/main-window.ts`, `launcher.ts` | Window creation, restore state, lifecycle, close-to-tray/macOS dock behavior | Replace with Tauri window APIs where needed. Re-evaluate platform-specific behavior against Windows-first scope. |
| Tabs and embedded views | `src/main/windows-manager/tab-views.ts` | Electron `WebContentsView` tabs, multiple origins/views, view lifecycle, spellcheck and persisted tab metadata | Major compatibility risk. Determine whether Khwarizmi needs embedded web-content tabs at all. Prefer application-level React navigation where product requirements permit; do not assume direct WebContentsView parity. |
| Popups | `src/main/windows-manager/popup.ts` | Transparent, always-on-top notification/recording popup windows with animation and display positioning | Rebuild only required popup behavior with Tauri windows; test Windows positioning, transparency, focus and lifecycle. |
| Application menu | `src/main/application-menu/create.ts` | Native menu, keyboard accelerators, navigation, devtools, tab and worker actions | Replace with Tauri menu APIs or frontend menu where appropriate. |
| Tray | `src/main/tray/index.ts` | Tray icon/menu, recording and window visibility state | Use Tauri tray support only if tray behavior remains in product scope. |
| Custom protocol and request routing | `src/main/protocol.ts` | Custom internal scheme, static resource resolution, authenticated API proxying, local-file allowlists, origin mapping | Security-sensitive redesign. Separate static asset protocol from authenticated network proxy. Preserve strict canonical path validation and origin policy. |
| Deep links | `src/main/deep-link.ts` | AFFiNE protocol registration, authentication callbacks, second-instance URL handling, tab/window routing | Use Tauri deep-link support and single-instance handling. Preserve validated URL parsing and authentication flow only if retained. |
| Security restrictions | `src/main/security-restrictions.ts`, `src/main/security/*`, `src/main/web-preferences.ts` | Navigation allowlist, external URL policy, redirect validation, renderer-origin checks, web preferences | Re-express as Tauri capabilities, CSP, command validation and external-link policy. Re-review the threat model; do not assume a plugin grants safe access automatically. |
| Authentication/session storage | `src/main/auth/auth-session.ts` | Token broker, encrypted token persistence via Electron `safeStorage`, endpoint normalization and auth transport | Replace secure storage with a Windows-protected secret store (DPAPI-backed/keyring approach) and preserve atomic file mutation and endpoint isolation. Do not store secrets in renderer storage. |
| Shared state/cache | `src/main/shared-storage/storage.ts`, `src/preload/shared-storage.ts` | Persistent JSON state/cache plus renderer memory mirror, revision tracking, broadcasts, synchronous bootstrap read | Replace with a single authoritative host-side store and explicit async commands/events. Define ordering, conflict, startup-read and crash-consistency semantics before implementation. |
| App configuration | `src/main/config-storage/persist.ts`, `src/main/config.ts` | User-data path, JSON configuration, build channel and environment flags | Map data paths to Tauri app-data locations; retain compatible config schema and explicit channel policy. |
| Updater | `src/main/updater/*`, `src/main/handlers.ts`, `src/main/events.ts` | electron-updater, AFFiNE feed provider, Windows-specific updater, progress/events, install/restart | Replace with signed Tauri updater artifacts and an explicit update-channel design. Existing feed formats and channel behavior require compatibility review. |
| Logging/telemetry | `src/main/logger.ts`, `src/main/recording/*`, Sentry setup in main entry | Electron logger, Sentry Electron SDK, application telemetry | Select host-neutral logging and Sentry Rust/JS integrations. Avoid duplicate events and preserve privacy controls. |
| Recording/system capture | `src/main/recording/feature.ts`, `src/main/tray/index.ts` | OS capture permissions, native module integration, process/application monitoring, popup and tray coordination | Product-scope decision required. Do not migrate inherited meeting-recording functionality automatically. |
| Packaging | `forge.config.mjs`, `scripts/common.ts`, `scripts/build-layers.ts`, `scripts/make-nsis.ts`, `scripts/make-squirrel.ts`, signing scripts | Electron layer bundling, Forge hooks, resource trimming, makers, NSIS/Squirrel and signing | Replace with Tauri CLI/bundler and Windows signing workflow after the shell proof of concept. No packaging repair is part of T1. |

## 5. Existing Rust/native workspace reuse evidence

The repository already has a Rust workspace at the root. Its members include `packages/frontend/native`, `packages/frontend/native/nbstore`, and `packages/frontend/native/sqlite_v1`, alongside other native packages. This is important: a Tauri host would not necessarily need to launch a Node.js helper merely to reach the existing Rust storage implementation.

Source evidence:

- `packages/frontend/native/Cargo.toml` defines `affine_native` as both `cdylib` and `rlib`; it depends on `affine_nbstore`, `affine_sqlite_v1`, disk-sync, preview, and media-capture components.
- `packages/frontend/native/src/lib.rs` exposes disk-sync and preview modules and re-exports nbstore/sqlite APIs.
- `packages/frontend/native/nbstore/Cargo.toml` defines `affine_nbstore` as `cdylib` and `rlib` and declares a `use-as-lib` feature. Its `src/lib.rs` conditionally disables N-API wrapper exports when that feature is enabled, providing evidence of a library-use path.
- `packages/frontend/native/sqlite_v1/Cargo.toml` is also `cdylib` and `rlib`, but its current manifest does not declare an equivalent `use-as-lib` feature; direct Tauri use may need a small crate-boundary refactor.
- The `affine_native` package currently includes N-API build dependencies and is designed to be loaded by Node. It should not be assumed to be a ready-made Tauri plugin or direct Tauri dependency.
- The workspace dependency for `affine_preview` is version-based rather than a listed local workspace member; verify its actual source/feature availability before relying on direct linkage.

**Assessment:** direct Rust reuse is plausible for parts of the native storage stack, but not yet proven. T1 still needs a Windows-targeted `cargo check` feasibility spike on a separate future task, review of N-API-specific types/macros, Tokio runtime ownership, SQLx migrations, feature flags, native capture dependencies, and crate licensing. The current Electron helper remains the only inspected integration path for these services.

The repository paths checked for a prospective Tauri app (`packages/frontend/apps/tauri/package.json`, `packages/frontend/apps/tauri/tauri.conf.json`, and `packages/frontend/apps/tauri/src-tauri/Cargo.toml`) do not currently exist on `canary`. No Tauri application scaffold was found at those expected paths.

## 6. Required product-scope classification

The current package is explicitly named and described as AFFiNE Desktop (`@affine/electron`, description `AFFiNE Desktop App`). Its source includes inherited workspace, sync, authentication, recording, tabbed web-content, preview and database capabilities.

A Tauri migration must not blindly reproduce the entire inherited desktop application. Each capability needs one of these dispositions before its implementation PR:

- **Retain:** required by Khwarizmi or required by the chosen integrated application foundation.
- **Adapt:** required behavior exists, but its Electron implementation must change.
- **Replace:** a different implementation is needed while preserving the public behavior/data contract.
- **Retire:** inherited functionality is not part of Khwarizmi's product scope. Requires explicit product-scope approval; T1 does not retire anything.
- **Unresolved:** evidence or product decision is insufficient.

Initial disposition:

| Capability | Initial disposition | Reason / decision needed |
|---|---|---|
| React/TypeScript frontend | Retain | Not Electron-specific. |
| BlockSuite editor | Retain | Existing editor foundation and integration work. |
| Khwarizmi document/calculation contracts | Retain | Framework-independent product semantics. |
| Main desktop lifecycle/window | Replace | Electron host responsibility. |
| Typed renderer API | Replace | IPC/preload implementation is Electron-specific. |
| Helper process and native database service | Adapt / unresolved | Separate service may remain appropriate; choose process model after dependency and native-crate audit. |
| Hidden worker BrowserWindows | Unresolved | Identify actual consumers and workload before selecting Web Worker or sidecar design. |
| AFFiNE embedded tabs/WebContentsView | Unresolved | May be unnecessary for a focused engineering worksheet application. |
| AFFiNE cloud/auth/deep-link behavior | Unresolved | Confirm whether the target product requires AFFiNE cloud integration. |
| Workspace sync and backups | Unresolved | Confirm local-only versus synced product requirements. |
| Meeting/desktop recording and tray | Unresolved | Appears inherited; no automatic migration commitment. |
| Mermaid/Typst preview | Retain capability; replace resource integration | Relevant rendering may be useful, but font/resource lookup is Electron-specific. |
| Native file open/save | Retain capability; replace API | Required desktop workflow. |
| Auto-update | Retain capability; replace implementation | Required release-management capability, subject to channel/signing decisions. |
| Sentry/logging | Adapt | Retain only approved telemetry and logging behavior. |

## 7. Tauri target mapping and unresolved design decisions

| Concern | Candidate Tauri mechanism | Must be decided/tested |
|---|---|---|
| Desktop lifecycle | Tauri Rust application setup/run | Single instance, startup arguments, orderly shutdown and crash recovery. |
| Frontend integration | Tauri v2 frontend build configuration | Integrate existing monorepo frontend build without duplicating or bypassing it. |
| Renderer API | Tauri commands/events with typed TypeScript wrapper | Command naming, schema validation, error mapping, payload limits and versioning. |
| Filesystem/dialogs | Tauri filesystem/dialog plugins or narrowly scoped Rust commands | Capability allowlists, document path permissions, import/export and atomic writes. |
| Windows secrets | Windows DPAPI-backed secure storage/keyring | Migration of existing Electron safeStorage ciphertext is not assumed portable; define re-authentication or explicit migration. |
| Background services | Rust-managed child process or Tauri sidecar | Keep calculation host and native database service isolated; define supervision and protocol. |
| RPC | Existing bounded RPC contract or a reviewed transport adapter | Preserve 16 MiB v1 framing limit, cancellation, request correlation, digest validation and error semantics where those contracts apply. |
| Worker execution | Web Worker, sidecar, or host-managed process | Determine actual worker needs and failure isolation; no one-to-one BrowserWindow mapping. |
| Custom schemes/assets | Tauri asset/custom protocol facilities | Preserve local path allowlists, origin separation and authenticated request handling. |
| Deep links/single instance | Tauri deep-link and single-instance support | Windows protocol registration and second-instance URL forwarding. |
| Windows menu/tray | Tauri menu/tray plugins | Retain only needed interactions. |
| Updates | Tauri updater | Signed artifacts, endpoint, channels, staged rollout, rollback and update failure recovery. |
| Packaging | Tauri bundler | NSIS primary, MSI secondary candidate; clean-machine install/uninstall tests. |
| Web runtime | WebView2 | Evergreen bootstrapper versus offline runtime; runtime presence, install failure and version testing. |
| Telemetry | Rust and frontend Sentry integrations or host-neutral telemetry | Release identity, crash coverage and privacy behavior. |

## 8. Cross-cutting invariants

The migration must preserve all of the following:

1. **Windows x64 is mandatory.** No migration stage is accepted solely on macOS/Linux or a development-only run.
2. **Calculation boundary:** retain the approved CalcpadCE calculation-engine boundary. Do not silently replace it with a different evaluator or make the Tauri Rust host the mathematical engine.
3. **Calculation semantics:** preserve sequential variable visibility, versioned reassignment, immutable evaluation snapshots, substitution provenance, units, diagnostics and deterministic behavior as approved.
4. **Worksheet layout:** fixed page-flow multi-column layout, one to four columns, with no arbitrary/free positioning. Preserve the approved PR #18 contract and dependency; do not adopt the older flexible-positioning wording in the current `canary` roadmap.
5. **Document compatibility:** no loss or silent rewriting of authored documents, stable IDs, revisions, snapshots, assets or migration history.
6. **Editor independence:** do not rewrite React, BlockSuite or MathLive as part of the desktop-host migration.
7. **Security:** no unrestricted shell, filesystem, process or secret access from the renderer. Tauri capabilities and Rust commands must follow least privilege.
8. **No silent feature retirement:** inherited AFFiNE features require explicit retain/adapt/retire decisions.
9. **No Electron work during T1:** do not repair Electron packaging, remove Electron dependencies, or change existing Electron runtime code as part of the inventory.

## 9. Documentation and decision conflicts to reconcile before implementation

The live `canary` documentation is not fully aligned with the user's later approved decisions and dependent PR work. This inventory records the conflicts; it does not rewrite those documents or reinterpret the decisions.

| Topic | Live `canary` evidence | Current approved direction / required action |
|---|---|---|
| Desktop host | `DEVELOPMENT_PLAN.md` and PR #12 describe AFFiNE as the host; current app package is AFFiNE Electron Desktop | Tauri 2 is the selected replacement desktop host. Clarify whether the AFFiNE-derived React app packages remain the frontend foundation. |
| Calculation evaluator | `CALCULATION-SEMANTICS.md` calls the v1 core TypeScript-authoritative; PR #5 describes MathJSON + CortexJS Compute Engine | CalcpadCE boundary is the approved target in the user's current project direction. Reconcile contract docs and PR dependencies before calculation-host implementation. |
| Layout | `canary` roadmap and `DOCUMENT-MODEL.md` still contain flexible/positioned placement wording | PR #18 defines fixed one-to-four-column page flow. Preserve that dependent approved contract; do not implement free positioning. |
| Release targets | Electron Forge scripts include multiple operating systems and makers | Windows x64 is mandatory. Other platform targets must not constrain the first Tauri delivery; whether to retire them is a separate product decision. |

## 10. Required local repository scan to close T1

Run these from a clean checkout after fetching current branches. These are inspection-only commands; do not apply automated replacements.

```powershell
# All Electron imports and runtime identifiers
rg -n --hidden --glob '!**/node_modules/**' --glob '!**/.yarn/**' "(from ['\"]electron['\"]|require\(['\"]electron['\"]\)|ipcMain|ipcRenderer|contextBridge|BrowserWindow|WebContentsView|MessageChannelMain|utilityProcess|safeStorage|electron-updater|electron-forge)" .

# Renderer globals and preload API consumers
rg -n --hidden --glob '!**/node_modules/**' --glob '!**/.yarn/**' "(__appInfo|__apis|__events|__sharedStorage|electron:worker-connect|electron-api)" packages

# Electron build/packaging references
rg -n --hidden --glob '!**/node_modules/**' --glob '!**/.yarn/**' "(apps/electron|@affine/electron|electron-forge|make-nsis|make-squirrel|electron-updater|ELECTRON_)" .

# Native module and Rust crate inventory
rg -n --hidden --glob '!**/node_modules/**' --glob '!**/.yarn/**' "(@affine/native|napi|napi-rs|DocStoragePool|DiskSync|renderTypstSvg|renderMermaidSvg)" packages
```

Record each result as:
- path and symbol;
- caller(s);
- whether it is renderer, main, preload, helper, worker, build, test or CI;
- user-visible capability;
- data/security contract;
- proposed retain/adapt/replace/retire disposition;
- test required.

Also inspect the full native package and Rust workspace manifests to determine whether native database, disk-sync, preview and other functionality can be linked directly into a Rust/Tauri host or must remain a separately packaged process.

## 10A. Repository-tree and renderer API verification (2026-11-12)

The live recursive Git tree for `canary` returned 12,828 paths with `truncated=false`. The Electron package tree contains the main/preload/helper/worker/window/security/storage/updater/import/preview/sync source areas and their focused tests. The scan confirms the following API boundary facts:

- `packages/frontend/apps/electron-renderer/package.json` directly depends on `@affine/electron-api`.
- `packages/frontend/electron-api/src/index.ts` exports four renderer globals: `__appInfo`, `__apis`, `__events`, and `__sharedStorage`; it also exports shared desktop state and tab/update/import types.
- The API package type-imports main handlers/events, helper handlers/events, preload `AppInfo`, shared storage, import-source types, and window/tab metadata. This couples the public renderer API types to Electron package implementation modules and should be removed by the T2 framework-neutral contract.
- The preload builds its callable API dynamically from handler/event metadata, merges main and helper namespaces, and adds a native-file import adapter using Electron `webUtils.getPathForFile`.
- The helper exposes `db`, `nbstore`, `diskSync`, `workspace`, `dialog`, and `preview` handler namespaces; its events include `db`, `diskSync`, and `workspace`.

**Verification limitation:** GitHub code search returned empty/incomplete results for the relevant Electron identifiers. A recursive tree proves path coverage, not symbol usage. Therefore this run does not claim an exhaustive list of every renderer consumer or Electron import. The local `rg` scan in section 10 remains required before T1 can be closed. The exact main-handler namespace map also remains to be captured from the local scan.

## 11. T1 deliverables and exit criteria

T1 is complete only when:

- [x] Initial Electron desktop package, lifecycle, preload, helper, worker, window, security, storage, updater and packaging paths are inventoried from live source.
- [x] Direct Electron package dependencies and build scripts are recorded.
- [x] Major migration risks and product-scope uncertainties are classified.
- [x] Windows x64 and product architecture invariants are explicit.
- [x] Existing PR dependencies and the no-packaging-repair/no-dependency-removal boundary are explicit.
- [ ] Full local monorepo Electron-import and renderer-global scan is recorded.
- [x] Initial Rust workspace and N-API reuse evidence is recorded; direct Tauri linkage feasibility remains unproven.
- [ ] Every discovered Electron API consumer has a disposition.
- [ ] Required inherited AFFiNE features have retain/adapt/retire decisions.
- [ ] Architecture conflicts listed in section 8 are reconciled through explicit decisions and appropriate documentation/PR updates.

T1 produces an evidence-backed migration inventory. It does not build Tauri, change dependencies, or claim Windows packaging readiness.

## 12. Follow-on milestones (not authorized by T1)

- **T2 — Desktop host contract:** define the framework-neutral typed frontend API and process/service contracts.
- **T3 — Tauri 2 Windows x64 shell:** minimal Tauri host running the existing frontend, with a repeatable development build and test installer.
- **T4 — Native desktop capabilities:** implement approved file, window, menu, deep-link, storage and security behavior.
- **T5 — Calculation host integration:** connect the independent calculation host through the approved CalcpadCE boundary.
- **T6 — Persistence and compatibility:** verify existing document and application data behavior.
- **T7 — Windows distribution:** signed NSIS/MSI candidate, WebView2 handling and updater.
- **T8 — Parity and Electron retirement:** complete Windows validation and only then remove Electron-specific code and dependencies.

No T2–T8 implementation is implied by completion of T1.
