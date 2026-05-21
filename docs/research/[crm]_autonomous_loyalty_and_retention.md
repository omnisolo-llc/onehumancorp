# [crm] Autonomous Loyalty & Retention Agent

## Title
**Zero-Click Abandoned Cart & Personalized Follow-up Engine**

## Problem Statement
Non-technical solopreneurs know they are losing sales to abandoned carts or "ghosted" service inquiries (like Carlos the handyman missing quotes or Leo the tutor losing recurring students), but setting up automated marketing flows in Mailchimp or Klaviyo is too complex and expensive. They need a system that does the follow-up for them.

## Research Report
- **Evidence:** 73% of 1-star Shopify reviews complain about the need to install and configure multiple apps. Abandoned cart tools are often a paid add-on or require designing complex email flows.
- **Competitor Flaw:** Platforms provide the *tools* to build automation (like Shopify Flow), but the user still has to be the architect.
- **OHC Opportunity:** OHC will provide a "Marketer" agent that automatically detects drop-offs (abandoned carts, unaccepted quotes, lapsed bookings) and drafts a personalized follow-up message (SMS/Email) with an optional dynamic discount, requiring only a 1-tap approval from the owner.

## Design Doc
- **Key Entities:** `Cart`, `Quote`, `Booking`, `Customer`, `CommunicationEvent`.
- **Agent Integration:** `Marketing Agent` and `Customer Success Agent`.
- **UI Flow:**
  - Customer abandons a cart containing 2 vanilla cupcakes.
  - 2 hours later, the OHC Marketer Agent drafts a message: "Hi Sarah, looks like you left some cupcakes behind! Here's 10% off if you complete your order today."
  - Agent pushes an interactive notification to Maya's phone: "Sarah abandoned a cart. Send a 10% discount to recover it?" [Send] / [Edit] / [Ignore].
  - Maya taps [Send]. The system dispatches the message via the customer's preferred channel (SMS/Email).
- **Architecture:** The state machine monitors active sessions/quotes. If they expire without conversion, an event triggers the agent to generate the draft and queue it in the Unified Inbox for approval.

## Implementation Prompt
**User-Facing Outcome:** The small business owner recovers lost revenue through personalized follow-ups without ever having to write an email sequence or configure a workflow.
**Critical User Journey:**
1. A potential customer abandons a transaction (cart, quote, or booking).
2. The system detects this and the AI agent automatically drafts a highly contextual recovery message.
3. The user receives a push notification on their mobile device to approve the message with a single tap.
4. Upon approval, the message is sent.
**Acceptance Criteria:**
- The system must detect abandoned transactions and trigger an agent task.
- The agent must generate a natural-sounding message referencing the specific items or services.
- The user must be able to approve the message via a 1-tap mobile interface.

## Priority
P1

## Estimated Scope
Medium
