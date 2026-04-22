# [sales] Instagram DM AI Responder

## Title
Implement AI-driven Instagram DM Auto-Responder for Sales Department

## Problem Statement
Many small business owners (e.g., Maya the Baker) rely heavily on Instagram DMs for lead generation and custom orders. Responding manually is time-consuming, prone to delays, and impossible while sleeping. Lost leads due to slow response times directly impact revenue.

## Research Report
*   **Competitor Analysis**: Shopify relies on third-party apps for robust social media auto-responders. Tools like ManyChat exist but are complex to configure and disconnected from the core business platform.
*   **User Need**: Seamless integration where the "Customer Success / Sales" AI agent handles basic inquiries ("Do you do vegan cakes?") and routes complex custom orders to a deposit workflow without the owner intervening.

## Design Doc
*   **Architecture**:
    *   Integrate with Instagram Graph API for Webhooks.
    *   New `InstagramWebhookHandler` in `Sales` microservice.
    *   AI Agent evaluates intent using LLM (Gemini Pro) configured with business context (e.g., menu, pricing, FAQs).
    *   State tracked via Redis for conversation context.
*   **UI Wireframes**:
    *   Mobile-first (375px) toggle in "Customer Success" Hub: "Enable Instagram Auto-Reply".
    *   Simple text area to input business-specific rules or tone guidelines (optional, defaults to friendly and helpful).

## Implementation Prompt
Create the Instagram DM webhook integration and AI responder logic. The user should be able to authenticate their Instagram Professional account via the UI, enable the auto-responder, and see conversations handled by the AI in their unified inbox. Acceptance criteria include successfully receiving an IG webhook, processing it through the LLM with tenant context, and dispatching the reply back to Instagram within 5 seconds.

## Priority
P0

## Estimated Scope
Medium
