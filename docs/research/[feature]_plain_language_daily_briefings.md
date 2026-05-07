# [feature] The Business Advisor: Plain Language Daily Briefings

**Priority:** P1
**Estimated Scope:** Medium

## Problem Statement
Small business owners (like Carlos, a 42-year-old handyman) are overwhelmed by complex dashboards with charts, graphs, and technical jargon (e.g., "conversion rate," "bounce rate"). They need to know how their business is doing and what actions to take, but they don't have the time or expertise to interpret raw data.

## Research Report
Our audit of competitor platforms (Shopify, Wix) reveals that dashboards are primarily designed for desktop use and assume a certain level of data literacy. "Financial Fog" is a significant pain point (35% frequency). Users want actionable insights, not just data. They want to know what to do next to grow their business.

## Design Doc
*   **High-level architecture:**
    *   **Trigger:** Daily scheduled job (e.g., 8:00 AM local time).
    *   **Agent (The Business Advisor):** An autonomous agent aggregates data from sales, traffic, inventory, and recent customer interactions.
    *   **Generation:** The agent uses an LLM to synthesize this data into a short, plain-language briefing (3-4 sentences maximum).
    *   **Delivery:** The briefing is delivered as a push notification and displayed prominently on the OHC mobile app home screen.
*   **Mobile UX Flow (375px first):**
    *   User receives a morning notification: "Your Daily Business Briefing is ready."
    *   User opens the OHC app.
    *   Top of the screen shows a friendly, conversational message: "Good morning, Carlos! Yesterday was your best Tuesday this month. You have 3 pending quotes to follow up on. Your 'Basic Plumbing' service is trending locally."
    *   Tapping the briefing provides quick links to take action (e.g., a button to view the pending quotes).
*   **AI Agent Integration:** The agent needs read access to aggregate business metrics and the ability to formulate concise, non-technical summaries.

## Implementation Prompt
Implement "The Business Advisor" feature to generate and deliver plain-language daily briefings. The system should analyze daily business data and present a short, jargon-free summary to the user each morning on their mobile device. The Critical User Journey involves the user opening the app and immediately understanding their business status and top priorities without needing to decipher any charts or graphs.
