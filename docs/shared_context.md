# Shared Context: Developer Insights

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.1); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 8px; color: white;">

## Developer Insights

### Hermeticity & Testing
*   **Hermetic Testing**: All builds and tests (unit, integration, E2E) MUST be executed exclusively via `bazelisk` or `bazel`. Manual scripts or other runners like `npm` or `go test` are strictly prohibited within the CI/CD and development workflow context.
*   **Global Overrides**: When overriding global package variables (like API URLs) for testing, expose a getter and wrap modifications with `defer func() { Set...(originalURL) }()$ ` to prevent test pollution and side-effects across the test suite.
*   **Playwright & UI Testing**: Mandatory use of the `browser` tool (Playwright) for UI stability, visual regression, and system-state verification.
*   **Flutter Testing**: In Flutter widget tests (`testWidgets`), use `await tester.pumpAndSettle();` instead of `await tester.pump();` after actions that trigger animations or asynchronous state changes (like tap events) to avoid flaky element-not-found errors. When utilizing `shared_preferences`, initialize mock values early using `SharedPreferences.setMockInitialValues({});` inside `setUpAll()`.
*   **Playwright inside Bazel**: When executing Playwright tests inside a Bazel `sh_test` wrapper, ensure strict hermeticity. Dynamically locate the binaries using `RUNFILES_DIR` and explicitly set `PLAYWRIGHT_BROWSERS_PATH`.

### Architecture & Design Patterns
*   **Zero Secrets Mandate**: Rely entirely on SPIFFE/SPIRE for identity and authentication across the platform. Do not use hardcoded credentials. Never mock network requests in the frontend; authenticate via the backend API.
*   **Swarm Intelligence Protocol (OHC-SIP)**: Includes a file-based 'Swarm-as-Code' orchestration layer. Agents must read/write shared global intelligence in `.agents-tasks/memory/{timestamp}.yml`, pick and execute/delete queued tasks from `.agents-tasks/missions/`, and output observability metrics to `.agents-tasks/status/{timestamp}.yml`.
*   **Visual Excellence**: All OHC interfaces and markdown documentation must adhere to the Aesthetic Excellence mandate, utilizing premium CSS tokens such as Glassmorphism (`backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.05)`) and `Outfit/Inter` typography. Every Bazel package component must include a `README.md` featuring Mermaid.js diagrams.
*   **Database Best Practices**: When using the `modernc.org/sqlite` driver in Go for high-concurrency connections, mitigate 'database is locked (SQLITE_BUSY)' by setting `db.SetMaxOpenConns(1)` and explicitly formatting PRAGMAs in the DSN.

### Go Language Best Practices
*   **Lock Contention**: To eliminate bottlenecks in Go task-execution engines (e.g., event publishers), reduce lock contention by aggregating resources (like notification channels) into a local slice inside the `sync.RWMutex` lock, then release the lock *before* iterating and sending to those channels.
*   **Memory Allocation**: For performance-critical Go code with high-frequency memory allocations, explicitly utilize `sync.Pool` to reuse objects and minimize latency.
*   **Circuit Breakers**: When implementing Circuit Breakers, instantiate them once per client/service rather than locally inside request handlers to ensure failure states persist correctly.

</div>
