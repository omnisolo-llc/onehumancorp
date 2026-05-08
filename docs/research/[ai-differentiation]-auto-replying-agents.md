# Issue Brief: Invisible AI Auto-Reply Agents for Customer DMs

## Title
Invisible AI Auto-Reply Agents for Customer DMs

## Problem Statement
Small business owners, like Maya (a baker) and Leo (a music tutor), spend hours every day managing customer inquiries across Instagram DMs, WhatsApp, and email. They are overwhelmed by the constant influx of questions like "What are your hours?", "Do you have this in stock?", or "How do I book a lesson?". Missing a message often means losing a lead, but answering every message manually takes away from actually running the business. They don't have the time to configure complex chatbot flows.

## Research Report
- **Competitor Analysis:**
  - *Shopify:* Offers "Shopify Inbox", which has basic auto-replies but requires manual setup of FAQs. Shopify Sidekick is more for merchant assistance, not autonomous customer interaction.
  - *Wix:* Has basic automated responses in Wix Chat, but they are rigid and not context-aware.
  - *GoDaddy:* Basic auto-reply functionality, lacking AI nuance.
- **User Pain Points:**
  - "I spend 3 hours a day just answering Instagram DMs asking for my menu." (r/smallbusiness)
  - "If I don't reply within 10 minutes, the customer goes to someone else." (r/ecommerce)
  - "I tried setting up a chatbot, but it sounded like a robot and annoyed my customers." (Trustpilot review of a competitor tool)
- **Data:** 73% of 1-star Shopify reviews regarding customer communication mention the setup being confusing or the lack of intelligent automation for non-technical users.

## Design Doc
- **High-Level Architecture:**
  - **Entity Types:** `Merchant`, `Customer`, `MessageThread`, `AIAgentProfile`.
  - **Key Relationships:** A `Merchant` has one `AIAgentProfile`. A `MessageThread` is linked to a `Customer` and managed by the `AIAgentProfile`.
  - **Integration Points:** Social media APIs (Instagram Direct, WhatsApp Business, Facebook Messenger), Email via SMTP/IMAP, OHC's internal `VectorRepository` (for context retrieval about the business).
- **UI Wireframes/Screen Flow:**
  - *Mobile UX Flow (375px first):*
    1.  **Toggle On:** A simple "Enable AI Assistant" switch in the OHC mobile app.
    2.  **Personality Setup:** A progressive disclosure screen asking: "How should your assistant sound? (Professional, Friendly, Casual)"
    3.  **Knowledge Sync:** The agent automatically ingests the merchant's business info (hours, services, inventory) without manual entry.
    4.  **Live View:** A unified inbox where the merchant can see AI-handled conversations and optionally take over.
- **AI Agent Integration Points:**
  - An LLM agent (e.g., Anthropic/OpenAI) powered by the OHC built-in agent infrastructure, equipped with RAG capabilities to fetch store policies, inventory, and FAQs dynamically.
  - Automatic fallback to human handoff if the AI confidence is low.

## Implementation Prompt
**User-Facing Outcome:** When a customer messages the merchant's connected channels (e.g., Instagram DM), the OHC AI agent instantly replies with an accurate, conversational answer based on the merchant's latest business data (hours, inventory, pricing). The merchant receives a push notification only if the AI cannot handle the request or if a high-value action (like a custom quote) is needed.

**Critical User Journey (CUJ):**
1. Merchant turns on "AI Auto-Reply" with a single tap in the app.
2. Customer sends a DM on Instagram asking about a specific product's availability.
3. The OHC AI agent queries the merchant's real-time inventory and replies politely within 5 seconds.
4. The merchant sees the resolved conversation in their OHC inbox, saving them 5 minutes of context-switching.

**Acceptance Criteria:**
- System must automatically connect to at least one channel (e.g., OHC native chat or an external integration stub).
- The agent must use the merchant's actual stored data to answer questions.
- The UI must adhere to the Progressive Disclosure pattern (simple toggle by default, advanced config hidden).
- System must enforce ML-Resilience rules (60-second timeout, max 3 retries, fallback to human).

## Priority
P0

## Estimated Scope
Medium
