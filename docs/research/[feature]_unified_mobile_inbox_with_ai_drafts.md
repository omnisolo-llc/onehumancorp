# Unified Mobile Inbox with AI Drafts

## Problem Statement
Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by managing customer inquiries across multiple channels (Instagram DMs, email, website chat, WhatsApp). They lose track of conversations, miss leads, and spend hours manually typing similar responses. They need a single, mobile-first inbox where all messages arrive, and they need AI to draft responses so they can just review and send.

## Research Report
Based on a deep competitor audit and SMB pain point analysis:
- **Pain Point #2**: "Managing Customer Inquiries Across Channels" is a top complaint.
- **Competitor Gap**: Shopify requires paid third-party apps for unified inboxes. Wix has a basic inbox but lacks proactive AI drafting. Durable has a CRM but the AI assistance is less department-focused.
- **AI Differentiation**: "Autonomous Customer Inquiry Drafting" by the Customer Success agent is a core OHC differentiator. It saves hours of manual work and ensures professional, prompt replies based on the business's specific knowledge base.

## Design Doc
- **Core Entity**: `UnifiedMessage` (aggregates messages from various sources: Email, Instagram, Web Chat).
- **Key Relationships**: `UnifiedMessage` links to `Tenant` and `Customer`. `AiDraft` links to `UnifiedMessage`.
- **UI Wireframes/Flow (Mobile-First 375px)**:
  - **Inbox List**: Clean list of conversations, sorted by recency. Unread indicator. Source icon (e.g., IG logo, envelope).
  - **Conversation View**: Chat bubble UI.
  - **AI Draft Area**: At the bottom of the conversation view, above the keyboard. Shows a generated draft with a clear "Approve & Send" button and an "Edit" button.
  - **Action Menu**: Swipe left on a message to mark as resolved or flag for follow-up.
- **AI Agent Integration**: The Customer Success ("The Ambassador") agent listens to incoming messages, queries the tenant's knowledge base (past interactions, policies, product catalog), and generates a draft reply.

## Implementation Prompt
Implement a unified mobile inbox feature. The system should aggregate messages from multiple mock channels into a single feed. When a new message arrives, the Customer Success agent should automatically generate a draft response based on the context. The user interface must be mobile-first (designed for 375px screens) and present the draft clearly for 1-tap approval or editing before sending. Ensure the flow covers receiving a message, viewing the AI draft, and sending the reply.

## Priority
P0

## Estimated Scope
Medium
