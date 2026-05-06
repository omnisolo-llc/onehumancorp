# [Social Media] Unified Inbox Integration

## Title
Implement Unified Inbox Integration for WhatsApp Business

## Problem Statement
Small business owners, especially those catering to international or non-English-speaking customers, rely heavily on messaging apps like WhatsApp. Currently, they have to constantly switch between their personal phone, the WhatsApp Business app, and the OHC platform to answer customer inquiries, track orders, and provide support. This fragmented experience leads to delayed responses, missed sales opportunities, and burnout for the business owner. They need a single, unified place to see and reply to all customer messages alongside the customer's OHC profile and order history.

## Research Report
### WhatsApp Business API Evaluation
- **Overview:** WhatsApp Business API is the official way for medium and large businesses to communicate with customers on WhatsApp at scale.
- **Key Benefits for SMBs:**
  - **Ubiquity:** WhatsApp is the most popular messaging app globally, with over 2 billion active users. It's the primary means of communication in regions like LATAM, India, and parts of Europe/Africa.
  - **Direct Engagement:** Messages have extremely high open rates compared to email.
  - **Rich Media:** Supports sending images, documents, and locations, which is vital for customer support and order updates.
- **Challenges/Risks:**
  - **Approval Process:** Requires Facebook Business Manager verification.
  - **Pricing Model:** Charges per conversation (user-initiated vs. business-initiated), which can be complex for small businesses to understand and budget for.
  - **Opt-in Rules:** Strict rules around getting customer consent before initiating conversations.
- **Ease of Use for Non-Technical Users:** The API itself is highly technical, so OHC must abstract away the complexity. The business owner should only need to connect their WhatsApp account via an OAuth-like flow and then simply see a chat interface within OHC.
- **Cloud vs. Standalone:**
  - **Cloud:** Highly feasible. Webhooks can be easily routed to the multi-tenant OHC Cloud backend.
  - **Standalone:** Challenging. Webhooks require a public IP or a tunneling service (like ngrok) which is difficult for a standalone, local application. This might require a cloud-relay architecture specifically for webhook delivery to standalone instances.
- **Pricing Estimate:** WhatsApp charges roughly $0.01 - $0.08 per conversation depending on the country. OHC could either pass this cost along or absorb it in a premium tier.

## Design Doc
- **Integration Trigger:** A new "Connect WhatsApp" button in the Settings > Channels page. This initiates an embedded signup flow or OAuth-like connection to link their WhatsApp Business Account.
- **Actions Taken:**
  - Incoming messages from WhatsApp trigger an update in the OHC platform, creating a new "Conversation" thread linked to a specific customer profile based on their phone number.
  - When the business owner replies via OHC, the message is sent back out via the WhatsApp API.
- **User Experience:**
  - The user sees a new "Inbox" tab.
  - The Inbox looks like a standard chat interface (similar to WhatsApp Web or iMessage).
  - Next to the chat, the customer's recent orders, appointments, and notes are displayed.
  - Simple Mode: Just the chat. Advanced Mode: Options to set up automated welcome messages or out-of-office replies.

## Implementation Prompt
Create a "Unified Inbox" feature that allows business owners to connect their WhatsApp Business account. Once connected, incoming WhatsApp messages from customers should appear in a new "Inbox" section within the OHC platform. The business owner must be able to read and reply to these messages directly from OHC. The chat interface should be simple, intuitive, and clearly link the conversation to the corresponding customer record if their phone number matches. Ensure the setup process is a simple click-through flow, hiding all API key and webhook complexities from the user.

## Priority
P1

## Estimated Scope
Large