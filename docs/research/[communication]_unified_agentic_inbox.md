# Title: Unified AI Agentic Inbox for Cross-Channel Communication

## Problem Statement
Small business owners like **Maya (baker)** and **Carlos (handyman)** are overwhelmed by communication silos. They receive inquiries via Instagram DMs, SMS, WhatsApp, emails, and website forms. Maya spends hours daily manually copying orders from Instagram DMs into a spreadsheet, often missing requests or losing track of context. Carlos misses leads because he cannot actively manage multiple inboxes while on the job. The sheer volume and fragmentation of communication are exhausting and directly cause lost revenue. Existing platforms like Shopify do not offer a natively unified inbox, let alone one that automatically handles routine inquiries without human intervention.

## Research Report
*   **Competitor Gap:**
    *   **Shopify:** Relies heavily on third-party apps for omnichannel inboxes (e.g., Gorgias), which are expensive and complex to set up. Their native "Shopify Inbox" is primarily for website chat.
    *   **Wix/Squarespace:** Offer basic centralized inboxes, but they lack autonomous agents to truly act *on behalf* of the business owner.
    *   **GoDaddy:** Basic communication tools, heavily siloed.
*   **User Pain Points (Validated from App Store/Reddit Reviews):**
    *   "I missed a $500 catering order because the Instagram DM got buried."
    *   "Switching between Facebook, Instagram, and email to answer the same 'what are your hours' question is killing me."
    *   "I need an assistant but can't afford one."
*   **The OHC Opportunity:** By integrating all channels (Instagram, SMS, WhatsApp, Web, Email) into a single feed and letting an AI Agent immediately draft (or auto-send) context-aware responses (e.g., "Yes, we are open until 6 PM" or "We have 3 chocolate cakes left"), OHC can save business owners hours per day.

## Design Doc
*   **Core Entity Types:**
    *   `UnifiedConversation`: Represents a thread regardless of source.
    *   `Message`: Individual communication items linked to a `UnifiedConversation`.
    *   `ChannelType`: Enum (Instagram, WhatsApp, SMS, Email, WebChat).
    *   `AgentDraft`: An AI-generated proposed response or action.
*   **Key Relationships:**
    *   `UnifiedConversation` belongs to a `Tenant` (the business).
    *   `Message` belongs to a `UnifiedConversation`.
    *   `AgentDraft` belongs to a `Message`.
*   **Mobile UX Flow (375px First):**
    1.  User opens the OHC app and sees a single "Inbox" tab with unread badges.
    2.  Tapping a conversation reveals the thread. A clear icon indicates the source (e.g., small Instagram logo).
    3.  If the AI has a high-confidence answer (e.g., pricing inquiry), a "Drafted by Agent" bubble is pre-filled in the text box. The user can simply hit "Send" or edit it.
    4.  If the AI recognized an order intent, a "Create Order" button appears inline in the chat.
*   **AI Agent Integration Points:**
    *   On incoming message, the event is routed to the conversational agent.
    *   The agent analyzes the intent (FAQ, order request, complaint) and checks business context (inventory, hours).
    *   The agent generates a draft reply or takes an action (e.g., creating a draft order) and notifies the user.

## Implementation Prompt
**User-Facing Outcome:** The small business owner opens their OHC dashboard and sees all customer messages from Instagram, SMS, email, and their website in one single list. Crucially, when they open a message asking about pricing or availability, an AI has already written a perfect, context-aware reply. The owner just taps "Send."

**Critical User Journey:**
1.  Customer sends a DM on Instagram asking, "Do you do custom cakes for weddings?"
2.  The message appears instantly in the OHC Unified Inbox.
3.  The OHC AI Agent analyzes the message, checks the business's known services, and drafts a reply: "Yes, we do custom wedding cakes! Our pricing starts at $200. Would you like to schedule a consultation?"
4.  The business owner opens the app, sees the draft, and taps "Approve and Send."
5.  The customer receives the reply seamlessly on Instagram.

**Acceptance Criteria:**
*   A user can view messages from at least two different simulated channels in a single, unified view.
*   Incoming messages automatically trigger an AI agent to generate a draft response if the intent is recognized.
*   The generated draft response is visible to the user and can be approved/sent with a single tap.
*   The UI must prioritize the 375px mobile experience.

## Priority
P0

## Estimated Scope
Large