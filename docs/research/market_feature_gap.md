# Issue Brief: Autonomous AI Background Agents for Operations

## Problem Statement
Small business owners (like Carlos the Handyman or Maya the Baker) are overwhelmed by manual tasks: answering repetitive questions ("Do you do vegan?"), writing product descriptions, and following up on incomplete bookings. Competitor platforms (Shopify, Wix) treat AI as a reactive chatbot or a one-time setup tool. Users need AI that operates autonomously in the background, acting as true functional departments (Customer Success, Operations, Marketing) rather than mere prompt-and-response tools.

## Research Report
Based on an analysis of Shopify, Wix, Squarespace, and GoDaddy, as well as Reddit/App Store user complaints:
- **Shopify & Wix** offer AI (Sidekick, ADI), but they require the user to initiate actions.
- **Top User Complaints** highlight the burden of constant customer communication and the fatigue of managing inventory descriptions.
- **Opportunity:** OHC can leapfrog competitors by implementing autonomous, background AI agents that continuously monitor the business state and take action on behalf of the owner, thereby fulfilling the promise of "AI does the heavy lifting invisibly."

## Design Doc
### High-Level Architecture
- **Agent Roles:** Introduce specific agent personas corresponding to business departments (e.g., "The Ambassador" for Customer Success, "The Operations Manager" for inventory).
- **Event-Driven Triggers:** Agents must be triggered by domain events (e.g., `MessageReceived`, `CartAbandoned`, `InventoryAdded`) rather than direct user prompts.
- **State Management:** Use the PostgreSQL `SKIP LOCKED` pattern for the AI Job Queue to ensure reliable processing of background tasks.
- **UI Integration:** The mobile UI (375px first) should display an "Agent Activity Feed" showing what the agents have done recently, allowing the user to review or override actions if necessary.

### Mobile UX Flow (375px First)
- **Home Screen:** A prominent, non-intrusive feed titled "Agent Actions Today" (e.g., "The Ambassador drafted 3 replies to Instagram DMs", "The Promoter scheduled a post for the new Vegan Cake").
- **Detail View:** Tapping an action allows the owner to read the draft and click "Approve & Send" or "Edit".
- **Settings:** A simple toggle screen to enable/disable specific autonomous behaviors (e.g., "Auto-reply to common questions", "Auto-draft social posts").

## Implementation Prompt
Implement the backend job queue and agent event processing loop to enable autonomous AI actions. The system should listen for standard business events (e.g., incoming messages) and queue them for the appropriate AI agent. Create the Flutter mobile UI (ensuring perfect rendering at 375px) to display the "Agent Activity Feed" on the home dashboard, allowing users to review and approve drafted actions. The feature must be entirely transparent to the user, with plain-language descriptions of the agent's actions.

## Priority
P0

## Estimated Scope
Large
