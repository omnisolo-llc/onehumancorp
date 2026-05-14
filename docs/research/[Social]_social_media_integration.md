# [Social] OHC Tool Integration Research Brief: Social Media Unified Inbox

## Title
Unified Inbox for Social Media DMs (Instagram, Facebook, WhatsApp)

## Problem Statement
Small business owners manage customer inquiries across multiple platforms: Facebook Messenger, Instagram DMs, WhatsApp, and email. Jumping between these apps on their phone while trying to run a business leads to missed messages, slow response times, and lost sales. They need a single place to view and respond to all customer messages.

## Research Report
The social media integration landscape for messaging is dominated by a few key players, largely dictated by Meta's ecosystem.

**Evaluated Tools:**

1. **Manychat**
    *   **Focus:** Chat marketing and automation for Instagram, Facebook, WhatsApp.
    *   **Pros:** Incredible visual automation builder. Very popular with SMBs and creators.
    *   **Cons:** It's a closed ecosystem designed for automation, not primarily as a dumb pipe for a third-party CRM to consume.
    *   **Pricing:** Pro starts at $15/mo.
    *   **Modes:** Cloud only.

2. **Intercom**
    *   **Focus:** Enterprise/Mid-market customer service helpdesk.
    *   **Pros:** Unbelievably powerful, AI agent (Fin) is market-leading.
    *   **Cons:** Way too expensive and complex for a typical small business owner.
    *   **Pricing:** Starts at $39/seat/mo, but full features easily cost hundreds.
    *   **Modes:** Cloud only.

3. **MessageBird (Bird)**
    *   **Focus:** Omnichannel communications.
    *   **Pros:** Supports all major channels. Excellent for building a custom unified inbox inside OHC.
    *   **Cons:** Requires OHC to build the entire chat UI.
    *   **Pricing:** Pay-as-you-go.
    *   **Modes:** Cloud and Standalone.

**Recommendation:**
To provide a unified inbox inside OHC, we need to integrate directly with an Omnichannel provider. **MessageBird** or Twilio are the standard choices here. Direct integration with **Meta's Graph API** (for FB/IG) and **WhatsApp Business API** is often the most cost-effective long-term strategy, despite the initial overhead.

## Design Doc
**Integration Approach: Direct Social Media Integration**

1.  **Authentication (Trigger):**
    *   The business owner clicks "Connect Facebook/Instagram" in OHC settings.
    *   They grant OHC permission to read and manage their Page/Account messages.

2.  **Message Ingestion (Action):**
    *   OHC receives notifications from the social media platforms when a new message arrives.
    *   OHC creates a unified "Conversation" record linked to the specific Customer (if known) or creates a new Lead.
    *   The message appears in the OHC Inbox UI.

3.  **Message Response (User View):**
    *   The business owner sees the message in the OHC Inbox, badged with the social media logo.
    *   They type a reply and hit send.
    *   OHC delivers the reply back to the customer's social media DM.

## Implementation Prompt
**Objective:** Implement the foundation for a Unified Inbox by creating the data models and a receiver for social media messages.

**Acceptance Criteria:**
1.  Create database models for Conversation and Message to track interactions across different channels.
2.  Implement a generic mechanism to receive incoming messages from external platforms.
3.  The receiver must create a new Conversation if one doesn't exist for the external identifier, and append the new Message.
4.  Implement a mechanism to simulate sending a message back to the external platform.

## Priority
P1

## Estimated Scope
Large
