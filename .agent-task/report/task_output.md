# Research Report: End-to-End Business Journey Lifecycle and Agentic Onboarding

## Overview
This report details the architectural design for the "Day One" onboarding flow and end-to-end business journey lifecycle for small business owners on the OneHumanCorp platform. The design prioritizes a conversational, AI-driven experience ("Agentic Onboarding") to minimize cognitive load, allowing non-technical users to launch a functional business in under 10 minutes from a mobile device.

## Findings
- **High Abandonment in Traditional Platforms:** Competitors like Shopify and Wix require users to configure complex systems (payment gateways, domains, shipping zones) before seeing value.
- **Mobile-First is Critical:** A significant portion of our personas (e.g., Carlos the Handyman, Maya the Baker) rely heavily or exclusively on mobile devices.
- **Agentic Onboarding as a Differentiator:** By using AI to dynamically generate content (website layout, product descriptions, initial emails) during a chat-like onboarding flow, we can drastically reduce time-to-value.
- **Deferred Configuration:** Non-critical setup steps (like Stripe connection and custom domain linking) should be deferred until the user reaches specific milestones (e.g., getting their first customer or hitting tier limits).

## Proposed Architecture
The proposed architecture, documented in `docs/research/[architecture]_business_journey.md`, includes:
1.  **Lifecycle Stages:** Defined stages from Acquisition to Referral, with specific AI agent interactions at each point.
2.  **Conversational Wizard:** A highly opinionated onboarding flow that asks basic questions (business name, type) and defers complex settings.
3.  **Persona Sequence Diagrams:** Mermaid.js sequence diagrams detailing the specific user journeys for five core personas, highlighting the interactions between the mobile app and AI agents (Marketing, Operations, Sales, Finance, Customer Success, Business Advisory).
4.  **UI/UX Guidelines:** Strict adherence to a 375px mobile-first design, utilizing the OHC Premium Token library (Glassmorphism, Outfit/Inter typography), and avoiding desktop-style navigation patterns on mobile.

## Next Steps
- An Implementer agent should pick up the implementation task to build the conversational onboarding wizard in the Flutter mobile application based on the provided design doc and acceptance criteria.

```yaml
issue_title: "[architecture] Implement End-to-End Business Journey Lifecycle and Agentic Onboarding"
issue_id: "agentic-onboarding-design"
issue_priority: "P0"
issue_description: "Implement the 'Day One' onboarding flow for the mobile application. Create a guided, conversational wizard that captures the user's business name and primary business type without requiring payment setup or custom domain configuration initially. Land the user on a simplified dashboard screen (375px mobile-first layout) that displays their live public URL and a primary CTA to 'Add your first product/service'. The UI must adhere to the OHC Premium Token library."
issue_todo_list:
  - [ ] Build conversational onboarding wizard UI in Flutter.
  - [ ] Implement state management (Riverpod) for the onboarding flow.
  - [ ] Create the simplified mobile-first dashboard screen.
  - [ ] Integrate with Marketing Agent for initial storefront generation.
issue_label: ["architecture", "mobile", "onboarding", "P0"]
```