**Title**: Autonomous Generative Engine Optimization (GEO) Agent

**Problem Statement**:
"Invisible Discovery" is the #4 SMB Pain Point (52% frequency). Non-technical owners like Fatima (Food Cart) or Leo (Music Tutor) do not understand SEO, schema markup, or how to get their business to show up when someone asks ChatGPT, "Where can I get good halal food near me?" Traditional SEO tools are too complex and jargon-heavy.

**Research Report**:
*   **Competitor Gap**: Shopify and Wix offer standard SEO tools (meta tags, sitemaps). Durable claims "AI visibility" but it requires manual setup. No competitor offers fully automated GEO tailored for LLM crawlers.
*   **Market Trend**: Search is shifting from traditional Google links to AI-generated answers (Google AI Overviews, ChatGPT, Perplexity).
*   **OHC Differentiation**: The "AI Discovery Agent" (Pillar 4 of the Manifesto). It runs in the background, automatically structuring data specifically for LLM consumption.
*   **Source Data**: *ohc_ai_differentiation_manifesto.md*, *smb_pain_points_top_10.md*.

**Design Doc**:
*   **Architecture (High Level)**:
    *   `Data Ingestion`: Extracts core business details (location, hours, menu/services, pricing) from the OHC database.
    *   `GEO Agent`: Automatically generates rich Schema.org JSON-LD markup. It dynamically updates this markup based on business changes (e.g., if Carlos adds a new service).
    *   `Distribution`: Automatically submits sitemaps and updated structured data to search engine APIs (Google Indexing API) and formats content to be easily readable by LLM crawlers (clear, factual, Q&A style hidden metadata).
*   **UI/UX Flow (Mobile 375px)**:
    1.  Zero configuration required by default.
    2.  Dashboard shows a simple "Discovery Health" metric (e.g., "Your business is optimized for AI search").
    3.  A weekly "Business Advisor" brief (Action Feed card) reports: "You appeared in 45 local AI searches this week."

**Implementation Prompt**:
Build an autonomous background agent that generates and maintains optimal structured data (JSON-LD) for a tenant's public storefront. The agent should monitor the tenant's data (products, services, hours) and automatically update the hidden metadata on their generated site to ensure maximum visibility in modern AI-driven search engines (GEO). The CUJ is entirely invisible to the user: they update their menu, and the agent instantly updates the site's schema without the user knowing what "schema" is.

**Priority**: P1
**Estimated Scope**: Medium
