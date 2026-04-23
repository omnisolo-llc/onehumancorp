# Plan: Implementation of Documentation Features for Non-Technical Users

1.  **Create Help Center Screen (`srcs/app/lib/screens/help_center_screen.dart`)**
    *   Build a responsive screen using `Scaffold` and `GlassCard` (for Glassmorphism).
    *   Include sections for: "Getting Started", "My Store", "Payments", "AI Agents", "Marketing", "Account & Billing".
    *   Include a search bar that visually filters the content.

2.  **Create Tooltip Registry Widget (`srcs/app/lib/widgets/help/tooltip_registry.dart`)**
    *   Create a reusable wrapper widget `ContextTooltip` that wraps UI elements with a `Tooltip` styled according to OHC Premium Glassmorphism tokens (`backdrop-filter: blur(20px)` via `BackdropFilter`, `Outfit` font, max 2 sentences plain language).
    *   Create a central registry class/map to hold the tooltip strings so they can be modified by agents without touching UI code.

3.  **Create AI Help Chat Widget (`srcs/app/lib/widgets/help/ai_help_chat_widget.dart`)**
    *   Implement a floating action button (FAB) that opens a Glassmorphism chat overlay.
    *   This chat will route to a specialized Help Agent, presenting "Ask anything" functionality. Includes mocked responses with "Read the full article →" links for E2E testing purposes.

4.  **Create Release Notes & Changelog Screen (`srcs/app/lib/screens/release_notes_screen.dart`)**
    *   Implement a "What's New" section showing recent updates in plain language with dummy screenshots/icons.

5.  **Create API Documentation Screen (`srcs/app/lib/screens/api_docs_screen.dart`)**
    *   Implement an "Advanced" section for technical users, presenting a mock interactive API reference using OHC Premium tokens.

6.  **Create Interactive Walkthrough Wrapper (`srcs/app/lib/widgets/help/interactive_walkthrough.dart`)**
    *   Implement a wrapper that provides an overlay highlight and speech bubble system for key flows ("Set up your store", etc.) using a simple state machine.

7.  **Integrate Documentation Features into `AppShell` and Router**
    *   Update `srcs/app/lib/router.dart` to add routes for `/help`, `/release-notes`, and `/api-docs`.
    *   Update `AppShell` in `router.dart` to include navigation items for these new screens and inject the `AI Help Chat Widget` FAB globally.

8.  **Inject Context Tooltips into Existing Screens**
    *   Update `dashboard_screen.dart` and `agents_screen.dart` to replace standard `Tooltip`s with the new `ContextTooltip` from the registry.

9.  **Write E2E Test (`srcs/app/e2e/documentation.spec.ts`)**
    *   Write a Playwright test simulating a user logging in, navigating to the Help Center, opening the AI Help Chat, checking the Release Notes, and checking for tooltips on the Dashboard. Asserting final product state.

10. **Write Unit Tests (`srcs/app/lib/screens/help_center_test.dart`, etc.)**
    *   Add widget tests to ensure 100% coverage of the new documentation code.

11. **Run Verifications**
    *   Run `bazelisk test //...` to ensure all tests pass (Backend + Frontend).

12. **Pre-commit Steps**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
