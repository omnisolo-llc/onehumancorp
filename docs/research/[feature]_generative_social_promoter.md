# Title: The Generative Promoter: Auto-Generated Social Media Calendars

## Problem Statement
"Marketing Dread" affects 55% of small business owners. Creating content for social media is the primary reason businesses stall. Owners do not have the time, skills, or budget to hire a social media manager.

## Research Report
Competitors offer basic SEO or allow users to use AI tools to *write* descriptions. None automatically *generate* a scheduled marketing plan based on core business events (like adding a new product).

## Design Doc
*   **Architecture Flow:**
    1.  Event: User publishes a new product or service.
    2.  Agent is triggered. It uses the product image and description as context.
    3.  Agent generates a 7-day social media campaign (e.g., Day 1: Announcement, Day 3: Behind the scenes, Day 7: Customer testimonial/Reminder).
    4.  Campaign is presented as a visual timeline.
*   **UI/UX:** A visual calendar view in the app. Each generated post has an image thumbnail and draft text. The user can swipe to approve or edit.
*   **AI Integration:** Uses multi-modal LLM capabilities (image generation/cropping + copywriting tailored to platforms like Instagram/Facebook).

## Implementation Prompt
Build an agent workflow that triggers upon the creation of a new product or service entity. The agent must use the provided product details and images to generate a sequence of 3-5 social media post drafts, scheduling them over a 7-day period. Surface these drafts in a dedicated marketing section of the Slint UI for user review and approval.

## Priority
P1

## Estimated Scope
Medium
