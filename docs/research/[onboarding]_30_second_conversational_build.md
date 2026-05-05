# [Onboarding] 30-Second Conversational Business Launch

## Problem Statement
The "Blank Page" is the biggest killer of new businesses. **Fatima (Food Cart)** or **Leo (Music Tutor)** are intimidated by "Choose a Template" or "Configure Shipping Zones." Durable.co proved that users love the "30-second build," but their sites are "thin" and lack a business backend (CRM/Agents).

## Research Report
- **Competitor Analysis**:
    - **Durable**: 30 seconds to a "site", but it's just a landing page.
    - **Shopify**: 30+ minutes of manual configuration before a store is "live."
    - **Wix**: ADI is fast but still ends in a complex dashboard.
- **User Pain**: 48% of SMB owners feel alienated by technical jargon (DNS, SKU, CNAME) during setup.
- **OHC Advantage**: We don't just build a site; we initialize a **Swarm**.

## Design Doc
- **Architecture**:
    - **Flow**: Conversational UI (Mobile-First) -> Collects: Business Name, Vibe (Color/Tone), Primary Goal (Sell/Book/Portfolio) -> `TaskDecompositionService` -> Initializes `Organization`, `Products`, and `DepartmentAgents`.
- **Mobile UX (375px)**:
    - No templates. Just a chat interface.
    - Progress bar: "Setting up your website..." -> "Hiring your Marketing Manager..." -> "Configuring Payments..."
    - End State: A live URL and a pre-filled "Action Feed" with the first 3 recommended steps.
- **AI Integration**: The `OnboardingAgent` uses the `OrganizationService` and `TaskDecomposition` to set up the entire tenant environment in one transaction.

## Implementation Prompt
**Outcome**: Create a unified "Conversational Setup" that launches a full business (Site + Agents + Payments) in under 60 seconds from a mobile phone.
**Critical User Journey**:
1. User enters company name and a one-sentence description.
2. AI generates a "Vibe" (Colors/Images) and a set of initial products/services.
3. System initializes all 7 Department Agents with business-specific context.
4. User is dropped directly into the "Action Feed" (Dashboard) with a live site.
**Acceptance Criteria**:
- Must bypass all "Template Selection" screens.
- Must initialize at least one `Product` or `Booking` entity based on the description.
- Total flow time < 60 seconds.

## Priority
P0

## Estimated Scope
Medium
