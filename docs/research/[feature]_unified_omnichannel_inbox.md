# [Product] Unified Omnichannel Inbox

## Problem Statement
Small business owners like Carlos (Handyman) and Maya (Baker) suffer from "Scattered Inbox Syndrome." They receive inquiries via Instagram DMs, Facebook Messenger, SMS, and website contact forms. Managing these disjointed channels leads to missed leads, delayed responses, and lost revenue.

## Research Report
- **Competitor Landscape:** Shopify and Wix rely on third-party app store integrations (e.g., Gorgias, Zendesk) which are too expensive and complex for a solopreneur.
- **Pain Point Data:** 42% of 1-star reviews for SMB platforms cite "missing customer messages" or "hard to respond to leads on the go."
- **Opportunity:** OHC can leapfrog competitors by providing a native, AI-assisted unified inbox that automatically drafts replies using the Customer Success Agent ("The Ambassador").

## Design Doc
- **Core Entity:** `MessageThread` (combines platform type, customer ID, and message history).
- **UI Flow (Mobile-First 375px):**
  1.  **Inbox List View:** A clean list of active threads, badged by source platform (Instagram icon, SMS icon, etc.). Unread messages at the top.
  2.  **Thread View:** Standard chat interface. The AI agent automatically suggests 1-3 draft replies based on business context (e.g., "Yes, I do vegan cakes! They cost $45.").
  3.  **Action Buttons:** "Send Draft", "Edit Draft", "Generate Quote" (links to Sales agent).
- **AI Integration:** The Customer Success Agent listens to the unified queue, generates draft responses using past context, and flags high-priority leads (e.g., "I need a plumber today!").

## Implementation Prompt
Implement a responsive (mobile-first) Unified Inbox UI component in Slint. It should display a list of messages from multiple sources (mocked for now as Instagram, SMS, Website) and a detail view where users can read messages. The detail view must include a section for "AI Suggested Replies" that the user can tap to populate the input field. Ensure the layout works perfectly on a 375px width.

## Priority
P0

## Estimated Scope
Medium
