# Feature Mission: Automated Social DM-to-Order Conversion

## Problem Statement
Maya (baker, 28) sells primarily via Instagram DMs. She spends hours manually typing prices, checking availability, and sending payment links. This "DM chaos" is where she loses most of her potential customers who drop off before the sale is finalized.

## Research Report
- **User Pain Point:** 40% of SMB owners cite "Communication Lag" as a major sales killer.
- **Competitor Audit:** Manychat/Wix offer "Chatbots," but they are often clunky, require complex "if-this-then-that" setups, and feel robotic.
- **Gap:** A proactive agent that doesn't just chat, but *converts*—by drafting order links and checking inventory in real-time within the DM flow.

## Design Doc
### UX Flow (Mobile-First)
1. **Event:** Customer DMs "Hi, how much is the chocolate cake?"
2. **Agent Action:** The Ambassador (Customer Success) drafts a reply: "Our chocolate cake is $25. We have 3 left for today! Here is a link to order: [Link]".
3. **User Action:** Maya receives a notification on her lock screen. She taps "Approve & Send".
4. **Order Sync:** Once the customer pays via the link, the order is automatically created in the OHC system and inventory is decremented.

### AI Agent Integration
- **The Ambassador (Customer Success):** Manages the DM integration and drafts replies based on the `products` table and `memory_store`.
- **The Manager (Operations):** Syncs the inventory and creates the order once payment is confirmed.

## Implementation Prompt
Build a "DM Commerce Agent" for "The Ambassador". The agent must integrate with external social messaging events (via webhook/mesh), parse intent for product inquiries, and automatically draft a reply containing the product price, current availability, and a direct checkout link. The user must be able to approve these drafts with 1-tap from their mobile dashboard.

## Priority
P0

## Estimated Scope
Medium
