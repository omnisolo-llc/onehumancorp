# Scribe Documentation Features

## Objective
Implement documentation features for the One Human Corp product to reduce support tickets by answering user questions from an owner/operator lens, using plain language.

## Key Features

1.  **Contextual Tooltips (`Tooltip Registry`)**
    -   Every non-obvious UI element must have a contextual tooltip.
    -   Hover on desktop, long-press on mobile.
    -   Plain language, max 2 sentences.
    -   Registry to allow agents to add/update without touching UI code.

2.  **In-App Help Center**
    -   Searchable portal accessible from a "?" button.
    -   Topics: Getting Started, My Store, Payments, AI Agents, Marketing, Account & Billing.
    -   Mobile-first design.

3.  **Interactive Walkthroughs**
    -   Step-by-step tours for key flows (e.g., "Set up your store", "Accept your first payment", "Activate your AI Support Agent").
    -   Overlay highlight + speech bubble system (no popups or modals).

4.  **AI-Powered Help Chat**
    -   Floating "Ask anything" chat button.
    -   Routes to Help Agent using help center context.
    -   Includes "Read the full article ->" links.

5.  **Video Tutorials**
    -   Short (< 90s) videos for top 10 common tasks.
    -   Mobile-accessible, portrait-optimized player.

6.  **API Documentation (Advanced Users)**
    -   Interactive API reference (OpenAPI/Swagger UI).
    -   Separated into an "Advanced" section.

7.  **Release Notes & Changelog**
    -   "What's New" section in the app.
    -   Plain language with screenshots.
    -   Links to full changelog.

## Implementation Details (Interactive Walkthroughs)
This task focuses specifically on addressing the issue with the Interactive Walkthroughs, as it is a core documentation feature that was partially refactored but broke existing flows due to missing references and un-updated UI files.

### Context
`safe-help-content.mjs` was introduced to safely render walkthrough controls to avoid interpolation/XSS issues, replacing inline HTML generation. However, many HTML files (e.g., `success.html`, `workflow-builder.html`, `setup.html`, `storefront.html`, `triage.html`, `trial-extension.html`) still rely on the old inline generation and hardcoded IDs (like `wt-close`, `wt-prev`, `wt-next`).

### Changes required:
1.  **Update HTML files to use `renderWalkthroughStep`:**
    Modify the `renderStep` (or equivalent) function in affected UI files to:
    -   Import `renderWalkthroughStep` from `safe-help-content.mjs`.
    -   Pass the bubble element, current step, current step index, and total step count to `renderWalkthroughStep`.
    -   Attach event listeners (`close`, `previous`, `next`) using the returned control elements.
2.  **Ensure OHC Premium Design Standards:**
    The generated walkthrough bubbles must use the Translucent Glass materials:
    -   Light mode uses `background: rgba(255, 255, 255, 0.65)` with `border: 1px solid rgba(255, 255, 255, 0.4)`.
    -   Rounded corners (`16px` for cards/bubbles).
3.  **Ensure Mobile First/Accessibility:**
    -   Use `aria-label` where appropriate.
    -   Ensure buttons meet hit target requirements (min-height 44px).
4.  **Verify Test Suite:**
    -   Run `make test` and `make lint` to ensure no regressions.

## Future Work (Other Features)
- Implement Tooltip Registry.
- Implement In-App Help Center search functionality.
- Implement AI-Powered Help Chat routing.
