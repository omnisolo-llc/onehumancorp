# OHC In-App Help Center Design Document

## 1. Overview
The documentation infrastructure for OHC is built around plain-language assistance embedded directly in the app. The target audience has zero technical knowledge, so every tooltip and help article must use business-first language (e.g., "Ask anything" instead of "Support bot query endpoint").

## 2. Architecture

### 2.1 Help Center Screen (`help_center_screen.dart`)
- Accessible via the sidebar or global `?` button.
- Contains a list of standard plain-language articles grouped into categories:
  - Getting Started
  - My Store
  - Payments
  - AI Agents
  - Marketing
  - Account & Billing
- Provides a Floating Action Button (FAB) reading "Ask anything" that directs users to the Chat screen `/chat` so the Help Agent can answer customized queries.

### 2.2 Tooltip Registry (`ohc_tooltip.dart`)
- A reusable component wrapping the default Flutter `Tooltip`.
- Uses OHC Premium Tokens: Glassmorphism (`backdrop-filter: blur(20px)` if applicable) or appropriate colors, padding, and text styles.
- Behavior: Hover on desktop, long-press on mobile.

### 2.3 Interactive Walkthroughs (`interactive_tour.dart`)
- An architecture for step-by-step in-app tours.
- Implemented using an overlay highlight system combined with `GlobalKey` targets.
- No blocking popups or modals.

## 3. Scope of Implementation
This initial PR will implement:
- The design document itself.
- `OhcTooltip` component and its usage in `dashboard_screen.dart` and `agents_screen.dart`.
- `HelpCenterScreen` with placeholder content.
- Update `AppShell` routing to expose the Help Center.
- `InteractiveTour` base widget and structure.
- Unit and E2E Tests for the Help Center.
