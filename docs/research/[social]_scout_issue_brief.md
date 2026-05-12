
# Title: Autonomous AI Social Media Marketer

## Problem Statement
Founders (like Maya the baker) know they need to post on social media to grow, but lack the time and copywriting skills. They need a system that automatically generates and schedules engaging posts based on their product catalog.

## Research Report
- Creating multi-channel social posts is cited as a top bottleneck by 7% of founders in marketing forums.
- Consistency is key, yet most founders abandon social media after 2 weeks.
- Native integration allows OHC to leverage product photos and descriptions automatically.

```mermaid
graph LR
    A[New Product Added] --> B[AI Analyzes Photo & Text]
    B --> C[Drafts 3 Social Posts]
    C --> D[Owner Approves with 1 Tap]
    D --> E[Posts scheduled to IG/FB]
```

## Design Doc
- **High-level architecture**: Integration with Meta Graph API, an AI content generation pipeline, and a scheduling cron job service.
- **UI wireframes or screen flow description (375px first)**:
    - **Marketing Tab**: Displays a carousel of AI-generated social posts ("Suggested Posts").
    - **Approval Flow**: Swiping right approves and schedules a post; swiping left dismisses it.
- **Mobile UX flow**: Tinder-like interface for approving marketing copy. Zero typing required unless editing.
- **AI Integration**: Vision model to analyze product photos; text model to generate platform-specific copy (e.g., short for IG, conversational for FB).

## Implementation Prompt
Implement the AI Social Media Marketer feature. The Critical User Journey involves the system detecting a new product addition, generating suggested posts, and the user approving one via a swipe interface, resulting in a scheduled post. Acceptance criteria: Fully usable at 375px width, AI generates relevant copy based on product data.

## Priority
P1

## Estimated Scope
Medium
