# Autonomous Customer Service Agent

## Problem Statement
Users like Carlos (handyman) and Maya (baker) lose leads because they are busy working and cannot reply to DMs or emails instantly. They need a system that answers common questions for them automatically.

## Research Report
* **Finding:** SMBs miss up to 30% of leads due to slow response times.
* **Competitor Comparison:** Shopify Sidekick is for the merchant, not the customer. Wix has basic auto-replies, but not context-aware AI.
* **Source:** Industry surveys on SMB response times.

## Design Doc
* **Architecture:** LLM-powered agent with read access to the merchant's FAQ, Inventory, and Policies data stores.
* **Mobile UX Flow:**
  1. Merchant toggles "AI Auto-Reply" ON.
  2. Customer sends an Instagram DM: "Do you have vegan cakes?"
  3. AI checks inventory, sees "Vegan Chocolate Cake", and replies: "Yes! We have a Vegan Chocolate Cake available for $30. Would you like the link to order?"
  4. Merchant sees the conversation flagged as "Handled by AI".

## Implementation Prompt
**Critical User Journey:** A customer messages the store asking a question about a product. The AI agent reads the store's data and replies accurately within 10 seconds, allowing the merchant to review the interaction later.
**Acceptance Criteria:**
* Agent can ingest basic store context (products, policies).
* Agent receives a simulated inbound message.
* Agent generates an accurate, polite response based ONLY on store data.
* Agent logs the interaction in the merchant's activity feed.

## Priority
P0

## Estimated Scope
Medium
