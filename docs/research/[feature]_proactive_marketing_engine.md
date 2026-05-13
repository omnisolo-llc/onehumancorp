# [feature] Proactive AI Marketing Engine

**Title**: Implement Proactive AI Marketing Engine

**Problem Statement**:
Small business owners know they need to market their products, but they suffer from "blank page syndrome." They do not have the time or expertise to conceptualize, write, and schedule social media posts or promotional emails. Marketing is always the task pushed to tomorrow.

**Research Report**:
- 45% of SMBs report struggling with content creation.
- Stores that post on social media 3+ times a week see a 40% increase in traffic.
- Current solutions (Hootsuite, Buffer) require the user to create the content first. They are scheduling tools, not creation tools.

**Design Doc**:
- **Architecture**:
  - A cron job runs daily to evaluate the store's state (new products added, slow-moving inventory, upcoming holidays).
  - The Event Engine triggers an LLM prompt: "Generate 3 potential social media posts promoting [Product X] which hasn't sold in a week. Tone: Friendly, urgent."
  - Output includes suggested text, hashtags, and selects an existing product image.
  - The proposal is queued in the user's "Proactive Feed."
- **UI/UX Flow (Mobile 375px first)**:
  - User opens the OHC app.
  - An actionable card appears at the top: "✨ I noticed your Summer Hats are selling well! Should I post this to Instagram?"
  - The card displays a preview of the post.
  - User has two buttons: "Approve & Post" or "Edit."
  - Tapping "Approve" triggers the Meta Graph API integration to publish immediately.

**Implementation Prompt**:
Develop the Proactive Marketing Engine backend service. It should periodically scan merchant inventory and sales data to identify marketing opportunities. Integrate with an LLM to generate high-quality social media copy and image selections. Build a mobile-first UI component that presents these generated posts to the user for 1-tap approval, and wire the approval action to social media publishing APIs.

**Priority**: P1
**Estimated Scope**: Large
