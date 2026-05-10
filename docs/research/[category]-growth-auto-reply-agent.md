# AI Auto-Reply Social Agent
## Title
AI Auto-Reply Agent for Social Media DMs

## Problem Statement
Small business owners, like Maya (a baker who sells via Instagram), are overwhelmed by the volume of DMs asking the same questions: "Are you open?", "How much for a custom cake?", or "Do you deliver?". They miss potential sales because they are too busy baking or serving customers to reply instantly. Existing platforms (Shopify, Wix) require manual management or complex 3rd party chat bots. Non-technical founders need an invisible assistant that automatically answers routine questions and schedules orders, without complex setup.

## Research Report
Based on a deep competitor audit and SMB pain point research:
* **Shopify:** Offers "Shopify Sidekick," which helps the *merchant* manage their store, but does not autonomously handle customer-facing DMs on external social channels without 3rd-party apps (e.g., Gorgias, which is complex and costly).
* **Wix:** Provides basic automated email responses, but lacks a native, agentic AI for social DMs.
* **Pain Point Data:** In communities like r/smallbusiness and app store reviews, a recurring theme is the struggle to balance fulfillment with customer communication. Responding to inquiries is cited as a top 3 time-drain for solopreneurs.
* **AI Opportunity:** Automating social replies is a high-value differentiation. According to our AI Differentiation Manifesto, auto-replying to messages can save hours per day and capture leads while the owner is offline.

## Design Doc
**High-level Architecture:**
*   **Channels:** Integration with Instagram/Facebook Messenger APIs.
*   **Agent Logic:** A background worker (using KAIROS orchestration) that listens for incoming messages.
*   **Context:** The agent has access to the business's product catalog, pricing, FAQ, and operating hours.
*   **Handoff:** A clear mechanism to escalate complex queries to the human owner via the OHC mobile app.

**UI Flow (Mobile First - 375px):**
1.  **Setup (Simple Mode):** A single toggle in the OHC mobile app: "Enable AI Assistant for Instagram". The app requests standard Meta OAuth permissions.
2.  **Configuration:** The AI automatically reads the store's existing data (products, hours). The user does not need to write a script.
3.  **Notification:** When the AI handles a conversation, the user sees a summary notification: "AI booked a $45 cake order with Sarah."
4.  **Advanced Mode:** The user can toggle to view full chat logs or adjust the AI's "tone" (Friendly, Professional).

## Implementation Prompt
Implement an AI agent that automatically replies to customer inquiries on connected social media channels (e.g., Instagram DMs).
**Critical User Journey (CUJ):**
1. The business owner connects their Instagram account with one click.
2. A customer sends a DM asking for a product price.
3. The AI agent reads the store's catalog and instantly replies with the correct price and a link to purchase.
4. The owner receives a notification of the successful interaction, saving them time and securing a sale.

**Acceptance Criteria:**
* Must use a Progressive Disclosure Pattern (one-click setup by default, complex rules hidden under 'Advanced Mode').
* Must integrate with the KAIROS Sub-Agent Queue for background processing.
* Must correctly pull data from the tenant's product database.
* Must include a seamless handoff to human interaction when requested by the customer.

## Priority
P1

## Estimated Scope
Medium
