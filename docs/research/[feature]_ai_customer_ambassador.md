# Title: The Silent Ambassador: Autonomous Customer Reply Agent

## Problem Statement
Small business owners, like Maya the Baker, lose up to 30% of potential sales due to slow response times in Direct Messages. They are often busy making products and cannot instantly reply to common inquiries like "What are your hours?" or "Do you have vegan options?".

## Research Report
Current tools (like Shopify Inbox) provide centralized messaging but still require the owner to manually type or trigger macro responses. SMBs experience "Operational Fatigue" (68% of users cite this as a major pain point).

## Design Doc
*   **Architecture Flow:**
    1.  Incoming Message Event (e.g., via Instagram DM integration).
    2.  Agent analyzes message against Business Knowledge Base (products, hours, policies).
    3.  Agent drafts a personalized response.
    4.  Draft is pushed to a "Pending Approvals" queue.
*   **UI/UX:** A mobile-first lock screen notification or dashboard widget displaying the drafted reply with two buttons: `[Approve & Send]` and `[Edit]`.
*   **AI Integration:** The built-in agent framework uses LLM to generate context-aware replies without sounding robotic.

## Implementation Prompt
Implement an event-driven background agent that listens for incoming customer messages. It should query the platform's vector memory for relevant business context and generate a draft reply. The draft must be surfaced in the Slint UI dashboard's activity feed, allowing the user to approve and send the message with a single tap. Do not auto-send without user approval.

## Priority
P0

## Estimated Scope
Medium
