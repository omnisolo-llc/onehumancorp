# Mobile floating controls collision design

## Goal

Prevent help and voice controls from obscuring product content on 320 px and 390 px viewports, including sticky-header scroll positions and active voice states, while preserving desktop behavior and mobile voice access.

## Architecture

- `HelpWidget` and `HelpChat` remain unchanged on desktop and hide their closed floating triggers below the Tailwind `sm` breakpoint. AppShell already exposes a keyboard-accessible Help Center action on every audited product page.
- `VoiceAssistant` has one owner: `AppShell`. RootLayout no longer renders it. On mobile, its trigger and listening, processing, error, and success surfaces participate in the topbar action region's normal document flow, allowing the sticky topbar to expand without covering navigation or product content. On desktop, responsive classes preserve its prior bottom-centered fixed presentation.
- Stable, non-sensitive data markers identify the voice root, trigger, and every active status surface. Active panels have a stable mobile width and cannot shift the trigger horizontally.

## Voice media lifetime and input contract

- `VoiceAssistant` owns mounted, pending-request, recorder, stream, recording, and timeout refs. One idempotent stream-release helper stops each acquired track at most once and clears the owned stream reference. Normal stop releases tracks immediately; recorder completion may safely call the same helper again.
- Unmount invalidates pending media work before detaching recorder callbacks, stopping an active recorder when possible, releasing every acquired track, clearing refs, and cancelling status-reset timers. Recorder callbacks and voice-command responses check the mounted/session guard before sending audio, dispatching events, or updating React state. A media stream that resolves after invalidation is released immediately and never reaches `MediaRecorder` construction or `start()`.
- Physical mouse/touch input remains press-and-hold. Non-repeating Enter and Space keydown/keyup provide the same hold-to-talk behavior and prevent browser defaults. Pointer-generated clicks are ignored; a `detail === 0` synthetic activation toggles recording for assistive technology, with keyboard-generated synthetic clicks consumed so they cannot double-trigger the key handlers.
- The trigger exposes recording state with `aria-pressed` and a dynamic label that explains hold or toggle operation. Every visible state surface is an atomic live `status`; routine states are polite and errors are assertive. Media/provider failures use fixed diagnostic messages and never log raw error objects.

## Inbox correction

Remove the literal `\n` text node between the inbox queue and conversation-detail sections. A component test verifies that the settled empty state never renders a literal backslash-n.

## Verification

- Add Playwright regressions covering `/website-builder`, `/login`, `/agent-marketplace`, `/integrations`, `/agents`, and `/inbox` at 320 px and 390 px, initially and after vertical scroll on tall routes. Exercise listening, processing, error, and success states with browser-local media/API mocks. Every visible voice surface must remain within the sticky topbar and viewport, avoid sibling actions and product content, and never shift the trigger unexpectedly.
- Add component assertions for a single shell-owned instance, responsive normal-flow/fixed classes, state markers, accessibility, and focus behavior.
- Add component regressions for normal track release, unmount during recording, late media acquisition after unmount, callback suppression, exact-once recorder/track cleanup, Enter/Space hold behavior, repeat suppression, assistive synthetic-click toggling, pointer behavior, and live-state announcements.
- Run the affected unit tests, focused collision Playwright tests, full Vitest, standalone TypeScript, production build, expanded production Playwright matrix, secured 36-case visual audit, and inspect initial/scrolled/active original-resolution screenshots.
- Preserve all existing shell-count, radius, overflow, hydration, and offline assertions.
