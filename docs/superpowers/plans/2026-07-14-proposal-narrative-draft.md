# Proposal Narrative Draft Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the authenticated `{topic} -> {proposal}` narrative drafting contract consumed by the proposal generator page.

**Architecture:** A dedicated Rust handler lives beside, but remains separate from, the database-oriented `/draft_agent` workflow. It validates claims and bounded input before invoking the existing local LLM boundary with a fixed output budget. The existing authenticated Next proxy forwards lossless JSON to the new fixed path.

**Tech Stack:** Rust/Axum, existing `ResearcherLlmClient` abstraction, Next.js App Router, React, Vitest/Testing Library.

---

### Task 1: Rust narrative draft contract

**Files:**
- Modify: `src/server/api/proposals.rs`

- [ ] **Step 1: Write failing router tests**

Add tests for `POST /draft` that assert:

```rust
// Valid claims and topic
assert_eq!(response.status(), StatusCode::OK);
assert!(body["proposal"].as_str().unwrap().contains("Bakery website"));

// Missing organization claim
assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

// Empty or >4,000 characters
assert_eq!(response.status(), StatusCode::BAD_REQUEST);
```

- [ ] **Step 2: Run and verify RED**

```bash
cargo test --lib api::proposals::tests::narrative_draft -- --nocapture
```

Expected: `404` because `/draft` does not exist.

- [ ] **Step 3: Implement bounded request/response and LLM adapter**

```rust
#[derive(Deserialize)]
struct NarrativeDraftRequest { topic: String }

#[derive(Serialize)]
struct NarrativeDraftResponse { proposal: String }
```

Require a nonblank `claims.organization_id`. Trim `topic`; return `400` when it
is empty or longer than 4,000 Unicode scalar values. Build a fixed system prompt
that requests a concise executive summary, scope, milestones, investment, and
next steps. Invoke the existing chat abstraction with `max_tokens: 900` and no
tools. In tests return a deterministic narrative containing the topic. Map model
errors to `502` and do not return provider error text.

- [ ] **Step 4: Register and verify the route**

Register `.route("/draft", post(draft_narrative))` without changing
`/draft_agent`. Run:

```bash
cargo test --lib api::proposals::tests -- --nocapture
cargo check -p ohc-mono --lib
```

Expected: proposal tests and compile pass.

### Task 2: Proposal page error and response behavior

**Files:**
- Modify: `src/ui/next/src/app/proposals/new/page.tsx`
- Create: `src/ui/next/src/app/proposals/new/page.test.tsx`

- [ ] **Step 1: Write failing component tests**

Mock a successful response `{ proposal: "Bakery website proposal" }`; enter a
topic, submit, and assert the narrative renders. Mock a `502`; assert an
accessible error message renders and `undefined` never appears.

- [ ] **Step 2: Run and verify RED**

```bash
cd src/ui/next
pnpm exec vitest run src/app/proposals/new/page.test.tsx
```

Expected: the non-OK response test fails because the page does not check
`response.ok`.

- [ ] **Step 3: Implement typed success/error handling**

Keep the existing `{ topic }` request. On non-OK response set a stable
`"Failed to draft proposal"` message; on success require a string `proposal`.
Render failures with `role="alert"` and always clear loading in `finally`.

- [ ] **Step 4: Verify the Next proxy and page together**

```bash
cd src/ui/next
pnpm exec vitest run src/app/api/proposals/draft/route.test.ts \
  src/app/proposals/new/page.test.tsx
pnpm exec tsc --noEmit
```

Expected: tests and TypeScript pass. Restore only `tsconfig.tsbuildinfo` if
modified.

### Task 3: Verify and commit

- [ ] **Step 1: Check scope and whitespace**

```bash
git diff --check
git status --short
```

Expected: only proposal Rust/UI/tests are changed.

- [ ] **Step 2: Commit**

```bash
git add src/server/api/proposals.rs \
  src/ui/next/src/app/proposals/new/page.tsx \
  src/ui/next/src/app/proposals/new/page.test.tsx
git commit -m "fix(proposals): add authenticated narrative drafting"
```

