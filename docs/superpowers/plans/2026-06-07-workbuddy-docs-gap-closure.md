# WorkBuddy Docs Gap Closure Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the official WorkBuddy docs-backed feature gaps that were not represented in the old 150-item parity registry.

**Architecture:** Extend the existing `/api/assistant` in-memory contract and `/assistant` workstation UI instead of adding a second system. Add typed fields and actions for connector roster, model capabilities, settings toggles, account/sidebar metadata, share/data lifecycle, feedback attachments, automation schedule kinds, and parity registry expansion.

**Tech Stack:** Next.js app router, React, TypeScript, Vitest, Testing Library.

---

### Task 1: API Gap Contract

**Files:**
- Modify: `src/ui/next/src/app/api/assistant/route.test.ts`
- Modify: `src/ui/next/src/app/api/assistant/store.ts`
- Modify: `src/ui/next/src/app/api/assistant/share/route.ts`

- [ ] **Step 1: Write failing API tests**

Add expectations for:
- default connector roster: GitHub, GitLab, Jira, Confluence, Google Drive, Gmail, Notion, Slack
- model capability flags and `customProtocol`
- settings: `compactMode`, `autoInstallLowRiskSkills`, `preventSleep`, `profile`, `version`
- share lifecycle: copy link, download, revoke/cancel sharing
- data lifecycle: unarchive archived task
- support ticket screenshot attachment
- automation schedule kinds: hourly, daily, weekly, one-time
- parity total expanded beyond 150 with new docs-gap categories

Run: `cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts`
Expected: FAIL because these fields/actions are missing.

- [ ] **Step 2: Implement minimal API support**

Update assistant types, seeds, mutators, and routes so the new tests pass.

- [ ] **Step 3: Verify API green**

Run: `cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts`
Expected: PASS.

### Task 2: UI Gap Surface

**Files:**
- Modify: `src/ui/next/src/app/assistant/page.test.tsx`
- Modify: `src/ui/next/src/app/assistant/page.tsx`
- Modify: `src/ui/next/src/app/assistant/assistant.module.css`

- [ ] **Step 1: Write failing UI tests**

Assert the assistant page renders:
- full official connector roster
- model capability/custom protocol cards
- compact mode, auto-install low-risk skills, prevent sleep, account/version cards
- copy/download/cancel sharing and unarchive controls
- updated parity count/category names

Run: `cd src/ui/next && npm test -- src/app/assistant/page.test.tsx`
Expected: FAIL because the UI still has static/partial cards.

- [ ] **Step 2: Implement minimal UI support**

Render missing WorkBuddy affordances through existing panels and wire buttons to the existing action dispatcher.

- [ ] **Step 3: Verify UI green**

Run: `cd src/ui/next && npm test -- src/app/assistant/page.test.tsx`
Expected: PASS.

### Task 3: Focused Regression

**Files:**
- Verify: `src/ui/next/src/app/api/assistant/route.test.ts`
- Verify: `src/ui/next/src/app/assistant/page.test.tsx`
- Verify: `src/ui/next/src/app/agents/page.test.tsx`

- [ ] **Step 1: Run focused assistant tests**

Run: `cd src/ui/next && npm test -- src/app/api/assistant/route.test.ts src/app/assistant/page.test.tsx`
Expected: PASS.

- [ ] **Step 2: Run Expert Center regression tests**

Run: `cd src/ui/next && npm test -- src/app/agents/page.test.tsx`
Expected: PASS.
