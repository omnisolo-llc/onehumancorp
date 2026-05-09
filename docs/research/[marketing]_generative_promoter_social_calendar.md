### Title
[marketing] The Generative Promoter: Automated 7-Day Social Media Calendar

### Problem Statement
"Marketing Dread" is a major barrier for small business owners, ranking as the #3 pain point (55% frequency). Creating consistent content for social media is cited as the primary reason digital storefronts go "dark" after three months. Solopreneurs lack the time, design skills, and copywriting expertise to maintain a strong social media presence, which is crucial for modern discovery.

### Research Report
- **Competitor Gap:** Competitors like GoDaddy (Airo) offer basic AI branding, and others offer isolated social post generation, but none provide a fully automated, event-driven, multi-day campaign generator.
- **Pain Points Addressed:** Marketing Dread, Setup Complexity.
- **Validation:** 55% of users report marketing content creation as a dread-inducing task. Automating this removes the biggest barrier to ongoing customer engagement.

### Design Doc
- **Architecture:**
  - Event Trigger: User adds a new product or service to their OHC store.
  - Generative Promoter Agent: Activated by the `product.added` event via the KAIROS Orchestrator.
  - Content Generation: Agent generates a comprehensive 7-day social media campaign, including diverse post types (announcement, behind-the-scenes, customer highlight), optimized copy, and suggested imagery (leveraging AI image generation or product photos).
  - Storage: The generated campaign is stored in the database, linked to the user's account and the specific product.
- **UI Flow (375px First):**
  - Upon adding a product, a notification appears: "The Promoter created a 7-day launch campaign for [Product Name]."
  - The user taps to view a calendar or list view of the 7 drafted posts.
  - Each day's post shows the image, copy, and scheduled platform.
  - The user can "Approve All", or edit individual posts before approval.

### Implementation Prompt
Implement "The Generative Promoter" feature. Develop the backend logic to listen for new product additions and trigger the AI agent to generate a 7-day social media content calendar (text and image prompts/suggestions). Build the mobile-first UI to present this calendar to the user for review. The UI must clearly display the scheduled date, content, and imagery for each post, with intuitive controls to edit or approve the entire campaign with minimal friction.

### Priority
P1

### Estimated Scope
Medium