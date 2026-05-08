# Title: Unified Inbox & Auto-Reply Agent

## Problem Statement
Small business owners like Leo (the music tutor) and Priya (the boutique owner) manage customer inquiries across multiple fragmented channels: Instagram DMs, Facebook Messenger, WhatsApp, email, and SMS. They constantly switch between apps, causing delayed responses and lost leads. Furthermore, many questions are repetitive ("What are your hours?", "Do you have this in size M?"). They need a single, unified inbox that aggregates all messages, coupled with an AI agent that can automatically handle routine inquiries and escalate complex ones, saving them hours per day.

## Research Report
- **Competitive Landscape**:
  - **Shopify**: Offers "Shopify Inbox", which centralizes chats and emails. It has basic automated replies but lacks true AI-driven conversational commerce capabilities.
  - **Meta Business Suite**: Aggregates FB/IG messages but doesn't integrate well with external inventory or booking systems.
  - **ManyChat / Intercom**: Powerful but far too complex for the average non-technical SMB owner to configure.
- **User Pain Points**:
  - "I spend 2 hours every night just answering Instagram DMs."
  - "Customers expect instant replies, but I'm busy actually doing the work." (Themes from r/smallbusiness and YouTube creator interviews).
- **Opportunity**: An invisible AI agent that sits in a unified inbox, understands the business's context (inventory, hours, policies), and autonomously answers 80% of routine questions without the owner lifting a finger.

## Design Doc
- **High-Level Architecture**:
  - **Channel Integration**: Connectors for Instagram Graph API, Facebook Messenger, WhatsApp Business, Email, and SMS.
  - **Unified Inbox UI**: A single chronological feed of all customer interactions, regardless of source.
  - **Auto-Reply Agent**: An AI model with access to the business's knowledge base (catalog, FAQ, business hours). It intercepts incoming messages, determines confidence in an answer, and replies automatically if confidence is high.
- **Mobile UX Flow (375px first)**:
  1. User opens the "Inbox" tab.
  2. Messages are grouped by customer, showing the source icon (IG, Email).
  3. Messages handled by the AI have a small "AI Handled" badge.
  4. For unhandled messages, the AI suggests 3 possible replies as one-tap buttons.
- **AI Agent Integration Points**: The agent monitors incoming webhook events from communication channels, queries the business knowledge base, and posts replies back via the respective APIs.

## Implementation Prompt
Develop a unified inbox that aggregates messages from various channels (starting with a simulated integration or one core channel like email/SMS) and integrate an Auto-Reply Agent. The agent should be able to answer basic questions based on a provided business context document. The critical user journey involves a customer sending a message, the AI automatically replying if it knows the answer, and the business owner seeing the interaction logged in their unified inbox view. Do not prescribe specific database schemas or API contracts.

## Priority
P1

## Estimated Scope
Medium
