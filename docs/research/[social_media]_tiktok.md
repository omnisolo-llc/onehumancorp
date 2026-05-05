# TikTok Integration for OHC

## Problem Statement
Small business owners (like Leo the Music Tutor or Priya the Boutique Owner) rely heavily on TikTok to showcase their products, services, and creative portfolios. However, managing TikTok interactions—such as replying to comments and tracking link-in-bio clicks—requires switching between apps. For non-technical users, bridging the gap between TikTok engagement and their central business inbox is disjointed and time-consuming. They need a way to auto-reply to TikTok comments, sync link-in-bio traffic, and manage engagement seamlessly from their OHC dashboard.

## Research Report
- **Features & API Suitability**: TikTok provides a Marketing API and Login Kit. Through the Display API, platforms can fetch user profiles and videos. The Comment API allows reading and replying to comments on videos. Webhooks can be set up for events.
- **Pricing**: API access is generally free, subject to rate limits.
- **Ease of Use for Non-Technical Users**: Very high once authenticated via OAuth. The user simply connects their TikTok account and the agent handles the rest.
- **Cloud vs. Standalone**: Works well in Cloud. For Standalone, OAuth callback handling might require a cloud-hosted relay or strict local configuration.
- **Advantages**: Direct access to Gen Z and younger millennial audiences; high engagement.
- **Risks**: API rate limits, strict data privacy requirements, and the API can sometimes be opaque or subject to sudden changes.

## Design Doc
- **Integration Point**: "The Promoter" (Marketing & Advertising) and "The Ambassador" (Customer Success).
- **Trigger**: User connects their TikTok account via OAuth in the Marketing settings.
- **Action**: OHC sets up webhooks or periodic polling to fetch new comments. The Customer Success agent drafts replies based on the user's settings and posts them. Analytics are synced to the dashboard.
- **User View**: A unified inbox showing TikTok comments alongside Instagram DMs and emails, with auto-drafted responses ready for approval or auto-sending.

## Implementation Prompt
Implement a TikTok integration that allows users to authenticate their TikTok business/creator account via OAuth. The system must listen for new comments on their videos and display them in the OHC unified inbox. "The Ambassador" AI agent must be able to auto-draft replies to these comments based on the business's context (e.g., answering "do you sell this in red?"). Ensure the user can view basic engagement analytics (views, likes, comments) on their OHC dashboard.

## Priority
P1

## Estimated Scope
Medium
