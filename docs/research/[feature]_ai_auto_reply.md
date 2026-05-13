# [feature] AI Auto-Responder Agent

## Problem Statement
Small business owners like Carlos (Handyman) and Priya (Boutique Owner) frequently miss leads because they cannot answer emails or direct messages while they are actively working with customers or away from their desks. They need a system that responds instantly and accurately to common questions without requiring them to lift a finger.

## Research Report
Reddit (r/smallbusiness) shows owners lose up to 30% of their online leads due to slow response times. Existing competitors like Shopify and Wix require paid 3rd-party apps for even basic auto-replies. No competitor currently offers a deeply integrated, autonomous AI agent that can negotiate bookings or answer complex inventory questions out-of-the-box.

## Design Doc
*   **Architecture**: An asynchronous background worker intercepts incoming customer inquiries (email, SMS, or web chat). It checks the intent against a local RAG knowledge base of the store's policies, inventory, and scheduling availability, then drafts and sends a response.
*   **UX Flow**: The store owner toggles an "Enable AI Auto-Reply" switch in their OHC settings dashboard. When a message arrives, the AI replies instantly. The owner receives a daily digest notification summarizing handled messages.
*   **Mobile UX**: At 375px viewport, the feature is a single prominent toggle switch on the home dashboard labeled "Let AI Answer Customers."

## Implementation Prompt
Create an AI worker that automatically drafts and sends replies to incoming customer inquiries based on the organization's existing knowledge base and inventory. It must be a simple toggleable feature from the dashboard and require absolutely zero prompt engineering or setup from the user.

## Priority
P0

## Estimated Scope
Medium
