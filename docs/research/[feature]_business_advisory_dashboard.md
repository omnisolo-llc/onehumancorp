# [Product] Business Advisory Dashboard

## Problem Statement
Most non-technical business owners find traditional analytics dashboards (line charts, conversion funnels, bounce rates) confusing and unactionable. They don't want raw data; they want plain-language advice on what to do next to grow their business.

## Research Report
- **Competitor Landscape:** Shopify provides robust analytics but requires data literacy to interpret. GoDaddy provides basic traffic numbers but no actionable insights.
- **Pain Point Data:** "I see I have 100 visitors, but I don't know what to do with that information" is a common sentiment in r/smallbusiness.
- **Opportunity:** OHC's Business Advisory Agent ("The Advisor") can translate raw Prometheus/Grafana metrics into simple, weekly English statements and suggested actions.

## Design Doc
- **Core Entity:** `AdvisoryInsight` (Type: Success, Warning, Opportunity; text description; suggested action link).
- **UI Flow (Mobile-First 375px):**
  1.  **Dashboard Home:** Instead of a giant chart, the top widget is a friendly greeting from the Advisor Agent: "Good morning Maya! You had a great week. Vegan cakes are trending."
  2.  **Insight Cards:** Scrollable cards detailing specific insights:
      - *Insight:* "Tuesday was your busiest day."
      - *Insight:* "3 people abandoned their cart at the deposit stage. [Send Discount Code]"
  3.  **Weekly Summary:** A plain-text block summarizing the week's financial health.
- **AI Integration:** The Advisor Agent ingests weekly metrics from the internal observability stack and generates a personalized summary and actionable tasks.

## Implementation Prompt
Design and implement a "Business Advisory" Slint component for the main dashboard. Replace traditional complex charts with a clean, mobile-first feed of "Insight Cards." Each card should have an icon (success, warning, idea), a plain-language sentence (e.g., "Your top seller this week was Lemonade."), and an optional call-to-action button (e.g., "Run a Promotion"). The layout must be perfectly usable on a 375px screen.

## Priority
P1

## Estimated Scope
Small
