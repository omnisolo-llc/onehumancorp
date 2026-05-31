# Issue Brief: Progressive AI Interview Onboarding

## Title
[Onboarding] Progressive AI Interview Onboarding

## Problem Statement
The "Blank Canvas" Problem: Non-technical users are paralyzed by complex onboarding dashboards and configuration menus (shipping, taxes, DNS). They abandon the platform when faced with traditional forms requiring them to manually piece together a store setup. A 30-60 minute setup process acts as a massive barrier to entry.

## Research Report
- **Market Gap:** Current solutions (Shopify, Wix, Squarespace) require users to act as systems integrators. Even those utilizing AI generation (e.g., Wix ADI) drop the user into an overwhelming configuration editor post-generation.
- **User Pain Points:** 70% drop-off rates on initial setup screens when presented with multi-tab configurations (Shipping, Payments, Taxes, Products). The cognitive load is simply too high for our target persona (e.g., Maya the baker, Carlos the handyman).
- **Opportunity:** The setup phase can be completely abstracted away by conversational AI. By treating onboarding as an interview rather than a data entry task, we drastically lower the barrier to entry, achieving our "< 10 min" launch promise.

## Design Doc
### Architecture & Logic
- **Trigger:** First-time user sign-up.
- **Flow:**
  - Launch a conversational AI interface (The "Onboarding Agent").
  - The Agent conducts a dynamic, short interview (3-5 questions) asking for:
    - Business Name/Concept (e.g., "I bake custom cakes in Seattle").
    - Target Audience/Pricing ("Mostly weddings, starting at $200").
    - Vibe/Aesthetic ("Elegant, minimalist").
  - **Background Processing:** While chatting, the agent translates these answers into structured platform configurations (Store layout, placeholder products, base pricing, default shipping/tax zones).
- **Presentation:** Once the interview concludes, the user is presented with a fully functional, populated storefront draft requiring a single "Approve" action.
- **Data Model:** Store the interview transcript in `tenant_onboarding_logs` and use it to seed the tenant's initial `tenant_settings` and `products` tables.

### Mobile UX Requirements
- **Constraint:** Must feel like a natural iMessage/WhatsApp chat on a 375px screen.
- **Components:**
  - Full-screen chat interface.
  - Native keyboard handling.
  - Non-blocking UI while the background generation happens (show "Thinking..." or "Building..." status indicators).
- **Accessibility:** Ensure high contrast text bubbles and clear "Skip" options for questions if necessary.

## Implementation Prompt
Implement a chat-based onboarding flow that eliminates traditional technical forms. Create a conversational UI component that interacts with the backend AI agent to collect basic business information in 3-5 natural language questions. The backend should use this transcript to autonomously configure a complete initial store state. The final step of the flow must present the user with their fully configured store for a 1-tap approval to go live.

## Priority
P0

## Estimated Scope
Large
