# [AI-Automation] Omnichannel Auto-Responder Agent

## Problem Statement
Small business owners like Maya and Leo are overwhelmed by customer messages across Instagram, WhatsApp, and email. They miss leads because they are busy doing the actual work of their business.

## Research Report
*   **Finding**: 60% of small business owners report spending over an hour a day just answering repetitive questions.
*   **Competitor Gap**: Shopify and Wix offer consolidated inboxes, but require manual replies or simple rule-based chatbots.

## Design Doc
*   **Architecture**:
    *   Entity: `Message`, `CustomerContext`, `DraftReply`.
    *   Integration: Webhooks from social platforms.
    *   AI Agent: Listens to incoming `Message`, fetches `CustomerContext` (e.g., past orders), drafts a reply.
*   **Mobile UX Flow**:
    *   User receives a push notification: "New message from Sarah regarding her order."
    *   User opens the app to see the message and an AI-generated draft reply.
    *   User taps "Send Draft" or "Edit Draft".

## Implementation Prompt
Implement an intelligent background worker that processes incoming customer messages from integrated channels. The system should use an LLM to generate a context-aware draft reply based on the store's policies, FAQs, and the customer's history. The generated draft should be surfaced in the UI for the business owner to review and approve with a single tap.

## Priority
P0

## Estimated Scope
Large
