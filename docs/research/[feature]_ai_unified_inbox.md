# [Feature] AI Unified Inbox

## Problem Statement
Small business owners (like Carlos the handyman or Priya the boutique owner) are overwhelmed by scattered customer communications. They receive inquiries via Instagram DMs, Facebook Messenger, SMS, and email. Managing multiple inboxes leads to missed messages, delayed responses, and lost sales. The pain point is "Operational Fatigue"—they spend hours jumping between apps instead of running their business.

## Research Report
- **Source**: Reddit r/smallbusiness, Trustpilot reviews for existing platforms.
- **Data Point**: "I keep losing leads because they message me on Instagram and I forget" is a recurring theme. Cross-referencing App Store reviews shows a high demand for a single communication hub.
- **Competitor Landscape**:
  - Shopify has "Shopify Inbox", but it's often clunky and requires manual operation.
  - Wix offers an inbox, but it lacks advanced AI auto-drafting capabilities.
- **Recommendation**: OHC should build an AI Unified Inbox because consolidating communication channels into one interface and leveraging AI to auto-draft responses directly addresses the #2 cited pain point across our target personas.

## Design Doc
- **Core Entities**: Messages, Conversations, Channels (Email, SMS, IG, FB), Customers.
- **Key Relationships**: A Customer has many Conversations. A Conversation belongs to a Channel and contains many Messages.
- **UI Wireframes/Flow**:
  - **Mobile First (375px)**: A single feed of all incoming messages, categorized by urgency or status (e.g., "Needs Reply").
  - **Message View**: Displays the conversation thread. Below the thread, an "AI Suggested Reply" card appears.
  - **Action**: The user can tap the suggested reply to send it instantly, edit it, or type a manual response.
- **AI Integration**:
  - An autonomous agent monitors incoming webhooks from connected channels.
  - The agent analyzes the message intent, checks business context (e.g., pricing, availability), and generates a draft response.

## Implementation Prompt
Implement a unified inbox interface that aggregates messages from multiple sources (Email, SMS, Social Media). The interface must prominently feature AI-suggested replies for every incoming customer message. The primary user journey involves opening the OHC app, viewing the combined message list, reviewing an AI-drafted response for a new inquiry, and sending it with a single tap. The solution must be fully functional on mobile devices and prioritize speed and simplicity.

## Priority
P0

## Estimated Scope
Large
