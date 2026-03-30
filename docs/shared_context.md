# Developer Insights: Shared Context

## Identity
This document aggregates architectural updates and technical debt resolutions translated into human-readable developer insights for the One Human Corp (OHC) Swarm platform. It serves as the bridge between raw source code implementations and our premium technical documentation.

## Technical Debt Synthesis

### 1. Database Locking & Concurrency
**Insight:** High-concurrency environments utilizing the SQLite driver (`modernc.org/sqlite`) for Swarm SIP Database testing often encounter 'database is locked (SQLITE_BUSY)' errors due to concurrent writer contention.
**Resolution:** Always enforce `db.SetMaxOpenConns(1)` and append standard PRAGMAs explicitly via the DSN (`?_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate`). This serializes writes cleanly, mitigating race conditions during Swarm state syncs. Do not prepend `_txlock` with `_pragma=`.

### 2. Bazel Wildcarding & Target Resolution
**Insight:** Developers frequently execute `bazelisk test //path/to/dir:...` which causes 'No such target' errors when Bazel interprets the colon as a specific target instead of a recursive wildcard.
**Resolution:** Always use the three-dot wildcard syntax `//path/to/dir/...` to recursively test all packages within a directory structure.

### 3. Agent Task Payload Integrity
**Insight:** Previous orchestration logic occasionally parsed raw string payloads directly from the `agent_missions` table, causing silent routing failures when the payload lacked necessary routing metadata.
**Resolution:** All tasks in `agent_missions` must contain valid JSON representing a `Message` struct (`id`, `from_agent`, `type`, `content`, `parent_thread_id`). The system strictly enforces this; legacy string payloads must be rewritten into valid JSON via DB migration, not silently cast.

### 4. Circuit Breaker Instantiation
**Insight:** Instantiating circuit breakers locally inside request handlers resets the failure count to zero on every concurrent call, completely defeating the mechanism.
**Resolution:** Circuit Breakers must be instantiated once per client/service (e.g., as a persistent field on the `Hub` or `Service` struct). This ensures the `state` correctly persists across concurrent API calls, triggering open/close modes properly.

### 5. Hermetic Frontend Testing (Playwright)
**Insight:** Dynamic execution of `npm install` inside a Bazel `sh_test` wrapper violates hermeticity and leads to non-deterministic test failures during frontend verification.
**Resolution:** Always declare `@nodejs//:node` and `//:node_modules` in the `data` attributes of the `sh_test`. Dynamically locate binaries using `RUNFILES_DIR` and explicitly set `PLAYWRIGHT_BROWSERS_PATH="${TEST_TMPDIR:-/tmp}/pw_browsers"` to maintain a strictly isolated execution sandbox.

## Premium Formatting
As per the Aesthetic Excellence Mandate, technical debt resolutions listed above directly inform the UI. Loading screens and error boundaries must wrap these technical failures in premium visual components utilizing our designated Glassmorphism tokens (`backdrop-filter: blur(15px) saturate(180%)`).
