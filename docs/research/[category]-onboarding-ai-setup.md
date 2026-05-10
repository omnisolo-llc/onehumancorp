# AI-Driven Store Setup Wizard
## Title
Conversational AI Store Onboarding Wizard

## Problem Statement
New business owners, such as Priya (a boutique owner), face a steep learning curve when moving online. Building a website on platforms like Shopify or Squarespace requires understanding themes, layouts, navigation, and product configurations. This technical complexity causes high drop-off rates during setup. SMB owners want to answer a few simple questions and have the platform generate a complete, ready-to-sell store for them instantly.

## Research Report
*   **Competitor Landscape:**
    *   **Shopify:** Setup is manual and can be confusing. "Sidekick" helps answer questions but doesn't autonomously build the store.
    *   **Wix ADI / GoDaddy Airo:** Offer basic AI generation, but the results are often generic and still require significant manual tweaking. They generate the *look*, but not always the complete business logic (products, pricing, shipping rules).
    *   **Durable:** Very fast generation, but lacks deep business management tools post-launch.
*   **User Pain Points:** YouTube tutorials and Reddit threads highlight "getting started" and "designing the homepage" as the biggest hurdles. The "blank canvas" problem paralyzes users.
*   **OHC Differentiation:** OHC must provide a setup experience that not only designs the UI but configures the backend (products, payment logic, booking types) through a natural language conversation, living up to the "launch in 10 minutes" vision.

## Design Doc
**High-level Architecture:**
*   **Conversational Interface:** A chat-like UI guiding the user through the setup.
*   **Agent Orchestration:** The wizard translates natural language ("I sell vintage clothes") into KAIROS tasks that generate products, descriptions, and a theme.
*   **Theming Engine:** Dynamic generation of Slint/Web UI components based on the AI's understanding of the brand's vibe.

**UI Flow (Mobile First - 375px):**
1.  **Welcome Screen:** "Let's build your business. What do you do?" (Text input or voice dictation).
2.  **Conversational Prompting:** The AI asks 3-4 targeted questions (e.g., "Do you sell physical items or services?", "What's the vibe: modern, playful, or elegant?").
3.  **Generation Phase:** A visually engaging loading screen ("Agentic Storefronts" at work).
4.  **Reveal & Refine:** The user is presented with a fully functional store preview. They can accept it or tap "Make it more professional" to trigger an AI refinement pass.

## Implementation Prompt
Implement an AI-driven setup wizard that builds a complete, functional store from a brief conversation.
**Critical User Journey (CUJ):**
1. The user creates a new OHC account.
2. They are greeted by the AI Wizard and describe their business in plain language.
3. The wizard asks a few clarifying questions.
4. The system automatically provisions the store, generates sample products/services based on the description, and applies a suitable theme.
5. The user is dropped into the main dashboard with a live store, ready to connect a payment gateway.

**Acceptance Criteria:**
* The wizard must be conversational and avoid technical jargon (Business Owner Lens).
* The output must include not just a layout, but actual database entries (e.g., generated sample products) correctly associated with the new `tenant_id`.
* The entire flow must be smooth on a 375px screen.
* Must implement Progressive Disclosure: simple conversational setup, with advanced manual configuration hidden until after the initial store is generated.

## Priority
P0

## Estimated Scope
Medium
