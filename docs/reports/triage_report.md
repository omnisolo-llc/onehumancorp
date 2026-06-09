# Triage Report

**Date:** 2026-06-02
**Role:** Maintainer - Incident Triage & Infrastructure

## Summary of Signals

During an initial codebase discovery and testing sweep using `bazel test //...` and `npm run vitest`, multiple recurring failure signals were observed. The signals indicate widespread issues with remote package resolution, Bazel execution timeouts in the local/sandbox environment, and Javascript dependency drift.

---

## Signals

### 1. External Network Fetch Failure (Bazel / Go Rules)
* **Category:** `bug`
* **Severity:** High (Blocks Go compilation)
* **Signal:**
  ```text
  Error downloading [https://github.com/cncf/xds/archive/555b57ec207be86f811fb0c04752db6f85e3d7e2.tar.gz] to ...: Unknown host: codeload.github.com
  ```
* **Context:** The `xds+` bazel repository fails to download because the sandbox/environment cannot resolve `codeload.github.com`. This causes a cascading failure preventing Go dependencies and the `gazelle` build systems from executing properly.
* **Resolution:** This was determined to be a transient networking issue. Subsequent runs of `bazel test //...` successfully resolved the host and fetched the dependencies.

### 2. Bazel Timeouts and Test Suite Freezes
* **Category:** `bug`
* **Severity:** High (Blocks test verification)
* **Signal:**
  ```text
  The command timed out after 401.3451166152954 seconds.
  ```
* **Context:** When running `bazel test //...` or even sub-targets like `//src/server/...` or `//src/server/common/...`, the tests run into timeouts and eventually freeze or crash. Memory limits and large action execution (e.g., compiling large Rust crates) might be overwhelming the local host / sandbox CPU.
* **Proposed Action:** Verify local sandbox resource limits, potentially limit local test concurrency (e.g., set `--local_test_jobs=1`), or skip remote execution if there is an issue with the remote-cache.

### 3. Missing JavaScript Modules for UI Tests (Vitest)
* **Category:** `bug`
* **Severity:** Medium (Breaks UI testing)
* **Signal:**
  ```text
  vitest.config.ts (1:223) [UNRESOLVED_IMPORT] Could not resolve 'vitest/config' in vitest.config.ts
  vitest.config.ts (2:18) [UNRESOLVED_IMPORT] Could not resolve '@vitejs/plugin-react' in vitest.config.ts
  ```
* **Context:** Running `npx vitest run` directly fails due to missing modules `vitest/config` and `@vitejs/plugin-react`. While `package.json` specifies `vitest: ^4.1.8` and `@vitejs/plugin-react: ^6.0.2`, the modules are not found at runtime, possibly due to `npm install` failing earlier or missing lockfile entries. (Note: Running `bazel test //src/ui/...` using the remote cache succeeded previously).
* **Resolution:** Running `npm ci` followed by explicitly installing the dependencies as dev dependencies `npm i -D @vitejs/plugin-react vitest` resolved the local module errors. Additionally, `vitest` tests failing due to un-wrapped `act(...)` updates in `src/ui/next/src/app/website-builder/page.test.tsx` were fixed by properly wrapping `fireEvent` calls.

### 4. Dependency Drift: Aspect Bazel Lib
* **Category:** `refactor` / `cleanup`
* **Severity:** Low
* **Signal:**
  ```text
  WARNING: For repository 'aspect_bazel_lib', the root module requires module version aspect_bazel_lib@2.7.9, but got aspect_bazel_lib@2.22.5 in the resolved dependency graph.
  ```
* **Context:** `MODULE.bazel` specifies `2.7.9`, but the resolved graph fetches `2.22.5`.
* **Proposed Action:** Update `MODULE.bazel` to specify `aspect_bazel_lib@2.22.5` to eliminate the warning and align the version graph.
