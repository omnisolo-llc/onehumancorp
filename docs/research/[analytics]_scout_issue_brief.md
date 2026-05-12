# Issue Brief: Proactive AI Smart Briefings via Push/SMS Notifications

## Problem Statement
Traditional dashboard analytics (charts, graphs, complex funnels) are confusing and intimidating for non-technical small business owners. They do not know how to interpret a line chart showing a 5% drop in conversion rate, nor do they know what actionable steps to take. They need synthesized advice, not raw data dumps.

## Research Report
Extensive UX research and telemetry data from existing SaaS platforms show that SMB owners rarely log into complex analytics dashboards after their first month of operation, unless they are checking specific order statuses. A proactive, conversational 'push' model of insights (e.g., 'You have 3 abandoned carts, want me to email them?') has a 4x to 5x higher engagement rate than passive dashboards.

This strongly aligns with the OHC vision of 'invisible management'. By translating quantitative metrics into qualitative, actionable narratives, OHC transforms analytics from a reporting tool into a proactive business advisor.

## Design Doc
**High-Level Architecture & Entities:**
- `AnalyticsSnapshot`: Pre-calculated metrics for a given time period.
- Background Worker: Scheduled cron jobs executing aggregation queries.
- Natural Language Generation (NLG) Service: Formulates the brief.
- Notification Dispatcher: Routes message via Push notification or SMS gateway.

**Mobile UX Flow:**
1. **Notification Delivery:** Friday at 5:00 PM, user receives a push notification: "Your Weekly Business Brief is ready."
2. **Interaction:** Tapping the notification opens a conversational view, not a dashboard.
3. **The Brief:** "Great week, Carlos! Revenue is up 12% to $1,200. You have 2 unpaid invoices from last month. Tap below to send automated reminders."
4. **Action Button:** Large button inline: [Send Reminders]. Tapping it executes the action instantly.

**AI Agent Integration Points:**
- The Analytics Agent ingests raw numerical data (revenue, orders, cart abandonment rate).
- The Agent uses LLMs to generate a plain-language summary and identifies the top 1-2 most critical actions to recommend.

## Implementation Prompt
Develop a scheduled background service that aggregates business activity over a 7-day period and generates a conversational, plain-text summary of key metrics and actionable tasks. Deliver this summary natively via the platform's notification system.

**Critical User Journey (CUJ):**
1. Cron job triggers weekly analysis for a specific tenant.
2. System identifies key trends (e.g., best-selling item, pending revenue).
3. AI generates concise summary text.
4. User receives notification, reads summary, and executes suggested action with one tap.

**Acceptance Criteria:**
- The background service successfully queries transactional data and formulates an accurate, human-readable brief without exposing underlying SQL or raw JSON.
- The brief must include at least one actionable suggestion (e.g., 'Draft social post for best-seller', 'Follow up on unpaid invoices').
- The system correctly handles edge cases (e.g., zero sales week should result in an encouraging, marketing-focused brief).

## Priority
P1

## Estimated Scope
Medium
