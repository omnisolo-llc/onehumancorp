### Title
[Feature] Plain-Language Daily Business Briefing

**Problem Statement:**
Founders suffer from "Financial Fog" (35% pain point frequency) and are overwhelmed by complex dashboards with raw metrics. They need actionable insights in human language, not charts.

**Research Report:**
- Competitors provide traditional analytics dashboards that require interpretation.
- OHC's "Business Advisor" persona should translate data into simple English.

**Design Doc:**
- **UI Flow:** A daily push notification leading to a single "Briefing" screen.
- **Content:** 3-4 bullet points (e.g., "You had 8 orders this week. Vegan cake requests doubled. Consider adding a vegan chocolate option!").

**Implementation Prompt:**
Create the UI and backend logic for a daily "Business Briefing". The backend should aggregate daily metrics and use the LLM provider to generate a short, plain-language summary. The frontend should display this summary prominently upon first login each day, tailored for a 375px mobile view.

**Priority:** P1
**Estimated Scope:** Medium
