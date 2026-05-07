# [feature] The Silent Ambassador

## Title
The Silent Ambassador (Proactive Customer Success)

## Problem Statement
Solopreneurs and small business owners (like Maya the baker) lose significant sales volume because they cannot respond to DMs or emails quickly enough while they are busy producing their goods or providing services. The "never-ending inbox" creates operational fatigue and lost revenue.

## Research Report
*   **Gap:** Solopreneurs lose 30% of sales due to slow response times in DMs.
*   **Differentiation:** Instead of "AI writing assistance" (which requires a prompt), the agent watches the event mesh, drafts a reply based on business memory, and queues it in the Dashboard's "Action Required" feed.
*   **Outcome:** 1-tap responses from the lock screen.
*   **Evidence:** "Communication Lag" is ranked #8 in the Top 10 SMB Pain Points (40% frequency).

## Design Doc
*   **Entities:** IncomingMessage, DraftResponse, ApprovalAction.
*   **Key Relationships:** IncomingMessage belongs to a Customer and triggers a DraftResponse. DraftResponse is reviewed via ApprovalAction.
*   **UI/UX (Mobile-First 375px):**
    *   Dashboard features an "Action Required: Messages" feed.
    *   Each item shows the customer's original message and the AI-generated draft response.
    *   The user can tap "Approve & Send" or "Edit" to modify the draft before sending.
*   **AI Agent Integration:** A background agent listens for `IncomingMessage` events from various channels (Instagram, email, website chat). It queries the business memory context and generates a contextual `DraftResponse`.

## Implementation Prompt
Implement a proactive messaging agent that listens to incoming communication channels. When a new message arrives, the agent should automatically draft a reply based on the store's knowledge base and product inventory. This draft should be surfaced to the user's mobile dashboard for a 1-tap "Approve" or "Edit" action. Do not define specific API endpoints or database schemas; focus on the event listener and the UI presentation of the draft.

## Priority
P0

## Estimated Scope
Large
