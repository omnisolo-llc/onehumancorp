# The Ambassador: AI Omnichannel Inbox

## Problem Statement
Small business owners (like Carlos the Handyman or Maya the Baker) suffer from "Operational Fatigue" and "Communication Lag." They lose 30% of sales because they are too busy working or sleeping to respond to Instagram DMs, WhatsApp messages, and website inquiries. Managing 3 different inboxes is overwhelming and leads to lost revenue and poor customer experience.

## Research Report
- **Pain Point:** Ranked #2 ("Operational Fatigue") and #8 ("Communication Lag") in our Top 10 SMB Pain Points audit.
- **Evidence:** Reddit (r/smallbusiness) shows users describing the "never-ending inbox." App Store reviews for legacy platforms highlight the difficulty of managing communications on mobile devices.
- **Competitor Landscape:** Shopify Sidekick is a reactive chat tool for the merchant, not an autonomous agent handling customers. Wix has basic auto-replies, but not context-aware AI drafting.
- **Strategic Fit:** This directly aligns with OHC's "AI Does the Work" core value and the "Silent Ambassador" pillar of our AI Differentiation Manifesto.

## Design Doc
### High-Level Architecture
- **Entity Types:** `Conversation`, `Message` (multichannel support: IG, WhatsApp, Web), `DraftReply`, `CustomerProfile`.
- **Key Relationships:**
  - A `Conversation` belongs to a `CustomerProfile` and a `Tenant`.
  - A `Message` triggers the `AmbassadorAgent` via the Event Mesh.
- **AI Agent Integration:**
  - The Event Mesh publishes `IncomingMessage` events.
  - "The Ambassador" (Customer Success Department Agent) subscribes, reads the message, retrieves business memory (e.g., pricing, FAQs, previous context via pgvector), and generates a `DraftReply`.
  - The drafted reply is queued in the user's dashboard.

### UI / UX Flow (Mobile-First 375px)
1. **Notification:** The user receives a push notification: "The Ambassador drafted a reply to Maya's custom cake inquiry."
2. **Action Feed (Dashboard):** The user opens the app to an "Action Required" feed.
3. **Review Card:** A clean, glassmorphic card shows the customer's original message (e.g., IG DM) and the AI's proposed response.
4. **1-Tap Action:** The user has two large (≥44px) buttons: "Send" (Approve) or "Edit."
5. **Editing:** If "Edit" is tapped, a native mobile keyboard appears. The user tweaks the text and hits "Send."

## Implementation Prompt
**User-Facing Outcome:** The system should provide a unified, single-screen inbox where business owners see incoming messages from all channels alongside pre-drafted, context-aware AI responses. The owner can approve and send the response with a single tap.

**Critical User Journey:**
1. A customer sends a message via Instagram DM ("Do you offer vegan options?").
2. The system ingests the message and triggers the AI Ambassador.
3. The AI Ambassador drafts a reply based on the store's inventory and FAQs ("Yes, we have vegan chocolate and vanilla options!").
4. The business owner opens the OHC mobile app, sees the drafted reply in their action feed, and taps "Approve and Send."
5. The reply is routed back and sent to the customer via Instagram DM.

**Acceptance Criteria:**
- The inbox unifies messages from at least two sources (e.g., a web widget and mock IG).
- The AI Ambassador successfully generates a context-aware draft for incoming messages.
- The UI provides a 1-tap approval flow for drafted responses.
- The UI is fully functional and visually sound on a 375px viewport, utilizing native keyboards for any editing.

## Metadata
- **Priority:** P0
- **Estimated Scope:** Large
