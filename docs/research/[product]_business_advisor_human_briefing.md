# Issue Brief: Autonomous "Business Advisor" for Plain-Language Insights

## Problem Statement
Small business owners often suffer from "Financial Fog." They struggle to understand the actual health of their business, differentiating between revenue and profit, and identifying actionable trends. Traditional analytics dashboards rely on complex charts and graphs that require analytical skills to interpret, leaving many founders overwhelmed and uninformed.

## Research Report
- **SMB Pain Points:** "Financial Fog" affects an estimated 35% of users, who often have to export data to spreadsheets to understand their real standing.
- **Competitor Gap:** Existing platforms provide traditional analytics dashboards. Users must actively seek out insights by interpreting data themselves.
- **OHC Opportunity:** OHC can provide a "Business Advisor" agent that analyzes financial and operational data in the background and delivers actionable, plain-language insights directly to the user, acting like a personal consultant.

## Design Doc
### High-Level Architecture
- **Data Aggregation:** A scheduled job (e.g., weekly or daily) aggregates data across sales, expenses, and customer engagement.
- **Insight Generation:** The "Business Advisor" agent analyzes this aggregated data against historical trends and business goals to extract 2-3 key insights.
- **Briefing Delivery:** The agent formats these insights into a concise, human-readable briefing (e.g., "Your top seller was lemonade. Tuesday was your busiest day. Consider running a promotion on Wednesday to boost mid-week sales.") and delivers it via the UI and potentially via email/push notification.

### Mobile UX Flow (375px First)
- **Home Dashboard:** A dedicated "Advisor Briefing" card at the top of the dashboard, featuring a friendly greeting and a brief summary statement.
- **Detail View:** Tapping the card reveals the full, bulleted, plain-language report. There are no complex graphs, only clear text and actionable recommendations.
- **Action Links:** Recommendations (e.g., "Run a promotion") include deep links to the relevant setup screens.

## Implementation Prompt
Implement the "Business Advisor" background worker. Create a scheduled task that queries the reporting database, synthesizes recent performance data, and uses the LLM provider to generate a short, actionable, plain-language business health report. Develop the Flutter UI to display this report prominently on the home dashboard. The design must adhere to the Glassmorphism visual style and be fully functional on a 375px screen without requiring horizontal scrolling or complex chart rendering.

## Priority
P1

## Estimated Scope
Medium
