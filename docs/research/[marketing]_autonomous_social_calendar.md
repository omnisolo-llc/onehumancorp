# [marketing] Autonomous Social Calendar

## Problem Statement
Small business owners know they need to post on social media to drive traffic, but they lack the time, design skills, and consistency to do so. They find tools like Buffer or Hootsuite too complex to set up.

## Research Report
Feedback from `r/Entrepreneur` and App Store reviews of Wix and GoDaddy highlight that while these platforms offer basic social post creation, they do not automate the scheduling process based on business events. Users want a system that "just does it for them."

## Design Doc
- **Architecture**: A new cron-triggered workflow within the `Marketing Promoter` agent. It monitors the `Product` catalog for new additions and the `Order` table for trending items.
- **Data Model**: `SocialPost` entity with `platform`, `scheduled_time`, `content`, and `status`.
- **UI/UX**:
  - A simple "Marketing Calendar" view on the mobile app.
  - A push notification: "Your new cupcakes are live! Tap to approve the 7-day Instagram promo campaign."

## Implementation Prompt
Implement the Autonomous Social Calendar. Create a background worker that generates a 7-day social media campaign (including drafted text and AI-generated image suggestions) whenever a user adds a new product to their catalog. Present this campaign in the OHC mobile app as a pending task. The user can review the posts and click "Approve All" to schedule them via the Meta Graph API.
The UI should be strictly mobile-first with 375px breakpoints.

## Priority
P1

## Estimated Scope
Medium
