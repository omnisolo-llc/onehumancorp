# Mobile Floating Controls Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Keep global help and voice controls accessible without covering mobile product actions, and remove the inbox literal `\n` artifact.

**Architecture:** Hide the two redundant closed help triggers below `sm`, because AppShell owns mobile Help Center access. Keep VoiceAssistant globally available but place its marked mobile wrapper in the unused top-right brand row, retaining desktop bottom-center behavior.

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

Add `data-global-floating-control="voice-assistant"` to the VoiceAssistant wrapper. Use mobile classes equivalent to `fixed top-2 right-4 bottom-auto left-auto w-auto max-w-none translate-x-0 px-0 flex-col-reverse`, overridden at `sm` by the existing bottom-center layout and `sm:flex-col`.

- [ ] **Step 3: Run collision test to verify it passes**

Run the Task 1 Playwright command.

Expected: 12 route/viewport cases pass at 320 and 390 with Voice Assistant as the only visible closed global trigger and zero collisions.

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
