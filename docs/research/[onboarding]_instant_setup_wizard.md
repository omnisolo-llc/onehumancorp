# Title
Proactive Setup Wizard: Instant Storefront Generation for Complete Beginners

# Problem Statement
Small business owners like Maya (The Home Baker) and Carlos (The Handyman) are often paralyzed by the technical complexity of existing platforms like Shopify and Wix. They do not know what DNS, liquid templates, or complex shipping zones are. They want to sell their services or goods quickly but are deterred by the "Setup Complexity" which affects 73% of users, leading to a high drop-off rate before the store even goes live. They need a system that builds itself based on plain language inputs without exposing the underlying infrastructure.

# Research Report
- **Target Personas**: Maya (Baker, 28) - sells via Instagram DMs, overwhelmed by Shopify; Carlos (Handyman, 42) - word-of-mouth only, no website.
- **Pain Points Addressed**: Setup Complexity (Rank 1, 73% frequency), Technical Jargon (Rank 5, 48% frequency).
- **Competitor Analysis**: Shopify takes 30m+ with high friction and no real AI setup. Wix takes 20m+ with some AI assistance. Durable is fast (< 1m) but lacks business management depth. OHC must capture Durable's speed while providing robust operational tools.
- **Evidence**: Reddit (r/shopify) users complain: "Why do I need to know what a CNAME record is just to sell a t-shirt?"
- **AI Differentiation**: The setup process should not be a traditional form but an interactive session with the "Silent Ambassador" agent.

# Design Doc
- **High-Level Architecture**:
  - **Entity Types**: `Tenant`, `StorefrontTheme`, `Product`, `ServiceBooking`.
  - **Key Relationships**: A `Tenant` has one active `StorefrontTheme` generated upon onboarding. `Product` and `ServiceBooking` are seeded by the generative agent.
  - **Integration Points**: Agent Orchestrator to trigger generative UI and text models during the onboarding wizard flow.
- **UI Wireframes/Screen Flow Description**:
  - Welcome Screen (Mobile 375px first): Simple text input - "What do you do?"
  - Loading State: "Glassmorphism" progress rings while agents build the store, fetch local imagery, and generate copy.
  - Reveal Screen: A fully functional store preview on mobile with a single "Launch" button. No settings menus are visible by default.
- **AI Agent Integration Points**: The onboarding agent orchestrates a series of parallel sub-tasks: writing SEO-optimized copy, selecting themes, generating sample inventory based on the business type, and pre-configuring a simple checkout flow.

# Implementation Prompt
Implement a new onboarding flow in the Slint UI for the desktop and mobile client that bypasses traditional multi-step forms. The flow should present a single text input asking the user to describe their business. Upon submission, it should trigger the backend generative agents to assemble a complete storefront profile (copy, styling, initial products/services) and return a live preview. The critical user journey (CUJ) is going from "New Account" to "Viewing a Fully Populated Storefront Preview" in under 1 minute without seeing a single technical setting or jargon term. Acceptance criteria include the successful generation of a storefront from a plain text prompt and the absence of complex configuration steps in the initial path.

# Priority
P0

# Estimated Scope
Large
