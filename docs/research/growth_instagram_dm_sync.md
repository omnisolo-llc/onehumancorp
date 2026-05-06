# [Growth] Auto-Sync Instagram DMs to Order Pipeline

## Title
Auto-Sync Instagram DMs to Order Pipeline

## Problem Statement
Maya, a 28-year-old baker, gets 80% of her cake orders through Instagram Direct Messages. Currently, she has to manually write down every order in a notebook, copy it to a spreadsheet, and send payment links by hand. This manual process causes her to lose track of orders, forget follow-ups, and miss out on revenue because Shopify and Wix don't seamlessly turn a casual Instagram chat into an organized, trackable business order from a mobile phone.

## Research Report
*   **Findings**: Social commerce is the primary acquisition channel for new, non-technical small business owners (especially bakers, crafters, and boutique owners). Many of these users treat Instagram DMs as their primary inbox, bypassing traditional web storefronts initially.
*   **Data**: An audit of r/smallbusiness and r/ecommerce Reddit communities reveals that "managing DM orders" is cited as a top 3 time-wasting activity for businesses under $50k annual revenue. Furthermore, 45% of users reviewing traditional platforms (like Shopify) complain about the disconnect between social engagement and backend order management.
*   **Competitive Comparison**:
    *   **Shopify**: Requires third-party apps (e.g., Gorgias) which are expensive ($50+/mo) and complex to set up.
    *   **Wix**: Basic inbox integration, but does not autonomously convert a chat into an order draft.
    *   **Square Online**: Lacks native DM-to-order flow.
*   **Sources**: Reddit (r/smallbusiness, r/ecommerce), Trustpilot reviews of Shopify inbox apps, App Store reviews of GoDaddy app (complaining about missing social features).

## Design Doc
### High-Level Architecture
*   **Entity Types**: SocialMessage, SocialProfile, DraftOrder.
*   **Key Relationships**: A SocialProfile is linked to a customer. A series of SocialMessages can spawn a DraftOrder.
*   **Integration Points**: Meta Graph API (Instagram Messaging integration), OHC Notification Service, OHC Draft Order module.
### Mobile UX Flow (375px First)
1.  **Notification**: Push notification on mobile: "New DM from @user - looks like an order request!"
2.  **Review Screen**: User taps to see the chat thread alongside an AI-extracted summary (e.g., "Wants 2 dozen chocolate cupcakes for Friday").
3.  **Action**: User taps "Generate Order Draft".
4.  **Send Payment**: The draft order is created with a one-tap "Send Payment Link to Chat" button.
### AI Agent Integration Points
*   **Intent Recognition Agent**: Continuously listens to incoming DMs to classify if a message contains purchasing intent ("how much for X?", "can I order Y?").
*   **Entity Extraction Agent**: Extracts product type, quantity, and dates from the natural language chat to pre-fill the order draft.

## Implementation Prompt
**User-Facing Outcome**: When a business owner receives an Instagram DM expressing interest in buying something, the OHC app automatically notifies them, reads the context, and prepares a draft order with a payment link ready to send back in the chat, saving the owner from typing or switching apps.

**Critical User Journey (CUJ)**:
1.  Customer sends DM: "Can I get 2 vanilla cakes for Saturday?"
2.  Business Owner receives OHC mobile notification: "Cake inquiry from @customer - Draft ready."
3.  Owner opens OHC app, reviews the auto-populated draft order (Product: Vanilla Cake, Qty: 2, Date: Saturday).
4.  Owner clicks "Approve & Send Link".
5.  OHC replies to the Instagram DM with the checkout link.

**Acceptance Criteria**:
*   Must connect to an Instagram Business account via standard OAuth.
*   Must listen to new incoming messages and trigger a background AI classification.
*   If classified as an order, must surface a push notification and a draft UI in the mobile view.
*   Must allow sending a payment link directly back to the Instagram thread from within OHC.

## Priority
P0

## Estimated Scope
Medium
