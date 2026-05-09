# Unified Inbox & AI Triage

## Problem Statement
Small business owners like Priya (boutique owner) and Fatima (food cart) are drowning in fragmented communication. They receive customer inquiries via Instagram DMs, Facebook Messenger, WhatsApp, SMS, and email. Switching between these apps causes them to miss messages, lose track of orders, and respond slowly, leading to lost revenue. Furthermore, many messages are simple repetitive questions ("What are your hours?", "Do you have this in size M?") that consume valuable time.

## Research Report
**Findings:**
*   A major pain point for solopreneurs is "context switching" between social media apps to handle customer service.
*   Many missed leads occur simply because the business owner was busy doing the actual work (e.g., baking, fixing a pipe) and couldn't check Instagram DMs in time.
*   **Competitor Comparison:**
    *   **Shopify Inbox:** Exists, but requires users to install a separate app. Adoption is lower among micro-merchants. Does not integrate well with personal WhatsApp/SMS which many SMBs use.
    *   **Wix:** Has an inbox, but AI capabilities are limited to basic auto-responders.
    *   **Standalone tools (e.g., Front, Intercom):** Too expensive and complex for a 1-person business.
*   **Opportunity:** OHC can provide a single, unified inbox natively integrated into the platform, powered by an AI agent that can automatically resolve common inquiries, categorize messages, and draft responses for complex issues.

## Design Doc
**Architecture / Entities:**
*   `Conversation`: A unified thread of messages, regardless of the source channel.
*   `Message`: Individual message unit with a `source_channel` enum (Instagram, SMS, Email, etc.).
*   `AIAction`: The action taken by the AI agent on a message (e.g., `DraftedReply`, `AutoResolved`, `FlaggedForReview`).

**Mobile UX Flow (375px first):**
1.  **Main Inbox View:** A single list of conversations, clearly marked with the source channel icon. Unread and "AI Needs Review" messages are pinned to the top.
2.  **Conversation View:** Standard chat interface. If the AI drafted a response, it appears in a distinct "Draft" state above the keyboard.
3.  **One-Tap Send:** The user taps "Approve & Send" on the AI draft, or taps to edit it.
4.  **Auto-Resolve Notification:** A subtle toast notification indicating "AI resolved 3 inquiries about store hours today."

**AI Agent Integration Points:**
*   **Triage Agent:** Analyzes incoming messages and categorizes them (e.g., `Question`, `Complaint`, `Lead`).
*   **Resolution Agent:** If the message is a common question (e.g., hours, location, return policy), the agent automatically responds and archives the thread.
*   **Drafting Agent:** For complex messages, the agent drafts a contextual reply based on the store's knowledge base and past interactions, waiting for user approval.

## Implementation Prompt
Implement a unified inbox feature that aggregates messages from simulated external channels. The core feature is the AI Triage Agent: it must intercept incoming messages, determine if they can be auto-answered based on business data (e.g., hours), and if not, draft a suggested response for the business owner to review before sending.

**Critical User Journey:**
1. User opens the OHC app and goes to the Inbox tab.
2. User sees a new message from "Customer A" via Instagram DM asking "Are you open on Sundays?".
3. User sees that the AI Triage Agent has already replied "Yes, we are open from 10 AM to 4 PM on Sundays!" and marked the conversation as resolved.
4. User sees a new message from "Customer B" via SMS asking "Can you build a custom dining table?".
5. User sees an AI-drafted reply: "Hi Customer B! Yes, I build custom tables. Could you provide the dimensions you're looking for?"
6. User taps "Send" to approve the draft.

**Acceptance Criteria:**
*   All messages, regardless of simulated source, appear in a single unified list.
*   The AI agent correctly identifies and auto-replies to explicitly defined common questions (e.g., hours, location).
*   The AI agent generates relevant draft replies for non-standard questions, requiring user approval.

## Priority
P1

## Estimated Scope
Medium