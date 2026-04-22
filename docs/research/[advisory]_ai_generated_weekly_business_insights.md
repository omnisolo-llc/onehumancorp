# [advisory] AI-Generated Weekly Business Insights

## Title
Weekly Plain-Language Business Health Reports via AI

## Problem Statement
Analytics dashboards (like Google Analytics or even Shopify's built-in stats) are often too dense for non-technical users. They see charts but don't know what action to take.

## Research Report
*   **Competitor Analysis**: Wix and Shopify provide charts and graphs but leave interpretation to the user.
*   **User Need**: A weekly "text message" style report from the Business Advisor agent that says things like "Your top seller was lemonade. Tuesday was your busiest day. Consider a Tuesday promotion."

## Design Doc
*   **Architecture**:
    *   Weekly batch job that aggregates tenant telemetry, order volume, and revenue data.
    *   Data is fed into an LLM with a strict prompt to produce concise, actionable, plain-language insights.
    *   Delivery via Push Notification and In-App Inbox.
*   **UI Wireframes**:
    *   "Advisor" tab in the app showing a chat-like interface with the weekly reports.

## Implementation Prompt
Implement the data aggregation and LLM generation pipeline for the weekly business insight report. Create a cron job that runs weekly, gathers sales and traffic data per tenant, and prompts the LLM to generate a short, actionable summary. Expose this summary in the Flutter app's dashboard.

## Priority
P2

## Estimated Scope
Small
