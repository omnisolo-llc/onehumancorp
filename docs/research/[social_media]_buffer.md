# Scout: Tool Integration Research

## Social Media Integration
**Title**: Integrate Buffer for Automated Social Media Scheduling
**Problem Statement**: Small business owners like bakers and handymen struggle to maintain a consistent presence across Facebook, X, and Instagram. They often forget to post while busy with their actual work, missing out on organic growth.
**Research Report**:
- Buffer is an established social media management platform that simplifies scheduling posts across multiple networks (Facebook, Instagram, LinkedIn, X, TikTok, etc.).
- Its API is developer-friendly, and its interface is known for being extremely intuitive for non-technical users.
- Pricing: Offers a very generous free tier (up to 3 channels), making it highly accessible for our base users. Paid tiers are affordable.
- Compatibility: Works well in Cloud mode via OAuth. For Standalone mode, users can supply their own Buffer API credentials.
- Compared to Hootsuite, Buffer is much easier for true SMBs.
**Design Doc**:
- The "Marketing & Advertising" tab gains a "Social Media" section.
- The user authenticates with Buffer via a simple OAuth flow.
- "The Promoter" AI agent automatically generates a weekly calendar of suggested posts (images and text) based on the business's recent activity (e.g., new products, positive reviews).
- The user reviews the queue and clicks "Approve All," and the agent schedules them through Buffer's unified API.
**Implementation Prompt**: Connect Buffer via OAuth so users can link their social accounts. Allow the AI to draft and queue multi-platform posts into Buffer based on the user's business updates. Provide a simple UI for the user to review and approve the scheduled posts.
**Priority**: P1
**Estimated Scope**: Medium