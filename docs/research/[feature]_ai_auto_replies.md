# [Feature] AI Auto-Replies for SMB Communications

## Problem Statement
Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by incoming messages across multiple channels (Instagram DMs, email, website chat). They miss leads when busy and spend hours answering repetitive questions about pricing, availability, and simple booking inquiries. The current OHC platform lacks built-in email marketing and AI features that directly address this communication burden, forcing owners to rely on manual replies or disjointed 3rd party tools.

## Research Report
### Validation
- **SMB Pain Points**: "Customer management is a mess", "I lose track of customer inquiries in DMs", and "I spend hours answering the same questions."
- **Feature Gap Analysis**: Shopify Sidekick acts as a chatbot but not a true invisible agent. Wix ADI is a one-time website builder. OHC has a significant opportunity to leapfrog competitors by offering true autonomous reply agents.
- **AI Differentiation**: Auto-replying to customer messages is identified as a top-priority automation that saves business owners hours per day and immediately recovers potential lost leads.

## Design Doc
### High-Level Architecture
- **Entity Types**: `CustomerMessage`, `AutoReplyAgent`, `KnowledgeBase` (pricing, FAQs, business hours), `CommunicationChannel` (IG, Email, Web).
- **Key Relationships**: An `AutoReplyAgent` is linked to a business `KnowledgeBase` and monitors multiple `CommunicationChannels`. When a `CustomerMessage` is received, the agent generates a response based on the `KnowledgeBase`.
- **Integration Points**: Needs to integrate with external platforms (Instagram API, Email providers) and the internal unified inbox system.

### Mobile UX Flow (375px first)
1. **Setup Screen**: Owner taps "Enable AI Assistant".
2. **Knowledge Base Input**: Owner uploads a menu, types a quick bio, or sets business hours.
3. **Channel Connection**: 1-click toggle to connect Instagram DMs or Email.
4. **Approval Mode**: Owner can set the agent to "Draft Only" (requires 1-tap approval before sending) or "Auto-Send".

## Implementation Prompt
**User-Facing Outcome**: "Your AI assistant is now managing your inbox. It will answer common questions based on your business info and draft replies for complex inquiries."
**Critical User Journey**:
1. User enables the AI Auto-Reply feature.
2. User provides basic business context (e.g., "I'm a baker, cakes start at $50, I need 2 days notice").
3. A customer sends an IG DM: "How much for a custom cake?"
4. The AI instantly drafts a reply: "Hi! Custom cakes start at $50 and require 2 days notice. Would you like to start an order?"
5. The user reviews and approves the draft with one tap on their phone.
**Acceptance Criteria**:
- Must support at least one mock/sandbox channel for testing.
- Must accurately parse basic business rules to generate a draft reply.
- Must have a simple mobile-first UI for reviewing and approving drafts.
- Must not hallucinate pricing or availability.

## Priority
P0

## Estimated Scope
Medium
