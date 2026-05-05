### Title
[Feature] AI Generative Promoter Campaigns

**Problem Statement:**
Small business owners, such as Priya the boutique owner or Maya the baker, struggle with "Social Media Paralysis." They know they need to advertise their products, but they lack the design skills, copywriting ability, and time to consistently create engaging posts for Instagram, Facebook, or TikTok. Current tools require manual prompt engineering, which is intimidating and time-consuming.

**Research Report:**
- Based on our analysis, "Social Media Paralysis" is the #2 biggest pain point for SMBs (38% frequency).
- Existing solutions (like Canva or ChatGPT) operate as disconnected, reactive tools. They require the user to initiate the action and switch context between their store and the marketing tool.
- OHC's differentiation lies in treating AI as a proactive teammate ("The Generative Promoter" department).

**Design Doc:**
- **Trigger:** The creation of a new Product, Service, or a significant business event (like an upcoming holiday sale).
- **Agent Action:** The "Marketing & Advertising Agent" automatically detects the event via the Hybrid Event Mesh. It generates a 7-day social media campaign, including images (using image generation or existing product photos) and tailored captions with relevant hashtags.
- **UI Flow (375px First):**
    - The generated campaign appears as a pending task card in the Home Dashboard's "Agent Activity Feed".
    - The user taps the card to review the proposed 7-day schedule.
    - The user can tap "Approve All" to schedule the posts, or edit individual days.
- **Integration:** Hooks into the existing scheduling system and the pending social media integrations (e.g., Meta Graph API).

**Implementation Prompt:**
Implement the backend logic for the "Generative Promoter" agent. When a new product is created in the database, emit an event that triggers the Marketing AI agent. The agent must use the LLM to generate a 3-part social media campaign (e.g., Teaser, Launch, Follow-up) including captions and image prompts. Store these as pending `AgentAction` records in the database. Update the mobile activity feed UI to display these pending actions, allowing the user to review and 1-tap approve the campaign for publishing. Ensure all UI elements are optimized for a 375px mobile view.

**Priority:** P0

**Estimated Scope:** Large
