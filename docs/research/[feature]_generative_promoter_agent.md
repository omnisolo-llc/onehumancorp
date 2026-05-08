# Feature: The Generative Promoter (Marketing Agent)

## Title
The Generative Promoter (Marketing Agent)

## Problem Statement
Small business owners, like Priya (boutique owner) and Carlos (handyman), struggle with "Marketing Dread" (identified in 55% of analyzed pain points). Creating consistent, high-quality content for social media and email campaigns is a major barrier. Traditional platforms (Shopify, Wix) treat marketing tools as manual workflows or basic templates, requiring the owner to act as a copywriter, designer, and social media manager. This often leads to "ghost town" stores that fail to drive discovery and retention.

## Research Report
- **Goal:** Transform marketing from a manual, time-consuming chore into an invisible, autonomous operation.
- **Pain Points Addressed:** Marketing Dread, Invisible Discovery.
- **Differentiation:** Instead of an AI "writing assistant" (like Shopify Sidekick), the Generative Promoter is a proactive teammate. It monitors the business state and automatically generates full campaigns without requiring a prompt.
- **Competitor Landscape:**
  - *Shopify:* App store reliant, high cost creep for marketing tools. Sidekick provides chat-based advice but doesn't autonomously execute.
  - *Wix:* Basic built-in email marketing and SEO, but still requires manual drafting and scheduling.
  - *Durable:* Fast setup, but weak ongoing marketing operations.
- **Strategy:** Leverage the OHC event mesh to trigger campaign generation based on business events (e.g., adding a new product, hitting a sales milestone, or local holidays).

## Design Doc
- **High-Level Architecture:**
  - **Triggers:** Listens to `ProductAdded`, `InventoryRestocked`, `SeasonalEvent`, and `CustomerMilestone` events.
  - **Generation Engine:** Utilizes LLMs to generate text copy (social media posts, email newsletters) and AI image generation models to create polished visuals based on product images or descriptions.
  - **Action Feed:** Generated campaigns are pushed to the merchant's "Action Required" feed in the dashboard.
  - **Approval Flow:** The merchant reviews the generated campaign with a simple 1-tap "Approve & Schedule" or "Reject/Regenerate" interaction.
  - **Distribution:** Once approved, the agent automatically posts to connected social channels (via Meta Graph API, etc.) and schedules email blasts (via native SendGrid/SES integration).
- **Mobile UX Flow (375px first):**
  1. Push notification: "I drafted a 7-day social campaign for your new 'Summer Dress'. Review?"
  2. User taps notification, opening the OHC app.
  3. App displays a swipeable carousel of the 7 posts (image + caption).
  4. Bottom sticky button: "Approve All & Schedule".
- **AI Agent Integration Points:**
  - Needs access to the unified `Product` catalog for details and imagery.
  - Interacts with the `BrandVoice` vector database to ensure generated copy matches the owner's tone.

## Implementation Prompt
Implement the Generative Promoter agent in the `src/agents/builtin/` directory. The agent should subscribe to business events (e.g., a new product being added) and automatically generate a complete, multi-channel marketing campaign (social posts, email draft). The generated campaign must be surfaced in the user's dashboard for 1-tap approval. Do not prescribe specific database schemas or API contracts; focus on the event-driven generation and the user approval workflow.

## Priority
P1

## Estimated Scope
Large
