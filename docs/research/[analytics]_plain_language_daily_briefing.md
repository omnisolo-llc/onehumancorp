# Plain-Language Daily Business Briefing

## Problem Statement
Traditional analytics dashboards (like Google Analytics or Shopify Analytics) are intimidating. Non-technical users don't know how to interpret line charts and bounce rates.

## Research Report
* **Finding:** Users ignore complex data. "Just tell me what to do" is a common sentiment.
* **Competitor Comparison:** No major platform provides daily plain-text insights; they all rely on visual dashboards.

## Design Doc
* **Architecture:** Scheduled cron job triggers an LLM to analyze the previous day's sales, traffic, and inventory data, generating a 3-sentence summary.
* **Mobile UX Flow:** Push notification at 8:00 AM: "Good morning! You had 3 sales yesterday ($150). Your 'Blue Shirt' is running low on stock. Tap to reorder."

## Implementation Prompt
**Critical User Journey:** Merchant receives a simple, actionable text summary of their business performance every morning instead of having to log in and interpret charts.
**Acceptance Criteria:**
* System aggregates daily metrics (sales, views).
* Agent generates a natural language summary.
* System delivers the summary via a notification or SMS simulation.

## Priority
P2

## Estimated Scope
Medium
