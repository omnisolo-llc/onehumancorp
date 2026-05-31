# [Strategy] SMB Platform Agentic Solutions

## Title
Implement the "Ambassador" and "Advisor" AI Agents to resolve SMB omnichannel and "blank canvas" paralysis.

## Problem Statement
Small business owners (like Maya the baker and Carlos the handyman) face two major pain points that traditional platforms like Shopify and Wix fail to address:
1. **Omnichannel Chaos (38% frequency):** SMBs miss sales because customer inquiries are fragmented across Instagram DMs, Facebook, WhatsApp, and website chat. The overhead of managing these channels manually is too high.
2. **"Blank Canvas" Paralysis (73% frequency):** After setting up a store, non-technical users do not know how to drive traffic or sales. They lack actionable, proactive guidance and feel abandoned post-launch.

## Research Report
An exhaustive audit of the SMB platform market reveals a significant gap. Traditional giants (Shopify, Wix, Squarespace) rely on complex app ecosystems (e.g., Zendesk, Klaviyo) or passive chatbots (Shopify Sidekick) that require user initiation. Emerging AI-native platforms (Durable, 10Web) focus solely on instant website generation but lack ongoing business logic and management tools.

User sentiment analysis across Reddit (`r/smallbusiness`, `r/ecommerce`), Trustpilot, and App Stores indicates that SMBs are overwhelmed by the "App Tax" and the technical jargon required to operate existing platforms.

We propose solving these issues by shifting from reactive tools to proactive, invisible AI agents that handle operations and marketing autonomously, requiring only simple approvals from the user on their mobile device.

## Design Doc
**Entity Types & Key Relationships:**
- `Tenant` (The SMB)
- `Message` (Incoming inquiry from any channel)
- `Conversation` (Thread of messages)
- `AgentAction` (Proposed action by an AI agent)
- `AgentType` (Ambassador, Advisor, etc.)

**High-Level Architecture & Integration Points:**
- **The "Ambassador" (Customer Success):**
  - Integrates with Meta APIs (Instagram, WhatsApp, FB Messenger) and Web Chat.
  - Ingests incoming `Messages` into a unified `Conversation` view.
  - Uses RAG against the `Tenant`'s inventory, policies, and past responses to generate a draft reply or take action (e.g., generate a custom quote).
  - High-confidence replies can be automated; lower-confidence replies are queued as an `AgentAction` for user approval via mobile push notification.
- **The "Advisor" (Business Advisory):**
  - Runs a scheduled job (e.g., weekly) to analyze `Tenant` sales data, traffic, and calendar availability.
  - Generates actionable insights (e.g., "Drafted a promotional email because next Tuesday has no bookings").
  - Pushes an `AgentAction` to the mobile app for 1-tap approval.

**UI Wireframes / Screen Flow (Mobile-First, 375px):**
- **Unified Inbox:** A clean, chat-like interface combining all channels. AI-drafted responses appear in the input field with a distinct visual indicator (e.g., a sparkle icon), ready to be sent or edited.
- **Action Center:** A Tinder-style card interface or simple list where the "Advisor" presents weekly recommendations. The user swipes right or taps "Approve" to execute the marketing action, or "Dismiss" to reject it.

## Implementation Prompt
Implement the backend architecture and mobile-first UI for the "Ambassador" and "Advisor" agents.
- **Ambassador CUJ:** A user receives an Instagram DM asking about a product. The Ambassador agent detects the message, uses RAG to confirm stock, and drafts a reply with a checkout link. The user opens the OHC mobile app, sees the drafted reply in the Unified Inbox, and taps "Send".
- **Advisor CUJ:** The Advisor agent detects a slow sales week. It drafts a promotional SMS to past customers. The user receives a push notification, opens the Action Center on their phone, reviews the proposed SMS, and taps "Approve & Send".
- **Acceptance Criteria:** Ensure all UI layouts are responsive starting at 375px. All agent actions must be auditable and reversible where possible. Implement robust error handling for API integrations (e.g., Meta).

## Priority
P0

## Estimated Scope
Large
