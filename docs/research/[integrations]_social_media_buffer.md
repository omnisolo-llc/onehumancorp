# 🔍 Scout: Buffer (Social Media Integration)

## Title
Integrate Buffer for Omni-Channel Social Media Scheduling

## Problem Statement
Small business owners (like Maya the Baker or Priya the Boutique Owner) struggle to maintain a consistent presence across multiple social media platforms (Instagram, Facebook, TikTok, X). Manually logging into each app to post the same content is time-consuming and distracts them from running their business. They need a single place to create, schedule, and publish posts automatically.

## Research Report
**Buffer** is a leading social media management platform with a robust, well-documented API. It allows users to schedule posts, manage multiple accounts, and track engagement metrics.

**Pros for Non-Technical Users:**
- Simplifies cross-platform posting.
- Offers a visual calendar for content planning.
- Free tier available for basic usage (up to 3 channels), which is ideal for small businesses starting out.

**Integration Risks:**
- OAuth flow requires users to authorize Buffer to access their social accounts, which can be confusing if they haven't set up business accounts properly.
- Rate limits on the Buffer API need to be managed to prevent failed scheduled posts.
- Does not inherently solve the "unified inbox" for DMs and comments (only handles outbound posting and basic engagement tracking).

**Pricing:**
- Generous free tier (up to 3 channels, 10 scheduled posts per channel).
- Paid plans start at $6/month per channel, very affordable for SMBs.

**Environment Support:**
- Primarily Cloud-based. Standalone mode would require the user to provide their own Buffer API keys, which degrades the "zero config" experience.

## Design Doc
- **Integration:** The OHC user completes a standard OAuth flow to connect their OHC account to a Buffer account.
- **Data Flow:** OHC's "Marketing & Advertising" agent generates content (text, images) and uses the Buffer API to schedule these as posts across the user's connected social channels.
- **Action:** The AI Agent automatically drafts weekly social content based on new products or services added to the business profile, presents them to the user for approval within the OHC dashboard, and then sends them to Buffer for scheduling.

## Implementation Prompt
Implement an OAuth connection flow allowing OHC users to link their Buffer accounts. Create a UI component within the "Marketing" dashboard that allows the user to review, edit, and approve AI-generated social media posts. Upon approval, send the post data (text, media URLs, scheduled time) to the Buffer API for publishing. Ensure error handling is robust enough to inform the user if a post fails to schedule due to API limits or disconnected accounts.

## Priority
P1

## Estimated Scope
Medium
