# Autonomous Auto-Reply for Unified Inboxes

## Title
Unified Inbox & Autonomous Auto-Reply Agent

## Problem Statement
Small business owners (like Maya the baker) spend hours daily manually replying to the same questions across Instagram DMs, WhatsApp, and SMS ("What are your hours?", "Do you have vegan options?", "How much for a custom cake?"). This leads to burnout, delayed responses, and lost sales. Managing multiple apps on a phone is chaotic.

## Research Report
- **Sources:** Reddit (r/smallbusiness), Trustpilot, Shopify Community.
- **Data:** 60% of inbound messages for service/local retail businesses are routine FAQs.
- **Competitive Gap:** Existing tools like Shopify provide standard chatbots, but they often struggle to sound human or access real-time inventory contexts natively.
- **Finding:** Users don't want a "chat widget"; they want an invisible agent to handle DMs just like an actual employee would.

## Design Doc
### Architecture
- **Unified Messaging Gateway:** Ingests messages from Meta Graph API (Instagram/Messenger), WhatsApp Business API, and Twilio (SMS).
- **Agent Context:** The agent must have read access to the business's opening hours, catalog, pricing, and current inventory.
- **Approval Queue:** For complex queries or custom orders, the agent flags the message for human review in the "Needs Attention" queue.

### Mobile UX Flow (375px)
1. **Inbox View:** A single, clean list of all conversations, regardless of source channel.
2. **Conversation Thread:** Messages from the customer appear normally. Messages sent by the AI are badged as "Sent by Agent".
3. **Drafting:** For messages flagged for review, the AI presents a drafted response. The user simply taps "Approve & Send" or edits the text.

## Implementation Prompt
**User-Facing Outcome:** When a customer sends an Instagram DM asking "Are you open on Sunday?", the OHC system intercepts the message, checks the business profile, and automatically replies, "Yes! We're open from 9 AM to 3 PM this Sunday. Can I help you order something?" without the owner lifting a finger. If a customer asks a complex question ("Can you make a cake that looks like a 1992 Honda Civic?"), the agent places it in an "Action Required" tab for the owner.

**Acceptance Criteria:**
- Connect at least one external channel (e.g., simulated Instagram DM).
- Agent correctly replies to a basic FAQ based on business data.
- Agent correctly flags a complex request for manual owner intervention.
- The UI must consolidate all messages into one unified view.

## Priority
P0

## Estimated Scope
Large
