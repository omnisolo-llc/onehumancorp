# Mobile floating controls collision design

## Goal

Prevent the three root-layout floating controls from obscuring product content on 320 px and 390 px viewports while preserving desktop behavior and mobile access to unique voice functionality.

## Architecture

- `HelpWidget` and `HelpChat` remain unchanged on desktop and hide their closed floating triggers below the Tailwind `sm` breakpoint. AppShell already exposes a keyboard-accessible Help Center action on every audited product page.
- `VoiceAssistant` remains bottom-centered on desktop. Below `sm`, its closed control moves to the unused top-right brand-row space, outside the centered brand, compact navigation, and page topbar. The wrapper receives an explicit `data-global-floating-control` marker for collision verification.
- Open dialogs/status surfaces retain their existing ownership and behavior; the change applies only to closed global triggers.

## Inbox correction

Remove the literal `\n` text node between the inbox queue and conversation-detail sections. A component test verifies that the settled empty state never renders a literal backslash-n.

## Verification

- Add a Playwright regression covering `/website-builder`, `/login`, `/agent-marketplace`, `/integrations`, `/agents`, and `/inbox` at 320 px and 390 px. It checks every visible marked global floating control against visible interactive elements and key shell regions, excluding the control and its own descendants.
- Run the affected unit tests, focused collision Playwright test, full Vitest, standalone TypeScript, production build, production 43-test Playwright matrix, secured 36-case visual audit, and inspect affected original-resolution screenshots.
- Preserve all existing shell-count, radius, overflow, hydration, and offline assertions.
