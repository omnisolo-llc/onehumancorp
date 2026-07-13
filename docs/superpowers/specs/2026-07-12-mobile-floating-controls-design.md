# Mobile floating controls collision design

## Goal

Prevent help and voice controls from obscuring product content on 320 px and 390 px viewports, including sticky-header scroll positions and active voice states, while preserving desktop behavior and mobile voice access.

## Architecture

- `HelpWidget` and `HelpChat` remain unchanged on desktop and hide their closed floating triggers below the Tailwind `sm` breakpoint. AppShell already exposes a keyboard-accessible Help Center action on every audited product page.
- `VoiceAssistant` has one owner: `AppShell`. RootLayout no longer renders it. On mobile, its trigger and listening, processing, error, and success surfaces participate in the topbar action region's normal document flow, allowing the sticky topbar to expand without covering navigation or product content. On desktop, responsive classes preserve its prior bottom-centered fixed presentation.
- Stable, non-sensitive data markers identify the voice root, trigger, and every active status surface. Active panels have a stable mobile width and cannot shift the trigger horizontally.

## Inbox correction

Remove the literal `\n` text node between the inbox queue and conversation-detail sections. A component test verifies that the settled empty state never renders a literal backslash-n.

## Verification

- Add Playwright regressions covering `/website-builder`, `/login`, `/agent-marketplace`, `/integrations`, `/agents`, and `/inbox` at 320 px and 390 px, initially and after vertical scroll on tall routes. Exercise listening, processing, error, and success states with browser-local media/API mocks. Every visible voice surface must remain within the sticky topbar and viewport, avoid sibling actions and product content, and never shift the trigger unexpectedly.
- Add component assertions for a single shell-owned instance, responsive normal-flow/fixed classes, state markers, accessibility, and focus behavior.
- Run the affected unit tests, focused collision Playwright tests, full Vitest, standalone TypeScript, production build, expanded production Playwright matrix, secured 36-case visual audit, and inspect initial/scrolled/active original-resolution screenshots.
- Preserve all existing shell-count, radius, overflow, hydration, and offline assertions.
