# Proactive Autonomous Social Media Manager

## Problem Statement
Small business owners understand on an intellectual level that they must post consistently on social media to acquire new customers, but they critically lack the time, creative energy, and consistency required to maintain it. Furthermore, they operate on margins too thin to afford hiring a dedicated $1,000/month freelance social media manager.

## Research Report
When surveying users on review platforms like Trustpilot who churned from platforms like Squarespace, a dominant theme emerged: 'I built the beautiful website, but nobody came.' Existing social media management tools like Buffer or Hootsuite only solve half the problem; they still require the user to generate the creative content and manually schedule it. That remains too much friction. Users require an active system that notifies them, 'Hey, you haven't posted in 4 days. Here is a high-quality post announcing the new product you added yesterday. Should I publish this to Instagram now?'

## Design Doc
### Architecture Vision
- **Entities**: SocialPost, MarketingCampaign, AssetLibrary, PostTrigger.
- **UX Flow**:
  1. The system detects a trigger condition: it has been 3 days since the last published post, OR a new product was just added to the inventory database.
  2. The system autonomously generates a compelling image and a relevant, engaging caption.
  3. The system dispatches a push notification to the owner: 'A draft is ready for Instagram!'
  4. The user taps the notification, reviews the generated post within a simple modal window, and hits 'Approve'.
- **Mobile UX**: The home screen features a feed of pending 'suggested actions' or 'tasks', prioritizing these low-friction approval workflows.
- **Agent Integration**: A dedicated Marketing Agent continuously monitors internal business activity and external social trends to generate highly contextual, timely post drafts.

## Implementation Prompt
**Outcome**: Build an intelligent system that proactively generates and suggests social media posts to the user based on their active inventory, scheduling calendar, or general business context, requiring only a single tap to approve and instantly publish.
**Critical User Journey**:
1. The user adds a new product, for example, a 'Summer Dress', to their inventory.
2. The AI immediately detects this and generates an Instagram post announcing the dress's availability.
3. The user reviews the draft directly from their dashboard and clicks 'Publish Now'.
**Acceptance Criteria**: The system must initiate the interaction proactively based on logic triggers; it must not passively wait for the user to navigate to a 'Create New Post' screen.

## Priority
P1

## Estimated Scope
Medium
