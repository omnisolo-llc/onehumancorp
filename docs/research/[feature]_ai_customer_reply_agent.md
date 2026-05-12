# AI Customer Reply Agent

## Title
Invisible AI Customer Reply Agent for Unified Inbox

## Problem Statement
Small business owners like Maya (baker) manage customer inquiries across multiple fragmented channels: Instagram DMs, WhatsApp, Email, and SMS. Constantly monitoring these platforms and replying to repetitive questions (e.g., "What are your hours?", "Do you do custom cakes?") consumes hours daily, distracting from core business operations and leading to lost leads if responses are slow.

## Research Report
Competitor analysis reveals that tools like Shopify Inbox provide rudimentary saved replies but lack intelligent, autonomous response capabilities. Tools like ManyChat are powerful but too complex for a non-technical SMB owner to configure (requires building complex decision trees). Reddit's r/smallbusiness frequently features complaints about "DM overwhelm" and the need to hire VAs just to answer basic questions. OHC must provide an AI agent that automatically drafts intelligent responses to common inquiries based on the business's context (hours, inventory, FAQs) without requiring the owner to build a bot.

## Design Doc
**High-Level Architecture:**
- **Entity Types:** `Conversation`, `Message`, `BusinessContext`, `AIResponseDraft`.
- **Key Relationships:** A `Conversation` contains multiple `Message`s and belongs to a specific channel (IG, WhatsApp).
- **Integration Points:** Meta Graph API (IG/FB), Twilio (WhatsApp/SMS), Email ingress.
- **Mobile UX Flow (375px first):**
  1. A customer sends a DM on Instagram.
  2. The message arrives in the OHC Unified Inbox.
  3. The AI agent instantly analyzes the message against `BusinessContext` and drafts a reply.
  4. The owner receives a push notification: "New Inquiry from [Name]. Tap to send AI reply."
  5. The owner reviews the drafted text in a simple chat bubble and taps "Send" (or edits it if needed).
- **AI Agent Integration:** An LLM-powered router intercepts incoming messages, fetches relevant business data (e.g., store hours from the database), and generates contextual `AIResponseDraft`s.

## Implementation Prompt
Build an AI-powered conversational agent integrated into the OHC unified inbox. The agent should monitor incoming messages from all connected channels and automatically generate suggested replies based on the business's predefined information (hours, location, active products). Ensure the UI allows the business owner to review, edit, and send the AI-drafted reply with a single tap on a mobile device.
Acceptance Criteria:
- Incoming messages trigger an AI draft generation in the background.
- Drafts are highly visible and easy to approve or discard in the mobile UI.
- The system prevents auto-sending by default; explicit 1-tap owner approval is required.

## Priority
P0

## Estimated Scope
Large
