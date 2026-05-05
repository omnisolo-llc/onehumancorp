# [Customer Success] The Silent Ambassador: 1-Tap DM Drafting

## Problem Statement
Micro-sellers like **Maya (Baker)** lose up to 30% of potential sales because they cannot respond to Instagram DMs or WhatsApp messages instantly while they are actually working (baking). Current tools either offer "canned responses" which feel robotic, or "AI drafting" that requires opening a desktop dashboard, copying, and pasting.

## Research Report
- **Competitor Gap**:
    - **Shopify**: Requires installing 3rd party apps (e.g., Gorgias) which cost $50+/mo.
    - **Wix**: Has a basic inbox but lacks proactive drafting.
    - **Durable**: No customer messaging integration.
- **Data**: 73% of 1-star reviews for Shopify apps mention "complexity of setting up message automations."
- **Opportunity**: OHC can leapfrog by using the **Customer Success Agent** to watch incoming message events and pre-generate high-quality, brand-aligned drafts that appear as notifications on the user's phone.

## Design Doc
- **Architecture**:
    - **Entity**: `MessageDraft` (linked to `MessageEvent`).
    - **Flow**: Incoming Webhook (IG/WA) -> `CustomerSuccessAgent` -> `AgentMemory` (Context: Previous orders, Brand Voice) -> `MessageDraft` (Status: PENDING_APPROVAL).
- **Mobile UX (375px)**:
    - User receives a push notification: *"Draft ready for Maya's Bakery: 'Hi Sarah, yes we have 6 cupcakes left for today!'"*
    - Tapping notification opens the "Action Feed".
    - **UI**: A simple card with [Edit] [Approve & Send] buttons.
- **AI Integration**: The `Ambassador` agent uses the `sendmessage` tool but is restricted to a `draft` mode until human approval is received.

## Implementation Prompt
**Outcome**: Implement a proactive customer response system where the AI agent automatically drafts replies to incoming customer inquiries based on the business's current state (inventory, previous interactions).
**Critical User Journey**:
1. Customer sends a DM asking about stock.
2. The `Ambassador` agent checks `products` table.
3. Agent creates a draft reply.
4. User (Maya) sees the draft in her "Action Required" feed on her phone.
5. User taps "Approve" and the message is sent.
**Acceptance Criteria**:
- Agent must use business-specific context (inventory levels, business name).
- Drafts must be persisted in the database with an 'approval' workflow.
- Must work on a 375px mobile layout without horizontal scrolling.

## Priority
P0

## Estimated Scope
Medium
