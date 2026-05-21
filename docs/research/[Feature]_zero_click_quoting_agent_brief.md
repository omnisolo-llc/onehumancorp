# [Research] Zero-Click Quoting Agent for Service SMBs

## Title
Zero-Click Quoting Agent for Service Businesses

## Problem Statement
Service business owners like Carlos (Handyman) lose leads because they are out on jobs and cannot manually draft quotes or invoices when inquiries arrive via DM or email. Traditional platforms (like Shopify or Wix) either don't support service quoting well or require manual data entry, leading to delayed response times and lost revenue.

## Research Report
Our analysis of the SMB platform market (comparing Shopify, Durable, Wix, etc.) reveals that while AI website builders (like Durable) can generate a site quickly, they fail to automate the ongoing operational burden of quoting. User sentiment data indicates that owners are overwhelmed by manual administrative tasks. Providing an AI agent that automatically drafts quotes based on historical data directly addresses this gap.

## Design Doc
**Architecture & Integration:**
*   **Inbox Integration:** Agent listens to connected channels (Instagram DM, email, SMS).
*   **KAIROS Memory Access:** Agent queries past job data and standard pricing from the KAIROS AutoDream memory vectors.
*   **Mobile UI Flow (375px first):**
    1.  New inquiry arrives.
    2.  Agent drafts quote in background.
    3.  Push notification sent to owner: "Quote ready for [Customer Name] - $150".
    4.  Owner taps notification.
    5.  Screen shows drafted quote with "Approve & Send" or "Edit" buttons.

## Implementation Prompt
**User-Facing Outcome:** The SMB owner receives a push notification on their phone with a fully drafted quote ready to be approved and sent to the customer with one tap.
**Critical User Journey:**
1. Customer messages "How much to fix a leaky pipe?"
2. Agent reads message, checks memory for "leaky pipe" pricing.
3. Agent drafts a $150 quote and notifies the owner.
4. Owner taps "Approve".
5. Agent sends the quote and booking link to the customer.
**Acceptance Criteria:**
* Agent accurately extracts intent from incoming messages.
* Agent drafts a quote based on historical context.
* Owner can approve the quote with a single action on mobile.

## Priority
P0

## Estimated Scope
Medium
