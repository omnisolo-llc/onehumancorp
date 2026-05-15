# [CRM] AI Auto-Reply Agent

## Problem Statement
Small business owners spend 1-2 hours daily answering repetitive customer questions (shipping times, return policies, basic sizing) across Instagram DMs, WhatsApp, and email. This leads to burnout, delayed responses, and lost sales.

## Research Report
Our analysis of r/smallbusiness reveals 'Customer Support' is the #5 top pain point. Competitors like Shopify offer Sidekick (which helps the merchant) but lack autonomous, integrated customer-facing agents. 73% of 1-star reviews for legacy platforms cite difficulty managing multi-channel communications. Source: Reddit r/ecommerce.

## Design Doc
- **High-level architecture:** A unified inbox system that ingests messages from Instagram, WhatsApp, and Web Chat via standard Webhooks. A centralized LLM processing queue evaluates incoming messages against the business's known context (Knowledge Base, Order History, Inventory).
- **UI Wireframes:** A 'Conversations' tab on the mobile app. Messages handled by AI are marked with a subtle sparkle icon. The owner can tap to 'Take Over' the chat at any time.
- **Mobile UX Flow (375px):** Home Screen -> Tap 'Inbox' -> View unread messages. AI-drafted responses wait for 1-tap approval, or can be set to 'Auto-Send' for high-confidence FAQs.
- **AI Integration:** The Auto-Reply Agent needs read access to the business's FAQ, active inventory, and order status tables.

## Implementation Prompt
Implement the unified Inbox view and the AI Auto-Reply system. The user should be able to connect their IG/WhatsApp, see incoming messages in one place, and toggle an 'AI Assistant' on. The CUJ is: User receives a DM asking 'Do you have the red shirt in medium?' -> System checks inventory -> System drafts 'Yes, we have 2 left! Here is the link...' -> User taps 'Approve & Send'.

## Priority
P0

## Estimated Scope
Large
