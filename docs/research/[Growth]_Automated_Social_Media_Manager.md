# Automated Social Media Manager

**Priority:** P0
**Scope:** Large

## Problem Statement
Priya (boutique owner) struggles to keep up with daily Instagram posts and syncing her in-store inventory to social media campaigns. Creating marketing materials takes hours she doesn't have.

## Research Report
- **Shopify:** Complex third-party app ecosystem required.
- **Wix:** Basic built-in tools but manual intervention needed.
- **Data:** 60% of SMBs report social media marketing as a top 3 time sink.
- **Conclusion:** An invisible agent that auto-generates posts based on new inventory and past performance will save owners 5+ hours a week.

## Design Doc
- **Architecture:** `InventoryEvent` triggers `SocialAgent`. `SocialAgent` generates content and schedules `SocialPost` via `PlatformIntegration`.
- **UX Flow:** User receives a notification "Drafted 3 posts for this week's new arrivals". User clicks "Approve All".
- **Mobile (375px):** Simple card swipe interface to approve/reject generated content.

## Implementation Prompt
Build a continuous background job that listens for inventory additions and generates engaging social media post drafts using an LLM. The user should be able to review and approve these drafts in a clean, swipeable mobile interface. Acceptance Criteria: The system must draft at least 3 posts automatically when 5 new items are added.
