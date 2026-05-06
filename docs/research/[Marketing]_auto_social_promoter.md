# 🔍 Scout: The Generative Promoter (Auto-Social Marketing)

## Title
The Generative Promoter (Auto-Social Marketing)

## Problem Statement
Small business owners like Priya (Boutique Owner) and Maya (The Home Baker) suffer from "Marketing Dread." Creating content for social media is the #1 reason stores go "dark" after 3 months. They lack the time and design skills to consistently post updates across platforms, leading to lost momentum and invisible discovery. They need a system that automatically generates high-quality social media content and schedules it for them, without requiring complex marketing tools.

## Research Report
- **Strategy**: Proactive, autonomous social media content generation and scheduling.
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Removes the marketing barrier for non-technical users. Ensures consistent brand presence. Leverages the "AI Teammate" philosophy over the "AI Tool" philosophy.
- **Risks**: Ensuring generated content matches the brand's unique "vibe" and voice. Avoiding generic-sounding posts.
- **Competitor Gap**: Shopify and Wix rely on third-party apps or reactive AI tools that still require user prompting and manual scheduling. OHC can leapfrog by making this autonomous and event-driven.
- **Data**: 55% of users report Marketing Dread as a top pain point.

## Design Doc
- **High-Level Architecture**:
  - A background agent ("The Promoter") listens for "New Product Added" or "Restock" events on the event mesh.
  - The agent accesses the business's brand profile (vibe, tone, key selling points).
  - The agent generates a 7-day social media calendar (images + captions) tailored to the specific event.
  - The generated content is queued in the user's Dashboard Action Feed for 1-tap approval.
- **UI Flow**:
  - User adds a new product via the mobile app.
  - The next morning, the Daily Briefing includes: "I've drafted 3 Instagram posts for your new Vegan Cake. Tap here to review and schedule."
  - User taps, sees the drafted images and captions in a simple feed.
  - User taps "Approve All" to schedule the posts.
- **AI Integration**: The Promoter Agent handles the generation and scheduling autonomously, only requiring user approval.

## Implementation Prompt
Implement "The Promoter" marketing agent. The agent should subscribe to product-related events (e.g., `ProductCreated`, `InventoryRestocked`). Upon receiving an event, it should generate a series of draft social media posts (text and image prompts/assets) based on the business's configured brand voice. These drafts should be stored and presented to the user in a simple "Action Required" feed in the UI, allowing for 1-tap approval to schedule them for publishing.

## Priority
P1

## Estimated Scope
Medium
