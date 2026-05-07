# [feature] The Business Advisor

## Title
The Business Advisor (Plain-Language Insights)

## Problem Statement
Small business owners are overwhelmed by complex dashboards with graphs and charts they don't understand ("Financial Fog"). They want to know what to do next, not just look at data.

## Research Report
*   **Gap:** Founders are overwhelmed by data but starving for insights. Existing tools require exporting to spreadsheets to understand profit vs. revenue.
*   **Differentiation:** No complex charts. A daily "Human-Language Briefing" that provides actionable advice based on store data.
*   **Outcome:** Clear, actionable strategic direction for non-technical users.
*   **Evidence:** "Financial Fog" is ranked #9 in the Top 10 SMB Pain Points (35% frequency).

## Design Doc
*   **Entities:** DailyBriefing, MetricInsight, RecommendedAction.
*   **Key Relationships:** DailyBriefing contains multiple MetricInsights and RecommendedActions.
*   **UI/UX (Mobile-First 375px):**
    *   Dashboard features a daily summary card at the top.
    *   Content is purely text-based and conversational: "Good morning! Tuesday is typically your best day. Your vegan cake is trending. Consider boosting your social spend by $5 today to capture more sales."
    *   Inline buttons within the text allow 1-tap execution of recommendations (e.g., "Boost Spend").
*   **AI Agent Integration:** A background agent analyzes sales, traffic, and inventory data overnight. It uses an LLM to synthesize this data into a conversational, plain-language briefing with specific, actionable recommendations.

## Implementation Prompt
Implement a daily briefing agent that synthesizes store performance data into a plain-language summary. The agent should run on a schedule, analyze recent metrics, and generate a conversational briefing text that highlights trends and suggests actions. Surface this briefing prominently on the mobile dashboard. Avoid complex graphing libraries; focus on the data synthesis and text generation.

## Priority
P2

## Estimated Scope
Small
