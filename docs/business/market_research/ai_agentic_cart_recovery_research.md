# Automated Cart Recovery via Agents

## Overview
This report analyzes the architectural gap and proposes a solution for an "Automated Cart Recovery Agent" within the OneHumanCorp platform. This addresses a critical business need for small businesses (Pain Point #5: "Abandoned Carts / Lack of Follow-up").

## Business Need
Users frequently abandon shopping carts, representing significant lost revenue for SMBs. The platform currently lacks a native, automated mechanism to re-engage these users without requiring complex third-party integrations (like Klaviyo), which alienates non-technical users.

## Proposed Solution: The Cart Recovery Agent (CRA)

### 1. Functional Description
The CRA is an invisible agent operating within the "Sales & Acquisition" or "Marketing & Advertising" department. It monitors active shopping sessions and automatically triggers personalized follow-up sequences when a cart is abandoned.

### 2. Trigger Mechanism
*   **Event:** `CartUpdated` or `SessionTimeout` events emitted by the checkout service.
*   **Condition:** A cart remains in a non-purchased state for a configurable duration (e.g., 1 hour, 24 hours).
*   **Data Requirement:** The user must have provided contact information (email or SMS) during the initial checkout steps.

### 3. Agent Capabilities
*   **Contextual Awareness:** The agent reads the cart contents, the user's browsing history, and any previous purchase history.
*   **Personalized Generation:** Utilizes the LLM (Gemini/GPT-4o) to generate a personalized message. It shouldn't just be "You left this." It should be "Hey, noticed you were looking at the Vegan Chocolate Cake. We bake fresh every morning. Complete your order now and we'll throw in a free cookie!"
*   **Multi-Channel Delivery:** Can send via Email, SMS, or WhatsApp depending on user preference and local regulations.
*   **Discount Strategy Integration:** Can optionally generate a temporary, single-use discount code to incentivize completion, based on the merchant's configured guidelines.

### 4. Architectural Gap Analysis
*   **Current State:** Cart state is likely stored in Redis or PostgreSQL, but there's no proactive monitoring or automated action triggering based on time-delays.
*   **Gap 1: Event Scheduling/Delays:** We need a robust mechanism to schedule a job (e.g., "Check this cart in 1 hour"). The existing PostgreSQL `SKIP LOCKED` job queue needs to support delayed execution or a dedicated time-series scheduler is required.
*   **Gap 2: Communication Infrastructure:** The platform needs reliable transactional email/SMS sending capabilities integrated with the Agent framework.
*   **Gap 3: Incentive Generation System:** The agent needs an API to generate valid, trackable, single-use discount codes within the platform's pricing engine.

### 5. Proposed Implementation Phases
*   **Phase 1: Basic Email Recovery:** Monitor carts, trigger standard template email after 4 hours. No AI generation yet, just reliable delivery.
*   **Phase 2: AI-Personalized Email:** Integrate LLM to draft the email based on cart contents and store tone.
*   **Phase 3: Omnichannel & Incentives:** Add SMS support and the ability for the agent to generate and offer discount codes.

### 6. User Experience (Merchant Side)
The merchant does *nothing* to set this up. It is on by default. They simply see an item in their weekly "Business Advisory" report: *"The Salesperson recovered 3 abandoned carts this week, resulting in $120 in additional revenue."*
