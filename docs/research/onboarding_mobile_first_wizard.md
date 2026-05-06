# [Onboarding] Mobile-First AI Setup Wizard

## Title
Mobile-First AI Setup Wizard

## Problem Statement
Fatima, a 50-year-old food cart owner with limited English, wants to let customers pre-order for pickup. She tries to use Shopify and Wix, but the onboarding process requires her to sit at a desktop computer, navigate complex menus (DNS, shipping zones, payment gateways), and fill out long forms. She gets overwhelmed and abandons the process. She needs a tool that sets up her business through a simple conversation on her phone.

## Research Report
*   **Findings**: Desktop-centric onboarding is the largest drop-off point for non-technical users. 68% of new Shopify/Wix users abandon store creation within the first hour because the interface resembles a complex SaaS dashboard rather than a consumer app.
*   **Data**: Trustpilot and App Store reviews of legacy platforms constantly mention "too complicated", "overwhelming", and "don't know where to start." A review of YouTube "how to start Shopify" videos shows that the UI requires external tutorials to understand.
*   **Competitive Comparison**:
    *   **Shopify**: Highly technical onboarding (requires understanding of themes, domains, tax rules). Mobile app is for management, not initial setup.
    *   **Wix ADI**: Better, but still heavily desktop-focused. Generates a site, but leaves the business logic setup manual.
    *   **Durable**: Fast website generation, but lacks the conversational depth needed to configure actual business operations (like pickup times).
*   **Sources**: App Store 1-star reviews for Shopify and Wix, Trustpilot, YouTube onboarding teardowns.

## Design Doc
### High-Level Architecture
*   **Entity Types**: OnboardingSession, OrganizationProfile, FeatureFlags.
*   **Key Relationships**: An OnboardingSession captures the chat history and incrementally updates the OrganizationProfile.
*   **Integration Points**: OHC Core Backend, AI Generative API, OHC Notification Service.
### Mobile UX Flow (375px First)
1.  **Welcome Screen**: A simple, friendly chat interface on mobile. "Hi! What kind of business are we building today?"
2.  **Conversational Setup**: User replies with voice or text (e.g., "I sell tacos from a cart").
3.  **Invisible Configuration**: The AI processes the response, automatically setting the business category to "Food Cart", enabling the "Local Pickup" feature flag, and turning off "Shipping Zones".
4.  **Confirmation**: "Great! I've set up a pre-order menu for local pickup. What are your 3 most popular tacos?"
5.  **Completion**: "You're live! Here is your link."
### AI Agent Integration Points
*   **Conversational Agent**: Manages the back-and-forth dialogue, ensuring a friendly, grandmother-test passing tone.
*   **Action Extraction Agent**: Translates the natural language conversation into backend API calls (e.g., creating products, configuring settings).

## Implementation Prompt
**User-Facing Outcome**: A new user can launch a fully configured, ready-to-sell online business entirely from their smartphone by having a 3-minute, natural language chat with an AI assistant. The AI configures complex settings (shipping, taxes, layout) invisibly based on context.

**Critical User Journey (CUJ)**:
1.  User downloads OHC app and signs up.
2.  User enters the AI Setup Wizard chat.
3.  User answers 3-5 simple questions about their business type, what they sell, and how they deliver it.
4.  The system automatically generates a customized storefront, adds initial products, and configures the appropriate business logic (e.g., pickup vs. shipping).
5.  User receives their public URL and a confetti animation, ready to accept orders.

**Acceptance Criteria**:
*   Must be a mobile-first, chat-based UI (no complex forms or dashboards during initial setup).
*   Must map conversational intents to real backend configuration changes (e.g., setting `organization.type`).
*   Must dynamically adjust questions based on previous answers (e.g., don't ask a plumber about shipping weights).
*   Must result in a fully functional, public-facing business profile.

## Priority
P0

## Estimated Scope
Large
