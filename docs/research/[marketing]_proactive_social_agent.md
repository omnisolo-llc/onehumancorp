# [marketing] Proactive Agentic Social Manager

## Problem Statement
Maya (baker) and Priya (boutique owner) are "Instagram-first" businesses. They are overwhelmed by the need to post 3-5 times a week to "stay in the algorithm." They find existing social schedulers (Buffer, Hootsuite) too "manual" — they still have to write the post, pick the photo, and decide the caption. They want an agent that "watches the shop" and creates content for them.

## Research Report
- **Market Gap**: No major SMB platform currently *generates* and *suggests* social content based on real business activity (e.g., "You just added a new dress to inventory, here is a reel script and 3 photos").
- **Competitor Comparison**:
  - **Durable**: Generates ads and "social posts," but they are static templates, not proactive.
  - **Wix**: Has an AI Email Generator, but social is still a "share this page" manual step.
  - **Shopify**: Sidekick can write captions, but doesn't "know" when to post.
- **User Evidence**: "Marketing Burnout" is cited on r/smallbusiness as the #1 reason solopreneurs quit within the first 12 months.

## Design Doc
### High-Level Architecture
- **Inventory Hook**: Monitors SIPDB for `ProductCreated` or `InventoryRestocked` events.
- **Creative Agent**: Uses the "Magic Catalog" assets to generate high-fidelity social media captions (using AIDA formula) and image overlays.
- **Approval Queue**: Content is not posted automatically; it enters a "1-Tap Approval" queue on the mobile dashboard.

### UI/Mobile UX Flow (375px)
1. **The "Pulse" Feed**: A row of "Suggested Posts" at the top of the app.
2. **Quick Preview**: User taps a suggestion -> Sees the generated image/caption.
3. **Approval**: "Post to IG & TikTok" -> Done.

### AI Agent Integration
- **The Social Co-pilot**: An agent that "vibe codes" the social presence based on the website's brand kit. It learns which posts get more engagement and adapts its tone over time.

## Implementation Prompt
**Outcome**: A "Marketing Pulse" that presents the user with 3 ready-to-post social media drafts every Monday morning, based on their actual business data.
**Critical User Journey**: User adds a new product -> AI notices -> AI generates a "New Arrival" Instagram post -> User gets a notification "Ready to post?" -> User taps "Approve."
**Acceptance Criteria**:
- Integration with Meta/TikTok APIs for posting.
- Proactive generation triggered by business events (not just a timer).
- Mobile-first "Swipe to Approve" interface.

**Priority**: P1
**Estimated Scope**: Medium
