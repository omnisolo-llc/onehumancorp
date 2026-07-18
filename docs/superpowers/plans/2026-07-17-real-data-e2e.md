# Real-data Browser E2E Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Bazel Playwright tests authenticate through the real service and prove that visible records and images originate from deterministic PostgreSQL seed data without network stubbing or fabricated UI fallbacks.

**Architecture:** Global setup obtains a genuine cookie session from `/api/v1/auth/login` and writes a reusable Playwright storage state, while the shared fixture performs the same real login when switching users. Static CI contracts reject response substitution in selected specs, and a real-data browser contract verifies stable seeded identifiers and a decoded tracked image through the running stack.

**Tech Stack:** Bazel, Bash, Playwright 1.61, TypeScript, Next.js, Rust/Axum, PostgreSQL.

---

### Task 1: Real authentication state

**Files:**
- Create: `src/e2e/authenticate.ts`
- Modify: `src/e2e/global-setup.ts`
- Modify: `src/e2e/fixtures.ts`
- Modify: `playwright.config.ts`
- Test: `src/ui/next/e2e/agents.spec.ts` through its generated Bazel target

- [ ] **Step 1: Capture the failing protected-page test**

Run:

```bash
bazel test //src/e2e:playwright__s__s_src_s_ui_s_next_c_e2e_s_agents_d_spec_d_ts_next_ui --remote_cache= --remote_executor= --nocache_test_results --test_output=errors
```

Expected: FAIL with `/agents` rendering `{"error":"authentication unavailable"}`.

- [ ] **Step 2: Add one real-login helper**

Create a helper with this interface:

```ts
export type E2ECredentials = {
  email: string;
  password: string;
  organizationId?: string;
};

export async function authenticateRequest(
  request: APIRequestContext,
  credentials: E2ECredentials,
): Promise<void>;
```

It must POST JSON to `/api/v1/auth/login`, require a successful response, and include the bounded response body in an error without printing cookies or tokens.

- [ ] **Step 3: Persist real global storage state**

In global setup, create an API request context using the configured `baseURL`, authenticate `test@example.com` with `password123`, and write `storageState` to `PLAYWRIGHT_STORAGE_STATE`. In `playwright.config.ts`, set `use.storageState` only when that environment variable is present.

- [ ] **Step 4: Repair user-switching fixtures**

Change `loginAs(page, user)` to authenticate through `page.request`, then navigate to `/dashboard`. Do not set cookies, tokens, tenant headers, or localStorage directly.

- [ ] **Step 5: Run the protected-page test**

Run the command from Step 1. Expected: browser launch and authentication succeed; any remaining failure must be a page assertion rather than `authentication unavailable`.

### Task 2: No-substitution CI contract

**Files:**
- Modify: `bazel/rules/playwright/playwright_spec_coverage_check.sh`
- Modify: `bazel/rules/playwright/playwright_spec_coverage_check_test.sh`

- [ ] **Step 1: Add failing fixtures for prohibited E2E substitutions**

Extend the shell test with selected spec fixtures containing `page.route(...)`, `route.fulfill(...)`, `page.setContent(...)`, `Buffer.from('fake image data')`, and a mock business payload. Each selected fixture must make the checker fail and name the offending file.

- [ ] **Step 2: Verify the new cases fail before implementation**

Run:

```bash
bazel test //bazel/rules/playwright:playwright_spec_coverage_check_test --remote_cache= --remote_executor= --nocache_test_results --test_output=errors
```

Expected: FAIL because the checker does not yet reject the prohibited patterns.

- [ ] **Step 3: Enforce the selected-spec contract**

Scan only the `--ci` selection for browser response substitution and fabricated image/business data. Diagnostics must contain the spec path and prohibited category. Do not reject legitimate words in test titles or assertions about the absence of mock data.

- [ ] **Step 4: Verify the checker**

Run the command from Step 2. Expected: PASS, including a control fixture that uses `setInputFiles` with a real PNG path and real `request.get/post` calls.

### Task 3: Seeded record and real-image proof

**Files:**
- Modify: `src/e2e/e2e-seed.sql`
- Modify: `src/e2e/real_data_contract.spec.ts`
- Use: `src/ui/next/src/e2e/fixtures/test_img.png`

- [ ] **Step 1: Add failing browser assertions**

Add a real-data test that authenticates through the shared fixture, requests the production products API, asserts stable seeded product ID `e2e-product-cake`, opens the corresponding UI, and verifies the rendered product text. Add an image assertion that loads the tracked PNG through the application-visible URL or upload path and requires `naturalWidth > 0` and `naturalHeight > 0`.

- [ ] **Step 2: Run the real-data target and capture the boundary failure**

Run the generated per-spec Bazel target for `src/e2e/real_data_contract.spec.ts` with remote cache/executor disabled. Expected before seed/API wiring: FAIL at the missing product/image boundary, not from a fabricated response.

- [ ] **Step 3: Extend deterministic seed data minimally**

Add only the image metadata or media row required by Step 1, using stable IDs and an application-served path. Preserve RLS restoration and idempotent `ON CONFLICT` behavior.

- [ ] **Step 4: Verify real data and image decoding**

Rerun the target. Expected: PASS with records fetched from PostgreSQL and the browser reporting non-zero intrinsic image dimensions.

### Task 4: Integration and repository gates

**Files:**
- Modify: `src/e2e/BUILD.bazel`
- Modify: `bazel/rules/playwright/playwright_test.sh` only if storage-state path export is required
- Review: all working-tree changes

- [ ] **Step 1: Put the real-data contract in the explicit CI selection**

Add `//src/e2e:real_data_contract.spec.ts` to the maintained CI list while preserving the already selected Next.js specs and honest coverage counts.

- [ ] **Step 2: Export a writable storage-state path**

Set `PLAYWRIGHT_STORAGE_STATE` under `TEST_TMPDIR` before Playwright list/run commands so global setup and workers share the same file.

- [ ] **Step 3: Run focused contracts**

```bash
bazel test //bazel/rules/playwright:playwright_browser_contract_test //bazel/rules/playwright:playwright_browser_smoke_test //bazel/rules/playwright:playwright_spec_coverage_check_test //bazel/rules/playwright:playwright_spec_discovery_test //bazel/rules/playwright:playwright_tls_certificate_test //src/e2e:playwright_spec_coverage --remote_cache= --remote_executor= --nocache_test_results --test_output=errors
```

Expected: all targets PASS.

- [ ] **Step 4: Run the genuine CI shard**

```bash
bazel test //src/e2e:playwright_shard_1_of_1 --remote_cache= --remote_executor= --nocache_test_results --test_output=errors
```

Expected: PASS with a non-zero test count.

- [ ] **Step 5: Run the full repository suite**

```bash
bazel test //...
```

Expected: all configured targets PASS with remote execution disabled and remote cache only.

- [ ] **Step 6: Rebase, push, and verify CI**

Fetch `origin/main`, rebase local `main`, push, then monitor the resulting GitHub Actions run until the required CI job and every substantive step are successful.
