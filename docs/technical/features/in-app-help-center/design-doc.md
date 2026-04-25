<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Doc: In-App Help Center & Documentation Architecture

**Author(s):** Principal Technical Writer & Scribe (L7)
**Status:** Draft
**Last Updated:** 2026-03-20

## 1. Overview
The Help Center and context-aware documentation system enables non-technical small business owners to seamlessly navigate and operate OneHumanCorp (OHC) without needing external support. This includes an in-app help center, contextual tooltips, interactive walkthroughs, and an AI-powered help chat.

## 2. Goals & Non-Goals
### 2.1 Goals
- **In-App Help Center:** A searchable portal with articles on Getting Started, Store, Payments, AI Agents, etc.
- **Contextual Tooltips:** A tooltip registry and global component for non-obvious UI elements.
- **Interactive Walkthroughs:** Step-by-step in-app tours for key flows using overlay highlights and speech bubbles.
- **AI-Powered Help Chat:** A floating chat button connected to a specialized Help Agent.
- **Video Tutorials & Release Notes:** Embedded <90s videos and plain-language release notes.

### 2.2 Non-Goals
- Replacing standard documentation (API docs will be a separate advanced section).

## 3. Implementation Details
- **Frontend Architecture (Flutter):**
  - Integrate a new `HelpCenterScreen` accessible from a global `?` button.
  - Create a generic `ContextualTooltip` widget that reads from a `TooltipRegistry`.
  - Build an `InteractiveWalkthroughOverlay` to drive step-by-step onboarding without modals.
  - Add a floating action button globally for the `AI Help Chat`.

- **Backend Architecture (Go/Bazel):**
  - Implement a registry/API for help articles and video metadata.
  - Integrate the Help Agent with Gemini Pro to query help center content.

## 4. Execution Plan
- Phase 1: Build documentation data models and tooltips infrastructure.
- Phase 2: Implement UI for Help Center, Walkthroughs, and Help Chat.
- Phase 3: Testing and E2E verification across mobile and desktop.

</div>
