# [AI-Automation] Auto-Reply Sales Agent

## Title
Implement Autonomous Auto-Reply Agent for Cross-Channel Customer Inquiries

## Problem Statement
Small business owners (like Maya the Baker) are overwhelmed by customer DMs across Instagram, WhatsApp, and email asking repetitive questions ("What are your hours?", "Do you have vegan options?"). This manual work steals hours from their day and delays responses, losing sales. They need an invisible employee to handle these questions automatically.

## Research Report
- **Frequency:** 28% of SMB owners cite "keeping up with messages" as a top 3 pain point.
- **Competitor Gap:** Shopify and Wix offer unified inboxes, but require human intervention or complex 3rd-party chatbot configurations. Shopify's "Sidekick" helps the *merchant*, not the *end-customer*.
- **Market Data:** 70% of consumers expect a response to a social media message within 1 hour.
- **Source:** Reddit r/smallbusiness, Shopify App Store reviews for messaging apps.

## Design Doc
- **Core Entity:** `CommunicationAgent` attached to a `Tenant`.
- **Integration Points:** Social/Messaging APIs (Instagram, WhatsApp, Web Chat).
- **UX Flow (Simple Mode):**
  - User goes to "Messages" tab.
  - Toggles "Enable AI Assistant" to ON.
  - User can view a log of AI-handled conversations.
- **Mobile First:** The conversation log and toggle must be fully accessible and readable on a 375px viewport.

## Implementation Prompt
Create an autonomous agent service that can intercept incoming messages for a tenant, query the tenant's store data (inventory, hours, policies), and reply intelligently to the customer.
- The Critical User Journey (CUJ) involves the merchant enabling the feature with one click, and the agent successfully answering a customer question about a product and providing a checkout link.
- Must follow the "Grandmother Test" - no technical jargon in the settings UI.

## Priority
P0

## Estimated Scope
Large