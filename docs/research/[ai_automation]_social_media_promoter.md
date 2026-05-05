# Issue Brief: The Generative Promoter (Auto-Social)

## Problem Statement
Small business owners struggle to maintain a consistent online presence. Creating content for social media is cited as the #1 reason stores go "dark" after 3 months ("Marketing Dread" - 55%). They don't have the time or skill to write compelling posts or design graphics regularly.

## Research Report
- **Pain Point Mapping**: "Marketing Dread" (55%).
- **Current Alternatives**: Using external tools like Buffer or manual raw ChatGPT, which breaks the "All-in-One" workflow and requires prompt engineering.
- **OHC Advantage**: Treat AI as a Teammate. The system automatically creates content based on internal business events (e.g., adding a new product).

## Design Doc
- **Event Trigger**: Listens for `ProductAdded` or `MilestoneReached` events on the mesh.
- **AI Agent**: "The Promoter" (Marketing Dept).
- **UI Flow (375px)**:
  1. User adds a new product: "Vegan Chocolate Cake".
  2. Agent generates a 7-day social media calendar (Posts, Images, Captions).
  3. Dashboard shows a notification: "Social posts ready for approval".
  4. User taps "Approve" -> Posts are scheduled via Meta Graph API integration.

## Implementation Prompt
Implement "The Generative Promoter" background listener that monitors the event mesh for product updates. When triggered, the agent must generate corresponding social media posts (caption + scheduling metadata) and queue them in the user's Action Feed for 1-tap approval. Ensure the generated content uses plain language and matches the business's vibe.

## Priority
P1

## Estimated Scope
Large
