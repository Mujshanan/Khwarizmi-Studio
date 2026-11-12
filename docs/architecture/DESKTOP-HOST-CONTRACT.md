# T2 — Framework-neutral desktop host contract

**Status:** Draft — contract design in progress  
**Target host:** Tauri 2 on Windows x64  
**Base:** `docs/tauri-desktop-migration-inventory`  
**Scope:** Define the boundary between the existing React/TypeScript renderer and desktop capabilities/services. This document does not implement a Tauri host, migrate Electron code, or change the calculation engine.

## 1. Design goals

1. Keep React, BlockSuite, MathLive, document models, and calculation semantics independent of Electron and Tauri.
2. Expose a small, explicit, typed API to the renderer. No dynamically discovered namespace/method bridge.
3. Keep privileged operations in the desktop host or a dedicated service; renderer code receives only capabilities it needs.
4. Keep persistence, import/export, preview, disk sync, and calculation execution behind stable service contracts.
5. Preserve Windows x64 as a required validation target.
6. Preserve the approved CalcpadCE calculation-engine boundary. The desktop host is an adapter/supervisor, not the mathematical evaluator.
7. Keep transport details replaceable. Tauri commands/events, sidecars, and any future transport are adapters behind these contracts.

## 2. Non-goals

- No Tauri commands, Rust structs, plugin choices, or IPC channel names are public frontend contracts.
- No Electron packaging repair, Electron dependency removal, or Electron runtime rewrite.
- No blanket port of AFFiNE Desktop features.
- No calculation-language, MathJSON, CalcpadCE, worksheet-layout, or document-schema redesign.
- No assumption that the current Electron helper process must remain a separate process.
- No direct renderer access to arbitrary filesystem paths, shell commands, environment variables, or OS secrets.

## 3. Layering

```text
React / BlockSuite application
          |
          v
Khwarizmi Desktop API (typed, framework-neutral)
          |
          +---- Desktop capability adapter
          |        (Tauri host; later)
          |
          +---- Workspace / persistence service
          |
          +---- Import / export service
          |
          +---- Preview / rendering service
          |
          +---- Disk sync / backup service (if retained)
          |
          +---- Calculation service client
                   |
                   v
             CalcpadCE host boundary
```

The renderer depends only on the desktop API package. Platform adapters implement it. Services may run in-process or out-of-process; that is an implementation decision constrained by isolation, reliability, and measured workload.

## 4. Public API shape

The initial contract is a composition of typed service interfaces, not a generic `invoke(namespace, method, ...args)` escape hatch.

```ts
export interface DesktopHost {
  readonly app: AppService;
  readonly window: WindowService;
  readonly files: FileDialogService;
  readonly clipboard: ClipboardService;
  readonly externalLinks: ExternalLinkService;
  readonly preferences: PreferencesService;
  readonly events: DesktopEventService;
  readonly workspace: WorkspaceService;
  readonly imports: ImportService;
  readonly previews: PreviewService;
  readonly calculations: CalculationService;
  readonly updates?: UpdateService;
  readonly diagnostics: DiagnosticsService;
}
```

The exact service set is provisional. Each service is retained only if Khwarizmi product requirements need it; optional services must be represented as optional capabilities, not no-op implementations that imply support.

### 4.1 App and host information

```ts
export interface AppInfo {
  readonly appId: 'khwarizmi-studio';
  readonly appVersion: string;
  readonly buildChannel: 'stable' | 'beta' | 'canary' | 'development';
  readonly host: 'desktop';
  readonly hostVersion: string;
  readonly platform: 'windows';
  readonly architecture: 'x64';
}
```

Do not expose an `electron: true` flag or require renderer code to branch on the desktop framework. Host version is diagnostic metadata, not a feature-detection mechanism.

### 4.2 Common result and error model

Operations that can fail use a consistent, serializable error contract. The adapter maps native errors into stable application codes; raw stack traces and OS error objects stay in host logs.

```ts
export type DesktopErrorCode =
  | 'CANCELLED'
  | 'NOT_FOUND'
  | 'PERMISSION_DENIED'
  | 'INVALID_ARGUMENT'
  | 'CONFLICT'
  | 'UNAVAILABLE'
  | 'TIMEOUT'
  | 'TOO_LARGE'
  | 'UNSUPPORTED'
  | 'INTERNAL';

export interface DesktopError {
  readonly code: DesktopErrorCode;
  readonly message: string;
  readonly retryable: boolean;
  readonly details?: Readonly<Record<string, string | number | boolean>>;
}

export type Result<T> =
  | { readonly ok: true; readonly value: T }
  | { readonly ok: false; readonly error: DesktopError };
```

Expected user cancellation is a normal `CANCELLED` result, not an unhandled exception. Unexpected programming errors remain observable in diagnostics and tests.

### 4.3 Events and cancellation

- Event payloads are versioned, typed, serializable DTOs.
- `subscribe` returns an idempotent unsubscribe function.
- Subscriptions are scoped to the renderer lifetime and cleaned up on navigation/window teardown.
- Events are notifications, not the source of truth for persisted state.
- Events that represent state transitions include a monotonic revision or operation ID where ordering matters.
- Long-running requests accept an opaque cancellation token; cancellation is explicit and idempotent.
- The transport must not expose raw Node `MessagePort`, Electron IPC objects, Rust channels, or browser-specific event types.

```ts
export interface DesktopEventService {
  subscribe<K extends keyof DesktopEventMap>(
    name: K,
    listener: (event: DesktopEventMap[K]) => void
  ): () => void;
}

export interface OperationOptions {
  readonly requestId?: string;
  readonly cancellationId?: string;
}
```

### 4.4 Window service

Initial candidates: query current window state, minimize, maximize/restore, close request, focus, and set title. Window creation, tabbed web contents, popups, and multi-window behavior remain unresolved until the T1 consumer audit and product-scope review establish that Khwarizmi needs them.

The renderer cannot select arbitrary URLs or create privileged windows. Any external navigation is routed through the host's allowlisted external-link service.

### 4.5 File dialogs and file capabilities

The API may request open/save dialogs and receive opaque, scoped file references. It must not grant the renderer ambient filesystem authority.

```ts
export interface FileCapability {
  readonly id: string;
  readonly displayName: string;
  readonly mediaType?: string;
  readonly sizeBytes?: number;
}

export interface FileDialogService {
  open(options: OpenFileDialogOptions): Promise<Result<readonly FileCapability[]>>;
  save(options: SaveFileDialogOptions): Promise<Result<FileCapability>>;
}
```

A selected file capability is limited to the selected operation and required read/write mode. Import/export services consume capabilities; arbitrary path strings are not accepted as authority. The final contract must support directory import only if the product requires it and the security review defines a bounded directory capability.

### 4.6 Workspace and persistence

Workspace operations use domain DTOs and stable document/workspace identifiers. The desktop contract does not leak SQLite handles, SQL queries, N-API objects, Yjs internals, or Electron helper implementation types.

Required semantic properties:
- durable authored content and evaluation snapshots remain separate, integrity-checked records;
- writes are atomic at the contract boundary or report a recoverable conflict;
- document identity, revision, and migration metadata are explicit;
- operations that can conflict return a typed conflict instead of silently overwriting;
- data migration is explicit, versioned, and testable.

Exact CRUD methods and storage DTOs must be reconciled with the current document-model and persistence PR contracts before being frozen.

### 4.7 Import/export

Import is a staged operation: validate source capability and format, create an import session, report progress, then commit or cancel. The renderer must not pass arbitrary local paths. The API must distinguish user cancellation, unsupported format, invalid source, partial import, and successful commit.

Export uses a host-selected destination capability and reports completion only after the output is committed. Large operations support progress and cancellation.

### 4.8 Preview/rendering

Preview requests are pure, bounded requests over explicit input content and rendering options. The service returns a typed result or a scoped preview artifact reference. It must not accept arbitrary filesystem paths or arbitrary executable commands.

Whether Mermaid and Typst rendering remain in a separate helper/sidecar or move into the Tauri-managed service is unresolved pending dependency and performance checks.

### 4.9 Calculation service boundary

The desktop API exposes calculation-host lifecycle and request operations only. It does not implement parsing/evaluation itself and does not make MathLive or MathJSON the numeric engine.

The service contract must preserve the approved CalcpadCE boundary and the existing calculation contract's:
- deterministic request/response semantics;
- exact decimal/rational representation rules;
- variable scope, assignment versioning, and immutable evaluation snapshots;
- substitution provenance and diagnostics;
- units and dimensional checks;
- bounded transport framing (16 MiB v1 limit where the existing RPC contract applies);
- cancellation, request correlation, and integrity/digest verification.

The exact calculation DTOs remain owned by the calculation-contract milestone and must be imported/reused, not duplicated in this desktop-host document.

### 4.10 Preferences, clipboard, external links, diagnostics, updates

- Preferences: typed, schema-versioned settings only; no generic arbitrary key-value access for privileged state.
- Clipboard: narrow text/image operations with explicit supported payload types and size limits.
- External links: host validation and allowlist policy; no renderer-side shell or arbitrary protocol invocation.
- Diagnostics: structured logs and support bundles with secrets and document content redacted by default.
- Updates: optional capability until update channels, signing, rollback, and release policy are decided. Update installation is never initiated by arbitrary renderer code.

## 5. Security and reliability requirements

1. The renderer is untrusted for authorization purposes even when it is first-party UI.
2. Every privileged command validates input schema, capability, origin/window identity, and allowed operation.
3. File access is capability-scoped and least-privilege.
4. Secrets are stored and used in the host; secrets are never returned to the renderer unless a narrowly defined user-facing flow requires a masked/non-secret status.
5. No shell execution API is exposed.
6. Every request has bounded payload size, explicit timeout/cancellation behavior, and structured errors.
7. Long-running service failures are isolated and recoverable; service restart must not silently discard committed document data.
8. Logs redact tokens, credentials, private file paths where appropriate, and authored calculation content by default.
9. Windows x64 integration tests are mandatory for host adapters and privileged operations.

## 6. Compatibility and versioning

- The public TypeScript contract has an explicit semantic version.
- Every transport adapter declares the contract version it implements.
- DTOs are plain serializable data; no class instances, functions, symbols, native handles, or framework-specific types cross the boundary.
- Breaking changes require a versioned contract update and migration tests.
- Additive optional fields are preferred; unknown enum values must be handled safely.
- The adapter validates both request and response payloads at runtime at the process boundary.

## 7. Existing Electron bridge mapping (initial, not exhaustive)

| Existing mechanism | Contract destination | Migration treatment |
|---|---|---|
| Dynamic main-process handler metadata and IPC invoke | Explicit service methods | Replace; no dynamic namespace reflection |
| Main-process events and subscription channels | Typed event service | Replace; preserve unsubscribe and lifecycle cleanup |
| Helper-process RPC over transferred MessagePort | Service interfaces | Adapt; process boundary remains a decision |
| `appInfo` with Electron flag/window identifiers | Framework-neutral `AppInfo` | Replace |
| `webUtils.getPathForFile` and native import path resolution | File capability + import service | Replace; do not pass ambient path authority |
| DB/nbstore handlers | Workspace/persistence service | Adapt after native crate and document-contract audit |
| Disk-sync handlers/events | Optional sync/backup service | Unresolved product requirement |
| Dialog handlers | File dialog service | Replace |
| Preview handlers | Preview service | Adapt |
| Workspace handlers/events | Workspace service + typed events | Adapt |
| Electron updater | Optional update service | Replace only after release policy decision |

The current `@affine/electron-api` package is a type/API entry point for the Electron renderer, and `@affine/electron-renderer` declares it as a dependency. This is an observed package-level consumer, not a complete symbol-level consumer inventory. The T1 local monorepo scan remains a prerequisite before declaring the migration inventory closed.

## 8. Open decisions and verification gates

| ID | Question | Gate |
|---|---|---|
| T2-D1 | Which inherited AFFiNE capabilities are required: cloud/auth, sync, embedded tabs, tray, recording, telemetry? | Product-scope approval; no silent retirement |
| T2-D2 | Which exact frontend modules consume each Electron API and renderer global? | Complete local `rg` scan from T1 |
| T2-D3 | Which native Rust crates can be linked directly into the Tauri host, and which require a sidecar? | Windows x64 `cargo check`, feature/N-API/Tokio/migration audit |
| T2-D4 | Does the target require one window or multiple native windows? | Consumer audit and UX requirements |
| T2-D5 | Are directory import and drag/drop local-file flows required? | Import workflow audit and security design |
| T2-D6 | What are update channels, signing, rollback, and offline WebView2 expectations? | Release policy; before updater implementation |
| T2-D7 | What is the exact persistence API and transaction model? | Reconcile document model and persistence PR contracts |
| T2-D8 | What is the service-process topology for storage, preview, and CalcpadCE? | Dependency and failure-isolation review |

## 9. T2 exit criteria

- [ ] Public desktop API types are reviewed and frozen.
- [ ] Each service method has request/response DTOs, error behavior, cancellation, and event semantics.
- [ ] Complete Electron consumer audit maps every consumer to the contract or a documented retirement decision.
- [ ] Product-scope decisions are explicit for inherited AFFiNE features.
- [ ] Process/service topology is decided with Windows x64 evidence.
- [ ] Security review approves file capabilities, external navigation, secrets, and command authorization.
- [ ] Contract conformance tests run independently of Electron and Tauri adapters.
- [ ] Windows x64 host-adapter smoke-test requirements are specified.

T2 is not complete merely because this draft exists. The next focused task is to reconcile the API inventory and product-scope decisions, then turn the provisional service shapes into reviewed DTOs and conformance tests.
