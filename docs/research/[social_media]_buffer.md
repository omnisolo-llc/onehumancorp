# Buffer Integration Issue Brief

## Title
Integrate Buffer for Simplified Social Media Posting

## Problem Statement
Small business owners like Maria the Florist don't have the time to manually log into Instagram, Facebook, and Twitter to post updates about their daily specials. They need a simple, unified way to schedule and publish posts without leaving their primary business management tool.

## Research Report
- Buffer provides a very intuitive and clean API for publishing content across major social networks (Instagram, Facebook, X, LinkedIn).
- It is highly regarded for its ease of use and focus on simplicity, making it perfect for non-technical users.
- Pricing: Very accessible free tier, with affordable paid plans for growing businesses.
- Competitors: Hootsuite is more complex and enterprise-focused; Ayrshare is good but Buffer has better brand recognition among small businesses.
- Integration: OHC users could leverage Buffer's infrastructure to handle the complex platform-specific API quirks.
- Cloud/Standalone: Works perfectly in Cloud mode. Standalone mode might require users to create their own Buffer account and provide API keys, or use a shared integration proxy.

## Design Doc
- Users link their Buffer account via a simple OAuth flow in the "Marketing" dashboard.
- The user interface provides a simple text box and image upload for drafting posts.
- The "Promoter" AI agent can suggest post content or scheduling times based on the business's activity.
- The posts are sent to Buffer's API, which handles the actual scheduling and delivery to the target platforms.

## Implementation Prompt
Implement a Buffer integration where users can authenticate their Buffer account via OAuth. Create a UI component that allows users to draft a post with text and an image, and send it to Buffer for scheduling. The feature should support checking the status of scheduled posts.

## Priority
P2

## Estimated Scope
Medium
