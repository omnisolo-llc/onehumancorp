# [Onboarding] AI-Driven 1-Tap Storefront Generation

## Title
AI-Driven 1-Tap Storefront Generation for Complete Beginners

## Problem Statement
Small business owners like Maya (a 28-year-old baker) are overwhelmed by the technical complexity of existing platforms like Shopify and Wix. Setting up a storefront requires understanding complex navigation, configuring payment gateways manually, and writing extensive copy. These users abandon the process because the initial hurdle of turning their idea into a live, professional-looking online presence takes hours or days instead of minutes. They need a system that does the heavy lifting invisibly, requiring only their approval, not their labor.

## Research Report
*   **Competitor Failures:** Shopify is the industry standard but is notoriously complex for beginners. It functions well for established stores but fails the "instant setup" test. Wix provides templates but still requires a "drag-and-drop" mentality that intimidates non-technical users.
*   **Emerging Threats:** Platforms like Durable demonstrate that AI can generate a website in 30 seconds. However, these platforms fall short on robust business management (POS, inventory, booking).
*   **User Evidence:** Based on analysis of App Store and Trustpilot reviews, a significant pattern emerges: users trying to transition from "Instagram DMs" to a real store frequently abandon the process due to setup friction.
*   **Opportunity:** OHC has the opportunity to leapfrog by combining instant AI website generation (like Durable) with powerful, invisible backend agents that automatically configure the business logic (like Shopify).

## Design Doc
*   **High-Level Architecture:**
    *   **Entity Types:** `BusinessIntent`, `GeneratedDraft`, `LiveStorefront`.
    *   **Key Relationships:** A user submits a `BusinessIntent` (e.g., natural language description). The AI agents consume this to produce a `GeneratedDraft`. Upon user approval, this draft is promoted to a `LiveStorefront`.
    *   **Integration Points:** OHC's internal LLM routing gateway interacts with the `AutoDream` agent to translate the intent into structured JSON representing the initial store configuration, copy, and layout.
*   **Mobile UX Flow (375px First):**
    *   *Screen 1 (The Intake):* A clean, single-input conversational UI. "What kind of business are you starting?"
    *   *Screen 2 (The Magic):* An engaging loading state (<=300ms entrance animation) explaining what the AI is doing ("Writing descriptions...", "Setting up payments...", "Designing layout...").
    *   *Screen 3 (The Reveal):* A fully generated, functional storefront preview. The only CTA is a prominent "Approve & Launch" button.

## Implementation Prompt
*   **User-Facing Outcome:** A non-technical user can type a single sentence describing their business and receive a complete, ready-to-launch mobile storefront in under 30 seconds.
*   **Critical User Journey (CUJ):** The user downloads the OHC app -> Enters business description -> Waits <30s -> Views the generated storefront draft -> Taps "Approve & Launch" -> Store is live.
*   **Acceptance Criteria:**
    *   The intake UI must be entirely conversational; no complex forms or settings menus during onboarding.
    *   The generated output must include AI-written product descriptions, placeholder images, and a functional (sandbox) checkout flow.
    *   The generated UI must strictly adhere to OHC's Glassmorphism UI and Outfit/Inter typography standards.
    *   The entire process must complete in under 30 seconds.

## Priority
P0

## Estimated Scope
Large