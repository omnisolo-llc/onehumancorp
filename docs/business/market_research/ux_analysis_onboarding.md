# Static UX Analysis: Onboarding Flows

This document contains a static analysis of the legacy Next.js onboarding wizards, comparing them to the OHC Premium Token library design standards.

## Evaluated Surfaces

- `src/ui/next/src/app/onboarding/page.tsx`
- `src/ui/next/src/app/globals.css`
- `src/ui/next/src/app/website-builder/page.tsx`
- `src/ui/next/src/app/business-setup/page.tsx`
- `src/server/services/onboarding/onboarding_agent.rs`

## UX and UI Analysis against OHC Premium Standards

### The Translucent Glass Mandate
**Requirement**:
Containers and sidebars must adopt premium, vibrant, translucent macOS-style Glass materials:
- Light Mode Translucent Glass: `background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px) saturate(210%)`, with an ultra-thin border highlight `border: 1px solid rgba(255, 255, 255, 0.4)`.
- Dark Mode Translucent Glass: `background: rgba(22, 22, 26, 0.7)`, `backdrop-filter: blur(30px) saturate(210%)`, with a delicate border highlight `border: 1px solid rgba(255, 255, 255, 0.1)`.
- Rounded corners must follow premium Apple macOS/UniFi curves: `8px` for buttons/controls/inputs, `16px` for container cards.

**Findings**:
The `.glassmorphism` class defined in `globals.css` and used throughout `onboarding/page.tsx` and `website-builder/page.tsx` largely met the transparency, filter, and border color standards.
However, it lacked the explicit `border-radius: 16px;` declaration directly on the glass material class, meaning containers had to individually specify `rounded-[16px]`. This is redundant and error-prone.
This was addressed by adding `border-radius: 16px;` natively to `.glassmorphism` in `globals.css`.

### Mobile-First Layouts and Touch Targets
**Requirement**: All wizards and steppers must be 100% usable on a 375px-wide phone screen. Touch targets must be at least 44x44px.

**Findings**:
- **Onboarding Wizard** (`onboarding/page.tsx`): Successfully employs a responsive flex-col layout. It sets `min-h-[54px]` on virtually all text inputs, textareas, and buttons. This comfortably exceeds the 44px minimum height touch target.
- **Website Builder Wizard** (`website-builder/page.tsx`): The preview mode overlay specifically indicates a 375px width design target. The bottom action buttons (e.g. `1-Tap Launch`) are full-width block buttons with adequate padding, satisfying the 44px constraint.
- Overall, the inputs and buttons have robust `rounded-[8px]` corners across both Next.js wizard flows, ensuring adherence to the control styling standards.

### Typography and Colors
**Requirement**: Primary accents should be `#0066FF`, connected statuses `#34C759`, warning statuses `#FF9500`, and critical `#FF3B30`. Core text colors `#1D1D1F` (Light) and `#F5F5F7` (Dark).

**Findings**:
`onboarding/page.tsx` consistently maps:
- `text-[#1D1D1F] dark:text-[#F5F5F7]` on headings and text inputs.
- `bg-[#0066FF]` for primary CTAs (e.g., "Start Onboarding", "Next").
- `bg-[#34C759]` and `text-[#34C759]` for auto-configured agent badges and success markers ("You're Live!").
- `border-red-500` for form validation errors, matching the critical error context but diverging slightly from the specific `#FF3B30` hex token.

## Backend Analysis: `onboarding_agent.rs`

**Architecture & Caching**:
- The agent utilizes `HybridCache` to locally cache onboarding progress using Redis.
- `get_onboarding_state` reads from the cache and falls back to a PostgreSQL database if not present.
- `save_onboarding_state` mutates the PostgreSQL database and subsequently invalidates the cache key.
- *Improvement made*: Tracing was insufficient for debugging race conditions or cache misses during multi-device stepper synchronization. We added `tracing::debug!` statements mapping cache hits and invalidations directly within `get_onboarding_state` and `save_onboarding_state`.

**AI Integration**:
- Heavy reliance on `minimax.reason()` for parsing conversational inputs into structured JSON payloads (`IntakeData`). The fallback is rudimentary string parsing and length clipping if the LLM request fails.

## Infrastructure Notes
- *Next.js Blockers*: Attempting to verify the UI dynamically resulted in build failures (`Cannot find module '@tailwindcss/postcss'`). A broader migration/audit to Tauri v2 is underway, rendering the Next.js routes legacy.
- *Docker Blockers*: Rebuilding and loading local images was obstructed by extraction permissions issues inside `pgvector/pgvector:pg15`.
