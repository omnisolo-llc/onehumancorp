# Issue Brief: Invisible AI-Driven Store Onboarding

## Title
Implement Invisible AI-Driven Store Onboarding

## Problem Statement
Small business owners, particularly non-technical users like "Maya (baker)" who currently sell via Instagram DMs, find traditional platform setups overwhelming. Shopify and Wix require users to manually configure settings, choose templates, and write copy before they can launch. This high friction leads to drop-offs and "blank canvas anxiety." Business owners want to start selling immediately, not become web designers.

## Research Report
Our analysis of App Store reviews and Reddit communities (r/smallbusiness, r/ecommerce) reveals that "I don't know where to start" and "setup is too confusing" are the most common complaints for new merchants. Competitors like Shopify offer "Sidekick" (a chatbot) and Wix offers "ADI" (template generation), but neither provides true autonomy. OHC has a unique opportunity to leverage its existing built-in agents (like Codex) to completely remove the setup burden, generating a functional store from a single plain-language prompt or brief conversation.

## Design Doc
```mermaid
graph TD
    A[User Setup Prompt] -->|Plain Language Input| B(Onboarding Agent)
    B -->|Generates State| C{OHC KAIROS Engine}
    C -->|Database Changes| D[PostgreSQL/SQLite]
    C -->|Asset Generation| E[Storage/CDN]
    D --> F[Live Storefront]
    E --> F

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

*   **Architecture:** The onboarding flow will trigger a background task managed by the `KAIROS Orchestration Engine`.
*   **Key Relationships:** The Onboarding Agent will interact with the `TenantRegistry` to create the tenant and populate initial schema data (products, settings).
*   **UI Flow:** A simple conversational UI ("What are you selling?") replaces complex forms. A loading screen with "glassmorphism" design tokens shows the agent "building" the store.
*   **Mobile UX:** Prioritize a 375px first design. Large touch targets, minimal text, clear progress indicators.

## Implementation Prompt
Implement a new onboarding flow powered by a dedicated AI agent. The user should only need to provide a basic description of their business (e.g., "I sell homemade cookies in Seattle"). The system must use this input to autonomously generate the initial store configuration, including a basic theme, sample product listings with AI-generated descriptions, and default shipping/payment settings.

The Critical User Journey (CUJ):
1.  User signs up and is presented with a simple chat interface.
2.  User describes their business in one sentence.
3.  The agent confirms and begins generation (displaying a premium loading state).
4.  The user is dropped into a fully functional, live storefront dashboard within 2 minutes.

Acceptance Criteria:
*   Onboarding must be completable without interacting with complex configuration forms.
*   The generated store must be immediately ready to accept a test order.
*   All UI elements must adhere to the "Plain Language Only" and "Business Owner Lens" constraints.

## Priority
P0

## Estimated Scope
Large
