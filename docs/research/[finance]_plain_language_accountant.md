# Issue Brief: Plain-Language Accountant Briefings

## Problem Statement
"Financial Fog" affects roughly 35% of SMB owners (Rank #9 in Top 10 SMB Pain Points). Most ecommerce analytics dashboards (like Shopify's default reporting) provide complex, overwhelming charts and graphs. Non-technical founders struggle to translate "Sessions by Traffic Source" or "Average Order Value (AOV) Trends" into actionable steps. They don't want a data export; they want an accountant to tell them if they are doing okay.

## Research Report
- **Competitor Landscape:**
  - **Shopify:** Excellent raw data and graphing, but overwhelming for beginners. "Reports" often require a higher-tier subscription.
  - **Wix / Squarespace:** Basic line charts showing traffic and sales, but no prescriptive advice.
- **User Needs:** A food cart operator (Fatima persona) or freelance handyman (Carlos persona) doesn't have time to interpret a line chart. They need a quick, readable summary telling them what worked and what didn't.
- **The Leapfrog Opportunity:** Translate complex Prometheus metrics and database aggregations into a 3-bullet point narrative delivered by "The Accountant" and "The Advisor" agents.

## Design Doc
### High-Level Architecture
- **Data Aggregation Worker:** A daily/weekly batch job aggregates Stripe payment data, order volume, and traffic stats.
- **Agent Analysis Pipeline:** Pass the aggregated JSON data to the LLM via "The Accountant" agent with a strict prompt: "Translate this JSON financial data into a friendly, plain-language summary suitable for a non-technical small business owner. Do not use jargon like 'conversion rate'."
- **Output Artifacts:** A short string or rich-text summary stored in the `weekly_briefings` table.
- **UI Presentation:** Displayed as a conversational chat bubble or an "Action Card" on the main Dashboard UI, designed for 375px screens. No charts—just text and an optional simple trend indicator (e.g., a green arrow).

### Implementation Prompt
Implement the "Plain-Language Financial Briefing" module within the "Finance & Payments" department. Create a scheduled worker that gathers the previous week's sales, refund, and top-product data. Feed this to the AI Agent to generate a 3-sentence plain-English summary (e.g., "Great week! You sold 15 more vegan cakes than last week, but refund requests slightly increased on chocolate."). Display this summary on the Flutter/Slint dashboard as a weekly "Advisor Insight."

## Priority
P2

## Estimated Scope
Medium
