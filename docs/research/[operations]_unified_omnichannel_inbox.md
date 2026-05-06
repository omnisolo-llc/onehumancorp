# Title: Unified Omnichannel Inbox (The Ambassador Agent)

## Problem Statement
Small business owners suffer from "Operational Fatigue." They miss sales because they cannot respond to Instagram DMs, WhatsApp messages, emails, and SMS fast enough, especially while they are working, sleeping, or managing customers in-person. Responding to the same 5 questions across 3 different apps is exhausting and inefficient.

## Research Report
- **Competitor Landscape**:
  - Shopify: Relies heavily on the App Store for omnichannel support, leading to "Cost Creep" and fragmented experiences.
  - Wix: Built-in inbox, but mostly reactive and requires manual human response.
- **User Pain Points Data**:
  - Operational Fatigue is the #2 pain point (68% frequency).
  - Communication Lag is ranked #8 (40% frequency), directly causing lost revenue.
  - Users explicitly mention "the never-ending inbox" on Reddit (r/sidehustle, r/ecommerce).
- **Sources**: Synthesis of Reddit, Trustpilot, App Store reviews.
- **Opportunity**: OHC can differentiate by not just aggregating messages, but actively managing them using "The Ambassador" AI agent to auto-reply or draft responses.

## Design Doc
- **High-Level Architecture**:
  - Omnichannel Event-Mesh Aggregator.
  - "The Ambassador" (Customer Success Agent).
- **Key Relationships & Integration Points**:
  - Integrates with Meta Graph API (Instagram/FB), Twilio (SMS), Resend/Sendgrid (Email).
  - Connects to the Order and Booking entities to answer questions like "Where is my order?"
- **UI/UX Flow (Mobile 375px First)**:
  - Screen 1: Single Inbox view showing messages from all channels natively.
  - Screen 2: Thread view. If AI has handled the query, it is marked as "Resolved by Agent." If it requires approval, it is marked "Draft Ready."
  - Screen 3: Background Draft & Approve. User sees AI proposed response, taps "Send" or edits quickly.
- **AI Agent Integration Points**:
  - Proactive Customer Support Agent parses incoming messages, checks business context (policies, inventory, order status), and acts (auto-replies or drafts).

## Implementation Prompt
**User-Facing Outcome:** The business owner receives all customer communications in one place, with common questions automatically answered by their AI Ambassador, saving hours of manual replies.
**Critical User Journey (CUJ):**
1. Customer asks via Instagram DM: "Are you open tomorrow?"
2. OHC Meta Integration routes the message to the Unified Inbox.
3. The Ambassador Agent checks business hours.
4. AI drafts a reply: "Yes, we are open tomorrow from 9 AM to 5 PM!" and holds for approval (or auto-sends based on settings).
5. Business owner taps "Approve" from their mobile push notification.
**Acceptance Criteria:**
- Inbox aggregates messages from at least two sources (e.g., Email, Meta/Instagram).
- AI agent correctly identifies intent and drafts context-aware responses.
- User can approve, edit, or reject drafted messages easily on mobile.

## Priority
P1

## Estimated Scope
Medium