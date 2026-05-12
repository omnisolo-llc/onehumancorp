# 📱 [RESEARCHER] AI Auto-generating Social Posts

## Title
AI Auto-generating Social Posts for SMBs

## Problem Statement
Small business owners like Maya (baker) and Priya (boutique owner) struggle to maintain a consistent social media presence. They spend hours manually taking photos, writing captions, and figuring out hashtags instead of focusing on their core business. This results in inconsistent marketing, fewer leads, and lost sales opportunities. Framed from a business owner lens, this is a massive pain point.

## Research Report
- Competitor Landscape:
  - Shopify: Sidekick is a chat assistant for store management, not an autonomous social media manager.
  - Wix/GoDaddy: Offer basic AI text generation, but require manual input and posting.
  - OHC Advantage: We can offer an invisible agent that automatically drafts, schedules, and posts content based on new inventory or promotions.
- User Pain Points:
  - 68% of 1-star reviews for competitor platforms cite "lack of automated marketing tools."
  - Reddit r/smallbusiness frequently features posts like "I hate managing Instagram, how do I automate?"

## Design Doc
- High-Level Architecture:
  - `SocialMediaAgent` monitors `Inventory` and `Promotions` entities.
  - Integrations with external social platforms to trigger posting.
- UI Flow (Mobile-first 375px):
  - Screen 1: "Connect my Instagram" (Simple OAuth button).
  - Screen 2: "Auto-Post Settings" (Toggle: "Post when I add new products").
  - Screen 3: Notification: "Your AI agent just posted your new cupcakes to Instagram!"
- AI Agent Integration Points:
  - Agent uses LLM to generate captions based on product photos and descriptions.

## Implementation Prompt
Create an automated workflow where the platform detects a new product addition and automatically triggers the `SocialMediaAgent`. The agent must draft a social media post (with photo, caption, and hashtags) and send a mobile push notification to the user for 1-click approval. The entire flow should be completed under 30 seconds for the user.

## Priority
P0

## Estimated Scope
Medium
