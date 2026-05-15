## 1. Social Media Integration: Buffer

**Title:** Integrate Buffer for Unified Social Media Publishing and Analytics
**Problem Statement:** Small business owners spend too much time logging into individual platforms (Instagram, Facebook, TikTok) to post updates and respond to comments. They need a single, unified inbox and scheduling tool that simplifies their workflow.
**Research Report:**
- **Tool evaluated:** Buffer
- **What problem it solves for which persona:** Solves the fragmented social media presence problem for small business owners like Fatima, who want to manage all social media updates in one place.
- **Ease of Use:** Buffer is known for its highly intuitive, non-technical interface.
- **Pricing:** Rough pricing is $6/month per social channel, with a free tier available for up to 3 channels.
- **Reputation:** Excellent reputation among SMBs for reliability and simplicity.
- **Advantages & Risks:**
  - *Advantages:* Very simple to use, clear pricing, integrates with all major networks.
  - *Risks:* API rate limits could be an issue if scaled rapidly, less advanced automation than competitors.
- **Cloud/Standalone Mode:** Works perfectly in Cloud mode via OAuth. For Standalone mode, users would need to supply their own API keys or rely on a proxy service, which is a bit complex but doable.
**Design Doc:**
- **Trigger:** The user connects their Buffer account in the OHC Settings page.
- **Action:** OHC pushes scheduled posts to Buffer and pulls comments/messages into the OHC Unified Inbox.
- **User View:** The business owner sees a 'Social Media' dashboard inside OHC where they can draft posts and view aggregated engagement metrics.
**Implementation Prompt:**
Create a Social Media module in the OHC frontend. Allow the user to authenticate with a third-party social media tool. Provide a unified view to draft posts (with text and image uploads) and a feed of recent comments across platforms. Success is defined by the user being able to publish a post to at least two networks simultaneously from the OHC dashboard.
**Priority:** P1
**Estimated Scope:** Medium
