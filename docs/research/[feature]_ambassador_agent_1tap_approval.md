# Feature Brief: The Silent Ambassador (1-Tap Approvals)

## Problem Statement
Solopreneurs suffer from "Operational Fatigue" (the #2 pain point). They lose up to 30% of sales because they cannot respond to customer DMs or inquiries while actually doing their work. They need an assistant that prepares responses for them.

## Research Report
Competitors treat AI as a "Tool" (the user opens an AI chat and asks it to write an email). OHC will treat AI as a "Teammate". The AI should monitor incoming messages and draft responses automatically, requiring only a single tap from the owner to send.

## Design Doc
**Architecture & Integration:**
- **Entity Types:** `CustomerMessage`, `AgentDraft`, `ApprovalAction`
- **Integration Points:** Background Worker (listening to the event mesh), LLM integration (for drafting).

**UX/UI Flow (Mobile-First 375px):**
1.  **Lock Screen Notification:** "The Ambassador drafted a reply to Sarah about the vegan cake."
2.  **Action Feed (Dashboard):** A card showing the incoming message and the AI-drafted reply.
3.  **1-Tap Approval:** A large, easily accessible "Approve & Send" button, alongside an "Edit" button.

## Implementation Prompt
Implement the "Silent Ambassador" feature. A background worker must listen for incoming customer messages, use an LLM to generate a context-aware draft response, and queue this draft in an "Action Required" feed on the dashboard. The user must be able to approve and send the draft with a single click. Ensure the background process does not block the UI. Acceptance criteria: Messages are processed automatically; the dashboard displays pending approvals; clicking approve sends the message and dismisses the card.

## Priority
P0

## Estimated Scope
Medium
