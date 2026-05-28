# [marketing] Proactive Agentic Social Manager

## Problem Statement
Maya (baker, 28) and Priya (boutique owner, 35) suffer from "Marketing Burnout." They sell through Instagram DMs because setting up a "real" store feels like a full-time job. Even with a store, they struggle to post consistently. They need an agent that doesn't just "schedule" posts, but "creates" them based on what's happening in the shop.

## Research Report
- **Competitive Audit**:
  - **Durable**: Generates "Post Templates," but the user still has to decide *what* to post and *when*.
  - **Hostinger AI**: Has a "Blog Generator," but no proactive social media hook.
  - **Shopify Sidekick**: Can write a caption if asked, but doesn't monitor inventory to suggest a "Low Stock Alert" post.
- **Direct User Quotes**:
  - "I have 5 new cakes this morning but I'm too tired to even open Instagram. I wish someone would just do it for me." - *Maya (Persona Evidence).*
  - "Social media feels like I'm screaming into a void, and I never know if it actually sells anything." - *r/smallbusiness User.*
- **Data**: Consistent social posting (3x/week) increases SMB conversion by 40% (Shopify Pulse 2024), yet only 12% of solopreneurs maintain this frequency for more than 6 months.

## Design Doc
### High-Level Architecture
```mermaid
graph TD
    A[SIPDB Inventory Change] --> B[Trigger: Proactive Agent]
    A[New Lead / Sale] --> B
    B --> C[Creative Agent: AIDA Caption Gen]
    B --> D[Visual Agent: Image Overlay/Remix]
    C --> E[Approval Queue]
    D --> E
    E -->|1-Tap| F[Meta/TikTok/X API]
    G[Performance Analytics] --> B
    Note right of B: Agent learns 'Vibe' from Brand Kit
```
### Mobile UX Flow (375px)
1. **The Pulse**: A horizontal scroll of "Drafts" on the home screen.
2. **Review View**: "I noticed you're low on Sourdough. I drafted a 'Last Call' post. [Post Now] [Edit]".
3. **Brand Consistency**: AI uses the site's "Glassmorphism" design tokens to theme the social images.

## Implementation Prompt
**Outcome**: A "Marketing Pulse" that proactively generates and queues social media content based on inventory, sales, and trending local hashtags.
**Critical User Journey**:
1. Priya adds "Red Silk Dress" to inventory.
2. AI creates an Instagram Story draft with a "New Arrival" sticker and a caption.
3. Priya swipes "Approve."
4. Post is live.
**Acceptance Criteria**:
- Integration with OHC-SIP events for triggers.
- Multi-platform posting (IG, TikTok, FB).
- Feedback loop where the agent adapts to engagement data.

**Priority**: P1
**Estimated Scope**: Medium
