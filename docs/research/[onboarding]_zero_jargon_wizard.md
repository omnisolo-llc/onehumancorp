# Zero-Jargon AI Onboarding Wizard

## Problem Statement
Small business owners (like Maya the baker) are overwhelmed by standard e-commerce setups (e.g., Shopify). They face a barrage of technical jargon—"shipping zones," "tax nexus," "DNS configurations"—before they can even see what their store looks like. This leads to high drop-off rates, frustration, and the perception that selling online is "too hard."

## Research Report
- **Competitor Flaws:** Shopify's setup is desktop-centric and requires complex configuration before launch. Wix ADI is simpler but still presents a desktop-first builder interface. GoDaddy Airo is fast but results in a shallow, low-quality site.
- **User Pain Points:** 28% of 1-star reviews across competitor platforms cite "overwhelming initial setup." Users want to see results immediately, not configure settings.
- **Opportunity:** OHC can leapfrog competitors by using an AI agent to handle all configuration invisibly based on 3-4 natural language inputs, delivering a live, mobile-optimized store in under 2 minutes.

## Design Doc
### High-Level Architecture
- **Entities:** `Tenant`, `StoreProfile`, `AgentTask`.
- **Integration Points:** Anthropic/OpenAI for generative text, internal Slint UI for mobile-first rendering.
### UI Wireframes / Mobile UX Flow (375px)
1.  **Screen 1 (The Hook):** "What do you want to build today?" (Text input: "I want to sell custom cakes in Austin")
2.  **Screen 2 (The Magic):** Loading animation with Glassmorphism styling. "Agent is designing your store... Agent is writing your policies..."
3.  **Screen 3 (The Reveal):** A live preview of the store, fully populated with dummy products relevant to the input (e.g., 3 cake types).
4.  **Screen 4 (Action):** "Looks good! Upload your first real product photo to replace ours."
### AI Agent Integration
- The Autodream agent (or a dedicated Onboarding Agent) takes the initial prompt, determines the business category, generates a store name, drafts a return policy, and selects a color palette.

## Implementation Prompt
**User-Facing Outcome:** A brand new user downloads the app, types a single sentence describing their business, and within 60 seconds is presented with a fully functional, beautifully designed mobile storefront.
**Critical User Journey (CUJ):**
1. User opens app and enters business description.
2. System calls the LLM backend to generate store metadata.
3. System provisions the tenant and applies the generated metadata.
4. User is navigated to the dashboard with their store already "Live."
**Acceptance Criteria:**
- Setup requires 0 configuration of shipping, taxes, or domains by the user.
- The UI strictly adheres to the Visual Excellence Mandate (Glassmorphism, entrance animations < 300ms).
- The onboarding flow must be completely mobile-responsive (375px first).
- The backend successfully provisions the tenant using the AI-generated payload without failing on schema validation.

## Priority
P0

## Estimated Scope
Medium