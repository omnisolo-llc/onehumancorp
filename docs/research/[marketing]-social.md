# Issue Brief: Autonomous Social Inbox Agent

## Title
Implement Autonomous Social Inbox Agent for Unified Messaging

## Problem Statement
Service professionals and "Instagram-first" sellers struggle to manage communications across multiple channels (Instagram DMs, WhatsApp, SMS, email). A common complaint is "I miss messages and lose sales" because they are too busy working on their business to constantly monitor every inbox. They need a system that not only consolidates messages but actively handles routine inquiries for them.

## Research Report
Competitor analysis shows that while platforms like Shopify integrate with specific sales channels, they lack a unified, AI-powered inbox that actively manages customer relationships. Our research into SMB pain points highlights fragmented communication as a major source of lost revenue and stress. Implementing an "Autonomous Social Inbox Agent" addresses pain point #3 ("I miss messages on Instagram and lose sales") and directly benefits personas like Maya and Carlos.

## Design Doc
```mermaid
graph TD
    A[Customer (IG/WA/SMS)] -->|Message| B(Integration Gateway)
    B --> C{Social Inbox Agent}
    C -->|Consults| D[(Store Data: Inventory/Policies)]
    C -->|Auto-Reply| E[Customer]
    C -->|Escalate| F[Owner Dashboard (Unified Inbox)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

*   **Architecture:** The agent runs as a background worker connected to external messaging integrations.
*   **Key Relationships:** The agent requires read access to product inventory, store policies, and existing customer profiles.
*   **UI Flow:** The dashboard features a unified "Inbox." Messages handled by the agent are marked "Resolved by AI." Messages requiring human intervention are flagged for the owner.
*   **Mobile UX (375px):** A simple chat interface. Swipe actions to "approve" or "take over" conversations.

## Implementation Prompt
Develop a "Social Inbox Agent" capable of connecting to external messaging platforms (starting with a generic webhook/SMS integration for testing). The agent must monitor incoming messages, identify intent (e.g., "Are you open today?", "Do you have the blue shirt in stock?"), and automatically reply based on the store's current data.

The Critical User Journey (CUJ):
1.  Customer sends a DM asking "Do you ship to Canada?"
2.  The Inbox Agent reads the store's shipping settings.
3.  The agent replies automatically: "Yes, we ship to Canada! Standard shipping is $10."
4.  The conversation is logged in the owner's dashboard as resolved.

Acceptance Criteria:
*   The agent must be able to accurately answer questions regarding inventory status, pricing, and basic policies (hours, shipping).
*   The agent must gracefully escalate to the human owner if it cannot confidently answer a query.
*   The setup must be simple, avoiding technical terms like "webhooks" or "API keys" where possible.

## Priority
P1

## Estimated Scope
Medium
