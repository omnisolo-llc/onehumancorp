# Mobile Floating Controls Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Keep help and voice controls accessible without covering mobile product actions in initial, scrolled, or active states, and remove the inbox literal `\n` artifact.

**Architecture:** Hide the two redundant closed help triggers below `sm`, because AppShell owns mobile Help Center access. Move the single VoiceAssistant owner from RootLayout into AppShell: mobile trigger/status surfaces use normal topbar flow, while responsive desktop classes retain bottom-center fixed behavior.

**Tech Stack:** Next.js 14, React 18, Tailwind CSS 3, Vitest, Playwright.

---

### Task 1: Mobile collision regression

**Files:**
- Modify: `src/ui/next/src/e2e/app-shell-style.spec.ts`

- [ ] **Step 1: Write the failing test**

Add a `Mobile global controls` matrix for `/website-builder`, `/login`, `/agent-marketplace`, `/integrations`, `/agents`, and `/inbox` at widths 320 and 390. Select `#ohc-floating-help-btn`, `#ai-chat-trigger-btn`, and `[aria-label="Voice Assistant"]`; assert the only visible closed global trigger is Voice Assistant, every visible control stays inside the viewport, and its bounding box does not intersect `.app-brand`, `.app-nav`, `.app-topbar`, or visible interactive descendants of `.app-main` outside its own overlay.

- [ ] **Step 2: Run test to verify it fails**

Run: `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/snap/bin/chromium pnpm exec playwright test src/e2e/app-shell-style.spec.ts --grep "mobile global controls" --reporter=line --workers=1`

Expected: FAIL because both redundant help triggers are visible and current controls intersect mobile content.

- [ ] **Step 3: Keep the regression unchanged for implementation**

Do not weaken existing shell, overflow, radius, or inbox hydration assertions. The collision selector must exclude the active control and its ancestors/descendants so a control cannot collide with itself.

### Task 2: Responsive global control ownership

**Files:**
- Modify: `src/ui/next/src/components/help.tsx`
- Modify: `src/ui/next/src/components/HelpChat.tsx`
- Modify: `src/ui/next/src/components/VoiceAssistant.tsx`
- Test: `src/ui/next/src/e2e/app-shell-style.spec.ts`

- [ ] **Step 1: Hide redundant mobile help triggers**

Add `hidden sm:block` to the closed HelpWidget and HelpChat fixed trigger wrappers. Keep their open surfaces and desktop positions unchanged.

- [ ] **Step 2: Move and mark mobile voice control**

Remove the RootLayout import/render and render one VoiceAssistant in AppShell's topbar action region. Mark the root, trigger, and status surface/state. Use mobile normal-flow classes with stable full-width status layout, overridden at `sm` by the existing bottom-center fixed layout.

- [ ] **Step 3: Run collision test to verify it passes**

Run the Task 1 Playwright command.

Expected: route/viewport cases pass at 320 and 390 initially and after scrolling, with Voice Assistant as the only visible mobile voice trigger and zero collisions in idle, listening, processing, error, and success states.

- [ ] **Step 4: Run affected component tests**

Run: `pnpm exec vitest run src/components/help.test.tsx src/components/HelpChat.test.tsx src/components/VoiceAssistant.test.tsx`

Expected: all files and tests pass.

- [ ] **Step 5: Commit**

Commit the three component changes and Playwright regression with message `fix(ui): prevent mobile floating control collisions`.

### Task 3: Inbox literal text artifact

**Files:**
- Modify: `src/ui/next/src/app/inbox/page.test.tsx`
- Modify: `src/ui/next/src/app/inbox/page.tsx`

- [ ] **Step 1: Write the failing component assertion**

In the settled empty-state test, capture the render container and assert `container.textContent` does not contain the two-character string `\\n`.

- [ ] **Step 2: Run test to verify it fails**

Run: `pnpm exec vitest run src/app/inbox/page.test.tsx`

Expected: FAIL because `page.tsx` renders a literal `\n` text node between sections.

- [ ] **Step 3: Remove the literal source text**

Delete only the stray `\n` token before the first inbox `</section>` boundary; preserve surrounding JSX.

- [ ] **Step 4: Run test to verify it passes**

Run the Task 3 Vitest command.

Expected: the inbox test file passes.

- [ ] **Step 5: Commit**

Commit the page and test with message `fix(ui): remove inbox literal newline artifact`.

### Task 4: Full verification and report evidence

**Files:**
- Modify: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Run code gates**

Run full Vitest, `npm run test:tailwind-config`, `pnpm exec tsc --noEmit`, and `pnpm run build`; require zero failures and exit 0.

- [ ] **Step 2: Run rendered gates**

Run the production 43-test Playwright matrix with `/snap/bin/chromium`, line reporter, one worker; run the secured 36-case visual audit against `next start`; require zero failures, complete coverage, and 36 screenshots.

- [ ] **Step 3: Inspect affected originals**

Open original-resolution desktop/mobile dashboard, agents, integrations, website-builder, login, mobile agent-marketplace, and mobile inbox screenshots. Confirm no control collision, clipping, duplicate shell, or invalid document overflow at 320/390; distinguish legitimate local tab scrollers.

- [ ] **Step 4: Run repository gates**

Run `bazel test //src/ui/next:next_vitest --test_output=errors` and root/UI `pnpm audit --prod` plus `npm audit --omit=dev`; require all zero.

- [ ] **Step 5: Update and commit report**

Record exact counts, build evidence, 18x2 matrix results, Chromium override, expected missing-backend errors, visual observations, and audits under `### UI-01 — Universal UI shell and rendered consistency`. Commit with message `docs: record universal UI verification`.

- [ ] **Step 6: Clean generated artifacts and final review**

Restore only run-generated `.next`, `tsconfig.tsbuildinfo`, `test-results`, `playwright-report`, and coverage changes; run `git diff --check`, review all commits, and require a clean `git status --short`.

### Task 5: Review correction for active and scrolled voice states

**Files:**
- Modify: `src/ui/next/src/app/layout.tsx`
- Modify: `src/ui/next/src/app/components/AppShell.tsx`
- Modify: `src/ui/next/src/components/VoiceAssistant.tsx`
- Modify: `src/ui/next/src/components/VoiceAssistant.test.tsx`
- Modify: `src/ui/next/src/e2e/app-shell-style.spec.ts`
- Modify: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Add RED ownership and state regressions**

Assert RootLayout no longer owns VoiceAssistant, AppShell owns exactly one instance, responsive classes are normal-flow below `sm` and fixed at desktop, and trigger/status markers remain accessible. Browser tests must mock media recording and voice API results to observe listening, processing, error, and success after scroll at both widths.

- [ ] **Step 2: Verify RED against reviewed defects**

Run affected Vitest files and the mobile collision Playwright grep. Expected failures: RootLayout ownership remains, voice is viewport-fixed on mobile, scrolled topbar intersects it, and active state surfaces are unmarked/untested.

- [ ] **Step 3: Implement shell-owned mobile flow**

Move VoiceAssistant into AppShell topbar actions, remove RootLayout ownership, add stable surface/state markers, keep mobile panels in normal flow with stable width, and preserve desktop fixed bottom-center placement.

- [ ] **Step 4: Verify GREEN and full gates**

Run focused component/browser tests, full Vitest, TypeScript, build, expanded production Playwright, secured audit, visual inspection including scrolled/active states, and final Bazel. Amend UI-01 and the historical TypeScript statement using actual results.

### Task 6: Review correction for voice lifetime and accessibility

**Files:**
- Modify: `src/ui/next/src/components/VoiceAssistant.test.tsx`
- Modify: `src/ui/next/src/components/VoiceAssistant.tsx`
- Modify: `docs/reports/production_agent_optimization_report.md` when final counts change

- [ ] **Step 1: Write lifecycle RED tests**

Use controllable `getUserMedia` promises and recorder instances. Assert that normal stop releases each track once; unmount during recording stops the active recorder and every track once, detaches callbacks, performs no fetch, and emits no post-unmount state warning; and a stream resolving after unmount has all tracks stopped without constructing or starting a recorder. Manually invoke retained recorder callbacks after unmount to prove they cannot send audio or update state.

- [ ] **Step 2: Write accessibility RED tests**

Assert Enter and Space independently start on non-repeating keydown and stop on keyup, repeated keydown does not request another stream, mouse hold remains start/stop, and `detail === 0` click toggles start/stop without a preceding keyboard sequence double-triggering. Assert dynamic `aria-pressed`/label values and visible `role="status"` live announcements, including assertive error output.

- [ ] **Step 3: Run focused tests and confirm expected RED**

Run: `pnpm exec vitest run src/components/VoiceAssistant.test.tsx`

Expected: FAIL because the current component has no mounted/stream lifetime guard, keyboard or synthetic-click handlers, recording ARIA state, or live status semantics.

- [ ] **Step 4: Implement idempotent media ownership**

Add mounted, stream, recorder, recording, pending-intent/session, released-track, and timeout refs. Centralize exact-once track release. On unmount, invalidate work, detach callbacks, stop the recorder when active, release tracks, clear refs/timers, and never set state. After `getUserMedia`, validate the mounted session before constructing `MediaRecorder`. Guard `ondataavailable`, `onstop`, fetch continuations, event dispatch, timers, and failure state; log only fixed messages.

- [ ] **Step 5: Implement accessible interaction semantics**

Keep mouse/touch hold handlers. Add non-repeating Enter/Space keydown/keyup handlers with `preventDefault`, plus a keyboard-click suppression ref and a `detail === 0` assistive toggle. Expose `aria-pressed`, dynamic operational labels, and atomic live status semantics without changing shell-flow classes or collision markers.

- [ ] **Step 6: Verify GREEN and code-dependent gates**

Run focused VoiceAssistant tests, full Vitest, `pnpm exec tsc --noEmit`, `pnpm run build`, focused production collision/state Playwright, the 58-case scoped production suite, the secured 36-case visual audit, and `bazel test //src/ui/next:next_vitest --test_output=errors`. Record exact changed counts and require a clean worktree after commits.
