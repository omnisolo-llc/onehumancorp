# [Customer Success] Omnichannel AI Inbox

## Title
Omnichannel AI Inbox (The Ambassador)

## Problem Statement
Small business owners (like Maya the Baker) suffer from "Scattered Inbox Syndrome." They receive inquiries across Instagram DMs, WhatsApp, SMS, and Email. Switching between apps to answer repetitive questions ("Do you have vegan options?", "When are you open?") is exhausting and leads to missed sales.

## Research Report
Based on the OHC Market Dominance Research Report, this is the #1 pain point for non-technical owners.
Existing solutions like Shopify require expensive 3rd-party apps for unified inboxes, and even then, they lack deep AI integration.
**Cloud vs. Standalone Capability:**
- **Cloud:** The cloud version will connect to external Webhooks (Meta Graph API for IG/WhatsApp, Twilio for SMS) to receive messages in real-time. The Autodream memory layer will provide context for the AI draft.
- **Standalone:** The local desktop app will poll a simplified sync server or rely on direct local OAuth tokens where feasible, ensuring privacy. Local LLMs (via MCP) could be utilized for drafting if cloud connectivity drops, falling back to basic templated replies.

## Design Doc
**Target Viewport (375px native mobile first):**
- **Unified List View:** A clean, Apple Mail-style list of conversations. Badges indicate the source (IG icon, SMS icon). Unread messages are bolded.
- **Conversation View:** Standard chat interface.
- **AI Drafting Feature ("The Ambassador"):** When a user opens a message, a translucent glass panel (UniFi/Apple style) slides up showing an AI-drafted reply based on `autodream_memories`.
- **1-Tap Action:** A prominent primary button to "Send Draft" or "Edit".
- **Settings:** A simple toggle: "Auto-Reply when sleeping." No complex routing rules exposed.

## Implementation Prompt
Create a unified inbox component that displays messages from multiple channels. Integrate the Autodream agent to analyze incoming text and provide a suggested reply draft. Provide a UI for the user to approve, edit, or reject the draft. Ensure the UI strictly follows the OHC Visual Excellence Mandate (translucent glass, Apple/UniFi curves).

## Priority
High (P0)

## Estimated Scope
3 weeks (Backend integration + UI implementation + Agent prompt tuning)
