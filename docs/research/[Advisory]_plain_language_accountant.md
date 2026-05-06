# 🔍 Scout: The Business Advisor (Plain Language Analytics)

## Title
The Business Advisor (Plain Language Analytics)

## Problem Statement
Small business owners like Carlos (Handyman) and Priya (Boutique Owner) suffer from "Financial Fog." They have access to complex analytics dashboards with charts and graphs, but they struggle to extract actionable insights. They need someone to tell them exactly what is happening in their business in plain English, not present them with raw data.

## Research Report
- **Strategy**: Daily, natural language business briefings.
- **Target Persona**: Carlos (Handyman), Priya (Boutique Owner)
- **Advantages**: Makes owners feel informed and in control without requiring data analysis skills.
- **Risks**: Providing inaccurate or overly generic advice that the user ignores.
- **Competitor Gap**: Competitors provide charts. OHC provides narratives.
- **Data**: 35% of users cite Financial Fog as a pain point.

## Design Doc
- **High-Level Architecture**:
  - A daily scheduled job triggers "The Advisor" agent.
  - The agent queries the database for yesterday's sales, traffic, and operational metrics.
  - The agent analyzes the data for trends or anomalies (e.g., "Sales are up 20% compared to last Tuesday").
  - The agent generates a 3-bullet-point summary in plain language.
  - The summary is displayed prominently on the mobile app home screen.
- **UI Flow**:
  - User opens the OHC app in the morning.
  - Top of screen shows a "Daily Briefing" card.
  - Content: "Yesterday was a great day! You made $450, mostly from Vegan Cupcakes. Your Instagram post drove 50 new visitors. Consider running a weekend promotion on standard cakes, as they have been slow."

## Implementation Prompt
Implement "The Advisor" reporting agent. Create a scheduled background task that aggregates daily business metrics (sales, views, orders). Pass these metrics to an LLM prompt designed to extract 2-3 key insights and format them as a friendly, encouraging, plain-language paragraph. Update the main dashboard UI to display this "Daily Briefing" card at the top of the screen.

## Priority
P2

## Estimated Scope
Medium
