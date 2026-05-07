# Feature Issue Brief: The Generative Promoter (Auto-Social Marketing)

## Title
Implement The Generative Promoter for Automated Social Media Campaigns

## Problem Statement
Small business owners like Priya (boutique owner) struggle with "Marketing Dread" (55% frequency). Creating consistent, high-quality social media content is the #1 reason new stores go "dark" after a few months. They don't have the time or design skills to promote their products.

## Research Report
- **Pain Point**: Marketing Dread is a critical barrier to sustained business success.
- **Competitor Gap**: Wix ADI helps build the initial site but offers little ongoing generative marketing support. Durable builds sites quickly but lacks robust post-launch marketing tools.
- **Evidence**: Small business owners abandon their stores due to the immense effort required to maintain a social presence. (Source: SMB Pain Point Audit).

## Design Doc
- **High-Level Architecture**: Upon the event of a new product being added to the catalog, the agent automatically triggers a workflow to generate a 7-day social media content calendar (captions, suggested posting times, and image prompts/generations based on the product photo).
- **Mobile UX Flow (375px First)**:
  1. User adds a new product via the mobile app.
  2. A notification appears: "Your 7-day social plan for [Product] is ready!"
  3. The user views a Tinder-style swipe interface or simple list of the 7 posts.
  4. 1-tap "Approve All" schedules the posts; or the user can edit individual posts.
- **AI Integration**: Triggers on product creation; uses LLM for copywriting and scheduling logic.

## Implementation Prompt
**To Implementer Agent:**
Develop the "Generative Promoter" automation. When a user creates a new product, automatically generate a 7-day social media post schedule (including text and image suggestions). Display this schedule in the mobile dashboard using a simple, intuitive approval interface. Focus on a seamless, jargon-free experience that empowers the user to schedule marketing with minimal taps. Ensure all UI elements are mobile-friendly (≥44x44px touch targets). Do not detail the underlying database schema or API endpoints.

## Priority
P1

## Estimated Scope
Large
