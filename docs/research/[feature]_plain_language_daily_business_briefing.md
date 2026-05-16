### Title
[Feature] Plain-Language Daily Business Briefing

**Problem Statement:**
Founders suffer from "Financial Fog" (35% pain point frequency) and are overwhelmed by complex dashboards with raw metrics. They need actionable insights in human language, not charts.

**Research Report:**
- Competitors provide traditional analytics dashboards that require interpretation.
- OHC's "Business Advisor" persona should translate data into simple English.

**Design Doc:**
- **High-Level Architecture:**
    - **Entity Types:** `DailyBriefing`, `BusinessMetric`, `InsightSummary`.
    - **Key Relationships:** `DailyBriefing` aggregates multiple `BusinessMetric` snapshots; `InsightSummary` is an LLM-generated interpretation of those metrics.
    - **Integration Points:** Payment Provider API (Stripe/PayPal), Store Analytics, LLM Interpretation Service.
- **Mobile UX Flow (375px First):**
    1. **Trigger:** Daily morning push notification: "Good morning Maya! Your Tuesday briefing is ready."
    2. **View:** Single "Briefing" screen with 3-4 bullet points in large, readable font.
    3. **Insight:** "Your vegan cupcakes are trending—you sold 15 yesterday. Consider baking an extra batch for today!"
- **AI Agent Integration Points:** Advisor Agent runs a daily cron job to pull metrics and generate the `InsightSummary`.

**Implementation Prompt:**
Create the UI and backend logic for a daily "Business Briefing". The backend should aggregate daily metrics and use the LLM provider to generate a short, plain-language summary. The frontend should display this summary prominently upon first login each day, tailored for a 375px mobile view.

**Priority:** P1
**Estimated Scope:** Medium
