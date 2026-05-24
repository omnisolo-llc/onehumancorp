# [marketing] Autonomous Social Marketer

## Problem Statement
Boutique owners (like Priya) know they need to post on social media to drive sales, but suffer from "marketing paralysis." They don't know what to write, lack the time to design posts, and treat social media as an afterthought rather than a consistent acquisition channel.

## Research Report
Analysis of solopreneur pain points reveals that content creation is often outsourced to expensive agencies or ignored entirely. Current platforms provide tools to schedule posts (if the user creates them) or basic AI text generation, but they lack *proactive* context. They don't know when a new product arrives and automatically suggest a campaign.

## Design Doc
**Architecture & Key Relationships:**
*   **Event Listener:** Monitors the OHC business engine for triggers (e.g., "New Inventory Added", "Empty Booking Slots Tomorrow").
*   **Creative Agent:** Generates image variations and marketing copy tailored to the specific trigger and the business's brand voice.
*   **Approval Queue:** A low-friction mobile UI where the user reviews the AI's drafts.
*   **Publishing Engine:** Schedules and posts to connected social channels (Instagram, Facebook).

**UX Flow:**
1.  Priya adds a new summer dress to her inventory.
2.  The Event Listener triggers the Creative Agent.
3.  Priya receives a push notification: "Drafted 3 Instagram posts for your new Summer Dress."
4.  Priya opens the app, reviews the images and captions, and taps "Approve All."
5.  The Publishing Engine schedules the posts for optimal times throughout the week.

## Implementation Prompt
Develop the Autonomous Social Marketer feature. The system should proactively generate marketing content based on business events (like adding inventory or having open calendar slots). The Critical User Journey involves the user receiving an AI-generated post draft and approving it with a single tap. Acceptance criteria: The feature must successfully detect a state change in the business (e.g., new product) and generate a draft post with an image and caption ready for user approval.

## Priority
P1

## Estimated Scope
Medium
