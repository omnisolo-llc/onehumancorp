# [AI] The Silent Ambassador: Autonomous Customer Success

## Problem Statement
Small business owners (especially solopreneurs like Maya the baker) lose up to 30% of their potential sales because they are too busy executing their craft to respond quickly to customer inquiries in Instagram DMs, email, or WhatsApp. Existing solutions rely on generic, frustrating chatbots or require the user to stop what they are doing and manually type out a response.

## Research Report
Our competitive audit and SMB pain point analysis (see `smb_pain_points_top_10.md` and the market matrix) reveals that "Operational Fatigue" and "Communication Lag" are top friction points.
*   **Competitor Failure:** Shopify Inbox requires manual response. Wix chatbots are rigid and logic-tree based.
*   **Data:** 68% of SMB owners cite managing cross-channel communication as a major source of fatigue.
*   **Opportunity:** Shift from a "tool" that requires manual input to a "teammate" that operates autonomously in the background.

## Design Doc
**High-Level Architecture:**
*   The system acts as an event listener on the unified inbox event mesh.
*   When a new customer message arrives, the backend triggers the "Silent Ambassador" AI agent.
*   The agent uses RAG (Retrieval-Augmented Generation) against the business's product catalog, store policies, and past conversation memory to draft a highly contextualized response.
*   The drafted response is placed into the "Sub-Agent Queue" with a status of `PENDING_APPROVAL`.

**Mobile UX Flow (375px First):**
1.  User receives a push notification: "Agent drafted a reply to Sarah about the Wedding Cake inquiry."
2.  User taps the notification, opening a modal.
3.  The modal displays the customer's message and the AI's drafted response.
4.  The user sees three large, easily tappable buttons: `Approve & Send`, `Edit`, `Discard`.
5.  If approved, the event mesh sends the reply via the appropriate channel.

## Implementation Prompt
Implement the backend "Silent Ambassador" agent capable of reading from the unified inbox event stream, querying the business context (catalog, policies), and drafting a contextual response. Create the mobile-first (375px optimized) "Action Required" UI component that displays the drafted message and allows for 1-tap approval or editing.
*   **Critical User Journey (CUJ):** A customer asks about a product on Instagram. The agent drafts a reply. The owner opens the OHC app on their phone, sees the drafted reply in the Activity Feed, taps "Approve", and the message is sent.
*   **Acceptance Criteria:** The drafted response must incorporate accurate data from the store's inventory/policies. The mobile UI must be fully functional and usable on a 375px viewport with clear, accessible tap targets.

## Priority
P0

## Estimated Scope
Medium
