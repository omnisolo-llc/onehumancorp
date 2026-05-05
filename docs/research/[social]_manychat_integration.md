# Title
Social Media Integration: ManyChat for Unified Inbox & Auto-Replies

# Problem Statement
Small business owners like Maya (The Home Baker) receive countless DMs on Instagram and Facebook asking the same questions ("Do you do vegan cakes?", "What are your hours?"). It is overwhelming to manually reply to all of them, leading to missed sales and slow response times. They need an automated way to handle common inquiries and funnel them into a single, unified inbox within OHC.

# Research Report
**Tool Analyzed:** ManyChat (Meta Graph API)
ManyChat provides an easy-to-use platform for automating Instagram DMs, Facebook Messenger, and WhatsApp. It is highly reliable and handles Meta's complex API requirements seamlessly.
- **Ease of Use (for non-technical users):** Excellent. ManyChat uses a visual flow builder, though OHC's goal is to abstract this entirely and let the "Customer Success" AI agent handle it.
- **Pricing:** Free tier available; Pro starts at $15/month. Very accessible for SMBs.
- **Reputation:** Industry standard for social chat automation.
- **Integration Risk:** Relying on Meta's Graph API is notoriously fickle, but ManyChat abstracts most of these headaches away.
- **Cloud/Standalone:** Fits perfectly into a Cloud (multi-tenant) model. Can be integrated via webhooks.

# Design Doc
- **Trigger:** User connects their Instagram/Facebook page to OHC via OAuth in the Marketing & Advertising department settings.
- **Actions:**
  1. OHC sets up webhooks with ManyChat (or directly via Meta Graph API if ManyChat provides a headless API) to listen for incoming DMs.
  2. When a DM arrives, it is routed to the OHC backend.
  3. The "Customer Success" AI agent generates a draft reply based on the business's context (menu, hours, FAQs) stored in the vector database.
  4. The reply is sent back through the API to the customer on Instagram/Facebook.
- **User Experience:** The business owner sees all incoming messages in a unified OHC Inbox tab. They can see what the AI has auto-replied and choose to take over the conversation at any time.

# Implementation Prompt
Implement a unified inbox feature that allows users to connect their Instagram and Facebook accounts. The system must listen for incoming direct messages, display them in a real-time UI within the OHC dashboard, and allow the "Customer Success" AI agent to automatically draft and send replies based on the business profile. Acceptance criteria include a working OAuth flow, real-time message syncing, and successful AI auto-replies.

# Priority
P0

# Estimated Scope
Large
