# Plain-Language Daily Business Briefing

## Problem Statement
Founders are intimidated by complex analytics dashboards filled with technical jargon and line charts. They experience "Financial Fog" and are starving for actionable insights but overwhelmed by data. They simply want to know "What happened yesterday?" and "What should I do today?" in plain English.

## Research Report
*   **Technical Jargon (48% frequency):** Users feel alienated by terms like Conversion Rate, CNAME, and Bounce Rate.
*   **Financial Fog (35% frequency):** Many owners resort to exporting data to spreadsheets to figure out their actual profitability.
*   **Competitor Gap:** Squarespace and Shopify present dense dashboards designed for desktop monitors. OHC must simplify this into a narrative "Morning Brief" that acts as a business advisor.

## Design Doc
*   **Architecture:** A nightly cron job aggregates core metrics (sales, traffic, inventory levels, appointment bookings). An LLM synthesizes this raw data into a concise, 3-sentence summary highlighting trends and providing one actionable recommendation.
*   **UI Flow:** A "Morning Brief" card pinned to the top of the mobile dashboard (optimized for 375px screens). It uses a friendly, conversational tone (e.g., "Good morning Maya! Tuesday is your best day. Your vegan cake is trending. Consider boosting your social spend by $5.").
*   **AI Integration:** Scheduled LLM summarization pipeline.

## Implementation Prompt
Build the scheduled backend job to aggregate daily business metrics and the corresponding Slint UI component to display the Daily Briefing. Ensure the generated brief uses plain language, the Inter font for body text, and avoids raw metric charts in favor of narrative text. The UI component must implement OHC premium Glassmorphism tokens (backdrop-filter: blur(20px)).

## Priority
P1

## Estimated Scope
Medium
