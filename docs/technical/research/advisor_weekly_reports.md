# Issue Brief: "The Advisor" Weekly SMS Reports (P1)

## Problem Statement
Standard analytics dashboards are overwhelming for non-technical users. Showing a graph of "Conversion Rate vs. Bounce Rate" causes anxiety for a baker or a handyman. They don't want to analyze data; they want to know *what happened* and *what they should do next*.

## Research Report
*   **Target Persona:** All non-technical personas (Maya, Carlos, Priya, Leo, Fatima).
*   **Pain Point Validation:** Small business owners rarely log into complex analytics tools like Google Analytics because they don't know how to interpret the data.
*   **Competitor Analysis:** Shopify and Wix provide complex, multi-tab dashboards modeled after enterprise e-commerce platforms.
*   **Opportunity:** Replace the "Dashboard" with "The Advisor"—an AI that digests the weekly data and pushes a simple, human-readable summary directly to the user's phone.

## Design Doc
*   **High-Level Architecture:**
    *   Weekly cron job querying the metrics database (sales, traffic, bookings) per tenant.
    *   LLM prompt summarizing the raw data into 3-4 bullet points of actionable insight.
    *   Notification delivery system (Push/SMS).
*   **UI/UX Flow (Mobile-First):**
    *   Instead of navigating to an "Analytics" tab, the user receives a push notification on Sunday morning. Tapping it opens a simple, conversational summary screen. Example: "You had a great week! 12 bookings. Tuesday was your busiest day. Since 'Sink Repair' was your top service, consider offering a 10% discount on it next week to boost sales further."
*   **AI Integration:** "The Advisor" agent interprets quantitative data and translates it into qualitative advice.

## Implementation Prompt
Create a backend service that aggregates a tenant's weekly activity (orders, revenue, page views) and uses an LLM to generate a short, encouraging, plain-language summary. Implement the frontend view to display this summary in a conversational format (like a chat message from "The Advisor"), rather than using traditional charts and graphs. Ensure the language used is completely free of marketing or technical jargon.

*   **Priority:** P1
*   **Estimated Scope:** Medium
