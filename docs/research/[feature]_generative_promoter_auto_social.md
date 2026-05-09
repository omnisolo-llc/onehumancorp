# Feature Brief: The Generative Promoter (Auto-Social)

## Problem Statement
Creating marketing content is the primary reason small stores fail to gain traction ("Marketing Dread" is the #3 pain point). Founders are not copywriters or social media managers.

## Research Report
Wix and Shopify require users to manually create campaigns or use basic templates. We need an agent that autonomously handles the creative burden whenever a new product is launched.

## Design Doc
**Architecture & Integration:**
- **Entity Types:** `Product`, `SocialCampaign`, `SocialPost`
- **Integration Points:** Product creation webhook/event, LLM (for copy), Image Gen (for visuals).

**UX/UI Flow (Mobile-First 375px):**
1.  **Trigger:** User adds a new product to their store.
2.  **Notification:** "The Promoter has generated your 7-day launch campaign."
3.  **Review Screen:** A carousel of 3-5 social media posts (images + captions) ready to be scheduled or posted immediately.

## Implementation Prompt
Implement "The Generative Promoter". When a user creates a new product, an event should trigger an AI agent to generate a small campaign (e.g., 3 social media posts with varied copy) related to that product. These posts should be presented to the user for review. Acceptance criteria: Product creation triggers the generation process; the UI displays the generated content; the user can view the proposed copy. Do not prescribe the specific LLM or queue implementation.

## Priority
P1

## Estimated Scope
Medium
