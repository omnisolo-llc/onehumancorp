# [Insights] Plain-Language Weekly Digest

## Title
Proactive Plain-Language Business Insights via SMS

## Problem Statement
Complex analytics dashboards (like Google Analytics or even Shopify's reporting) confuse non-technical owners. They don't want to parse line charts; they just want to know how the business is doing and what they should do next.

## Research Report
- **Frequency:** 18% of SMBs struggle to understand their store analytics.
- **Competitor Gap:** Standard dashboards require the user to log in and interpret data manually.
- **Goal:** Shift from "pull" analytics (user logs in) to "push" insights (system tells the user).

## Design Doc
- **Core Entity:** `InsightAgent`.
- **Integration Points:** Orders Database, Traffic Analytics, SMS Gateway.
- **UX Flow:**
  - Every Friday at 5 PM, the owner receives an SMS.
  - "Hi Maya! Great week—you had 15 orders ($450 total). Your Chocolate Chip Cookies are selling twice as fast as usual. Should we order more supplies?"

## Implementation Prompt
Build a cron-triggered agent that analyzes a tenant's weekly performance metrics, synthesizes them into a brief, human-readable summary, and delivers actionable recommendations via push notification or SMS.
- The CUJ involves the generation and delivery of this summary message based on mock weekly data.
- Must adhere strictly to the "Grandmother Test" (no technical jargon).

## Priority
P3

## Estimated Scope
Small