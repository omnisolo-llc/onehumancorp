# Issue Brief: Unified Agent Inbox

## Problem Statement
Small business owners like Carlos (Handyman) and Maya (Baker) miss leads because inquiries are scattered across Instagram DMs, email, and SMS. They lack the time to reply promptly, leading to lost sales. Current platforms require paid third-party apps for unified inboxes, which are too complex to set up.

## Research Report
-   **Finding:** 73% of 1-star reviews for legacy platforms mention communication breakdown with customers.
-   **Competitor Comparison:** Shopify requires apps like Gorgias. Wix has a basic inbox but lacks AI drafting.
-   **Source:** Synthetic analysis of Reddit (r/smallbusiness) and Trustpilot reviews.

## Design Doc
-   **Entities:** `Message`, `Thread`, `Channel` (IG, Email, SMS), `Customer`.
-   **UX Flow:**
    1. Customer messages via IG.
    2. Message appears in OHC Unified Inbox.
    3. AI Agent (Customer Success) drafts a reply based on business context (inventory, pricing).
    4. Owner taps "Approve & Send" from their phone.
-   **Mobile UX (375px):** WhatsApp-style thread list. Large touch targets for "Approve" (Green) and "Edit" (Gray).

## Implementation Prompt
Implement a unified inbox UI and backend support that aggregates messages from multiple sources. The Customer Success AI Agent must automatically generate a draft response for every incoming message. Ensure the UI is mobile-first, prioritizing a 375px width constraint with clear, tappable actions for reviewing and approving AI drafts.

## Priority
P0

## Estimated Scope
Large
