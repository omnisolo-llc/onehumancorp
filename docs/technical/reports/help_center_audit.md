# Technical Audit Report: OneHumanCorp In-App Help Center

## Overview
This document represents a comprehensive architectural, styling, and design audit of the **OneHumanCorp (OHC) In-App Help Center**, focusing specifically on the visual layout, responsive mobile adapters, styling tokens, and automated test coverage of the principal Help components:
1. **Getting Started Page** (`src/app/help/getting-started-1/page.tsx`)
2. **Video Tutorials Page** (`src/app/help/videos/page.tsx`)

---

## 1. System Architecture & Component Mapping

The Help Center operates as a fully integrated, multi-channel user enablement tool targeting non-technical owners and operators. It leverages standard Next.js App Router mechanisms and custom client-side hooks to bridge localized storage, real backend proxies, and interactive widgets.

```
                    ┌────────────────────────┐
                    │      AppShell / "?"     │
                    └───────────┬────────────┘
                                │
                    ┌───────────▼────────────┐
                    │    Help Center Router   │
                    │      (/help/...)       │
                    └─────┬──────────────┬───┘
                          │              │
           ┌──────────────▼──────┐ ┌─────▼──────────────┐
           │ Getting Started Card │ │ Video Tutorials    │
           │  (getting-started-1)│ │  (videos/page.tsx) │
           └─────────────────────┘ └────────────────────┘
```

### Component Details
*   **Getting Started Component (`/help/getting-started-1/page.tsx`)**:
    *   **Purpose**: Actionable step-by-step guidance for brand-new OHC operators.
    *   **Data Source**: Hardcoded structured steps designed for offline resilience.
    *   **Routing**: Interactive router back-navigation integration via Next.js `useRouter`.
*   **Video Tutorials Component (`/help/videos/page.tsx`)**:
    *   **Purpose**: Unified gallery for short (<90 second) portrait-optimized training videos.
    *   **Data Source**: Dynamic query parameter parsing fetching via `/api/v1/videos?mobile_optimized=...` to adapt to flaky network states and viewport sizes.

---

## 2. Styling Tokens & Design Standards

OHC adheres to a sophisticated, restrained, and translucent styling vocabulary inspired by professional high-end macOS/UniFi control portals. The audited help components use these tokens meticulously to remain functional and beautiful on desktop as well as ultra-small screens.

### Design Tokens & Classes Applied:
*   **Blur & Glassmorphism**:
    *   `backdrop-blur-[30px] saturate-[210%]` - Produces soft, organic background transparency that mirrors underneath layers.
    *   `bg-white/65 dark:bg-[#16161a]/70` - High-contrast backdrop surfaces optimized for light and dark operating modes.
    *   `border-white/40 dark:border-white/10` - Thin, translucent crisp borders for depth rendering.
*   **Typography**:
    *   `font-outfit` - Elegant geometric sans-serif for high-visibility display titles and badge metrics.
    *   `font-inter` - High-legibility humanist sans-serif for dense help copy and prose structures.
*   **Colors**:
    *   `text-[#1D1D1F]` - Apple-standard dark charcoal body color.
    *   `text-[#0071E3]`, `bg-blue-50/50`, `border-blue-100/50` - Custom electric blue accents for interactivity and primary CTAs.

---

## 3. Responsive Mobile-First Breakpoints

To meet the absolute non-negotiables for Fatima (Food Cart Operator), Carlos (Field Service Owner), and Maya (Home Baker), the Help Center implements exact mobile scaling adapters:

*   **Touch Targets**: Every interactive button, list item, and router navigation element implements a minimum of `44x44px` padding box bounds (e.g., `.min-h-[44px]` on CTA buttons) to prevent misclicks on touch screens.
*   **Grid Adapters**:
    *   The layout utilizes flexible mobile-first CSS grids: `grid-cols-1 sm:grid-cols-2 lg:grid-cols-3` to wrap nicely on 375px screens without horizontal scrolls.
    *   Portrait-optimized aspect ratios (`aspect-[9/16]`) are enforced for portrait video players on mobile viewport scopes.
*   **Low-Data Fallback**: Static built-in fallback mock states are integrated natively into the help registry, allowing immediate local rendering if the endpoint fetches time out or error out due to unstable network connectivity.

---

## 4. Test Coverage & Verification Metrics

All help components are validated via automated unit tests running on top of **Vitest** and **Testing Library**. We maintain a strict quality standard ensuring zero regressions on route adapters.

### Verified Test Files & Assertions:
1.  **`src/app/help/getting-started-1/page.test.tsx`**:
    *   Verifies core heading visibility and structured walkthrough steps.
    *   Asserts back-to-home navigation pushes back to `/help`.
2.  **`src/app/help/videos/page.test.tsx`**:
    *   Asserts portrait-optimized video lists correctly load tutorial items.
    *   Verifies correct styling of aspect-ratio containers.
3.  **`src/app/api/v1/help/route.test.ts`**:
    *   Verifies telemetry and authentication credentials routing headers.
4.  **`src/components/Walkthrough.test.tsx`**:
    *   Checks target highlight overlays and speech-bubble layout positions.
