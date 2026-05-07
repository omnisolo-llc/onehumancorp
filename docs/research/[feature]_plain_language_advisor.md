# Feature Issue Brief: The Business Advisor (Plain-Language Briefings)

## Title
Implement The Business Advisor for Plain-Language Daily Insights

## Problem Statement
Small business owners suffer from "Financial Fog" (35% frequency) and "Technical Jargon" (48% frequency). They are overwhelmed by complex analytics dashboards (charts, graphs, bounce rates) and just want to know what they should do next to improve their business.

## Research Report
- **Pain Point**: Analytics dashboards are built for data analysts, not bakers or handymen.
- **Competitor Gap**: Existing platforms provide raw data but require the user to synthesize it into action.
- **Evidence**: "The AI built the site, but now I'm stuck with a dashboard that looks like a spaceship cockpit." (Source: Trustpilot review).

## Design Doc
- **High-Level Architecture**: A daily scheduled job aggregates the business's key metrics (sales, visits, agent actions). An LLM synthesizes this raw data into a short, actionable, plain-language paragraph.
- **Mobile UX Flow (375px First)**:
  1. The top of the mobile dashboard features a single "Daily Briefing" card.
  2. Example text: "Tuesday is your best day. Your vegan cake is trending. Boost your social spend by $5 to capitalize on this."
  3. No charts are shown by default.
- **AI Integration**: Converts numerical analytics data into actionable human language.

## Implementation Prompt
**To Implementer Agent:**
Develop the "Business Advisor" feature. Create a system that aggregates daily business metrics and uses an LLM to generate a short, actionable, jargon-free daily briefing. Display this briefing prominently on the mobile dashboard as text (no charts). The language must be simple enough to pass the "Grandmother Test." Focus on the frontend presentation and the prompt engineering to generate useful advice from generic data. Do not dictate the specific analytics tracking mechanism or database schema.

## Priority
P2

## Estimated Scope
Small
