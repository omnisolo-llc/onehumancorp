# Jarvis Assistant Vertical Slice Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Build the primary `/assistant` Jarvis workstation surface with tested API contracts for tasks, artifacts, changes, remote control, automations, memory, skills, connectors, and Expert Center navigation.

**Architecture:** Add a focused Next.js vertical slice under `src/ui/next/src/app/assistant` and `src/ui/next/src/app/api/assistant`. The API uses an in-memory store as the first durable contract for UI and tests; later server/Rust/Tauri integrations can replace storage behind the same response shapes. The UI is a client workstation layout with task list, conversation, composer, results tabs, and utility panels.

**Tech Stack:** Next.js app router, React, TypeScript, Vitest, Testing Library.

---

### Task 1: Assistant API Contract

**Files:**
- Create: `src/ui/next/src/app/api/assistant/store.ts`
- Create: `src/ui/next/src/app/api/assistant/tasks/route.ts`
- Create: `src/ui/next/src/app/api/assistant/remote/route.ts`
- Create: `src/ui/next/src/app/api/assistant/automations/route.ts`
- Create: `src/ui/next/src/app/api/assistant/memory/route.ts`
- Test: `src/ui/next/src/app/api/assistant/route.test.ts`

- [x] **Step 1: Write failing API tests**

Create tests for task creation/listing, remote intake, automation creation, and memory edit/forget/import. The first run should fail because the modules do not exist.

- [x] **Step 2: Run API tests and verify RED**

Run: `cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts`
Expected: FAIL with missing module errors for assistant API routes.

- [x] **Step 3: Implement in-memory assistant API store and routes**

Implement typed task, artifact, change, automation, remote, and memory helpers. Keep response shapes stable and small.

- [x] **Step 4: Run API tests and verify GREEN**

Run: `cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts`
Expected: PASS.

### Task 2: Primary `/assistant` UI

**Files:**
- Create: `src/ui/next/src/app/assistant/page.tsx`
- Test: `src/ui/next/src/app/assistant/page.test.tsx`

- [x] **Step 1: Write failing UI tests**

Create tests asserting the workstation renders, links to `/agents`, exposes WorkBuddy-parity controls, submits a task payload, updates task/results state, and handles remote/memory/automation panels.

- [x] **Step 2: Run UI tests and verify RED**

Run: `cd src/ui/next && npm test -- src/app/assistant/page.test.tsx`
Expected: FAIL with missing route/component.

- [x] **Step 3: Implement the `/assistant` page**

Build a client component with left task rail, center conversation/composer, right results tabs, and utility panels. Use existing app styling patterns without deleting `/agents`.

- [x] **Step 4: Run UI tests and verify GREEN**

Run: `cd src/ui/next && npm test -- src/app/assistant/page.test.tsx`
Expected: PASS.

### Task 3: Integration Verification

**Files:**
- Verify: assistant API tests
- Verify: assistant UI tests
- Verify: existing `/agents` tests remain compatible

- [x] **Step 1: Run focused assistant tests**

Run: `cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts src/app/assistant/page.test.tsx`
Expected: PASS.

- [x] **Step 2: Run Expert Center regression tests**

Run: `cd src/ui/next && npm test -- src/app/agents/page.test.tsx`
Expected: PASS.

- [x] **Step 3: Commit implementation**

Run: `git add docs/superpowers/plans/2026-06-07-jarvis-assistant-vertical-slice.md src/ui/next/src/app/api/assistant src/ui/next/src/app/assistant && git commit -m "feat: add jarvis assistant surface"`
Expected: Commit succeeds.

## Scope Notes

This plan implements the tested primary app surface and API contracts across all WorkBuddy/Jarvis capability categories. It does not claim native desktop file authorization, real Slack/Telegram/Discord bot deployment, persistent database migrations, or full DOCX/PPTX/PDF generation engines are complete. Those are backend integration tasks that should attach to the contracts created here.
