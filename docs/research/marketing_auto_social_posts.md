# Autonomous Marketing Agent

## Problem Statement
Small business owners like Leo (music tutor) and Maya (baker) struggle with marketing consistency. They know they need to post on social media and send emails to drive sales, but they lack the time, copywriting skills, and design expertise. Marketing feels like a separate full-time job. When they don't post, their sales drop.

## Research Report
**Findings:**
*   "I don't know what to post" and "I don't have time to post" are the top two reasons SMBs fail at social media marketing.
*   Businesses that post consistently see a 40% increase in customer engagement and retention.
*   **Competitor Comparison:**
    *   **Shopify:** Requires third-party apps for automated social posting. Email marketing tools exist but require manual template creation and copywriting.
    *   **Wix:** Has built-in email marketing and social post generators, but still relies heavily on the user to initiate and schedule the content.
    *   **Buffer/Hootsuite:** Great for scheduling, but the user still has to create the content.
*   **Opportunity:** OHC can leapfrog competitors by shifting from "marketing tools" to a true "Marketing Agent" that autonomously generates, schedules, and (with approval) publishes content based on the store's inventory and activity.

## Design Doc
**Architecture / Entities:**
*   `MarketingCampaign`: A container for a sequence of generated posts/emails.
*   `ContentDraft`: The actual generated content (text, image prompt, target channel).
*   `ApprovalQueue`: The staging area where drafts wait for user approval.

**Mobile UX Flow (375px first):**
1.  **Weekly Review:** User receives a push notification on Sunday: "Your marketing plan for the week is ready."
2.  **Dashboard:** User opens the app to see a proposed schedule: 3 Instagram posts, 1 promotional email.
3.  **Draft Review:** User taps on an Instagram post draft. It shows a generated image (or placeholder for their own photo) and a drafted caption based on a new product or a timely theme (e.g., "Back to school lessons").
4.  **Action:** User taps "Approve All" to schedule the week, or "Edit" to tweak a caption.

**AI Agent Integration Points:**
*   **Strategy Agent:** Analyzes business data (new products, slow-selling items, upcoming holidays) to determine *what* to market.
*   **Creative Agent:** Generates the copy and suggests visual themes or image generation prompts.
*   **Scheduling Agent:** Optimizes posting times based on industry best practices.

## Implementation Prompt
Create an Autonomous Marketing Agent that generates a weekly marketing plan for the business owner. The agent should look at the business profile and any simulated product data to generate drafts for social media posts and emails. The user must be able to review, edit, and approve these drafts in a simple feed view.

**Critical User Journey:**
1. User receives a notification that their weekly marketing plan is ready.
2. User opens the "Marketing" tab in the app.
3. User sees 3 proposed Instagram posts for the week, complete with AI-generated captions highlighting specific products or services.
4. User taps "Approve" on the first two.
5. User taps "Edit" on the third, changes a sentence, and then taps "Approve."
6. The system schedules these posts for automatic publishing.

**Acceptance Criteria:**
*   The system can generate text content that is contextually relevant to the business's industry and simulated products.
*   The UI provides a clear, swipeable or list-based queue for reviewing pending drafts.
*   Approved drafts transition to a "Scheduled" state.

## Priority
P1

## Estimated Scope
Large