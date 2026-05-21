# Title: Plain-Language Daily Business Briefing

## Problem Statement
Traditional analytics dashboards are confusing for non-technical users. They provide charts instead of actionable insights. Users don't have time to log in and interpret data.

## Research Report
*   **Gap identified:** Competitors offer complex dashboards (Shopify Analytics, Google Analytics integrations) which overwhelm users.
*   **Pain Point:** Understanding analytics is a major roadblock. Users want to know *what to do*, not just *what happened*.

## Design Doc
*   **High-level Architecture:** A daily cron job triggers an AI agent to query the sales, inventory, and marketing databases.
*   **UI Flow:** The user receives a simple SMS or push notification each morning. No login required to see the brief.
*   **AI Integration:** The agent synthesizes the raw data into a friendly, plain-language text (e.g., "Good morning! You had 3 orders yesterday. Stock is low on vanilla cupcakes.").

## Implementation Prompt
Implement a daily briefing system. A background worker should collect daily metrics (sales volume, low inventory alerts) and pass them to an LLM to generate a concise, friendly summary. The system should then deliver this summary via SMS or push notification to the business owner at a configured time each morning. The critical user journey is the owner waking up, checking their phone, and instantly knowing the state of their business without opening a dashboard app.

## Priority
P1

## Estimated Scope
Medium
