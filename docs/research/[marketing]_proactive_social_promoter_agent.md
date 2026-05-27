# Issue Brief: Proactive Social Promoter Agent

## Title
[Marketing] Proactive Social Promoter Agent

## Problem Statement
Marketing Paralysis: The "Blank Page" problem extends beyond initial setup. Small business owners often forget or lack the time to promote new products or services. Adding a new item to their catalog is a purely administrative task, and the critical step of marketing that new item is missed. Consistency is the biggest hurdle for SMB marketing.

## Research Report
- **Competitor Landscape**: Platforms like Shopify and Wix require the user to manually craft social media posts or use separate, reactive AI tools to generate copy after the fact.
- **User Needs**: Users need marketing to happen automatically as a byproduct of their regular business operations (like adding a new cake to the menu).
- **AI Differentiation**: OHC shifts from reactive assistance to autonomous, background infrastructure. The Proactive Social Promoter Agent acts as a dedicated marketing employee, instantly leveraging new business events into promotional content.

## Design Doc
### High-Level Architecture
- **Trigger**: The Backend Event Mesh detects a `ProductAdded` or `ServiceAdded` event within the catalog.
- **Agent Action**: The Proactive Social Promoter Agent (The Promoter) is invoked.
  - Pulls the new item's details (name, description, price, image).
  - Drafts multiple engaging social media posts (e.g., for Instagram, Facebook) tailored to the business's predefined "vibe".
  - Optionally, generates composite promotional images using the product photo.
- **UI Flow**: The drafts are pushed to the user's mobile dashboard Activity Feed, requesting a simple 1-tap approval.

### Mobile UX Flow (375px First)
1. **Activity Feed**: "You added 'Vegan Chocolate Cake'. We drafted 3 Instagram posts for it."
2. **Review Screen**: User views the suggested images and captions in a native-feeling, mobile-optimized card layout.
3. **Action**: User taps "Approve" to publish immediately or schedule, or "Edit" to make quick tweaks.

## Implementation Prompt
Implement the "Proactive Social Promoter Agent" feature. Create an event listener that triggers The Promoter whenever a user adds a new product or service. The agent should automatically generate corresponding social media drafts (images and captions) and surface them in the user's dashboard for 1-tap approval and publishing.

## Priority
P1

## Estimated Scope
Medium
