### Title
[Feature] AI Discovery Agent (GEO - Generative Engine Optimization)

**Problem Statement:**
"I built it, but nobody came." Small business owners (like Carlos the Handyman) find traditional SEO confusing and expensive. With the rise of AI search (ChatGPT, Gemini, Perplexity), being discoverable by LLM crawlers is more important than ranking for keywords on Google.

**Research Report:**
- Durable (2024) is the only competitor explicitly marketing "ChatGPT Discoverability."
- 52% of SMB owners see SEO as a "black art" (Pain Point #4).
- OHC can leapfrog by automating the creation of AI-readable structured data (JSON-LD, Schema.org) and proactive indexing.

**Design Doc:**
- **High-Level Architecture:**
    - **Entity Types:** `DiscoveryManifest`, `CrawlerStatus`, `BusinessProfile`.
    - **Key Relationships:** `BusinessProfile` has many `DiscoveryManifest` versions; `DiscoveryManifest` tracks multiple `CrawlerStatus` entries (e.g., OpenAI, Google, Bing).
    - **Integration Points:** Google Business Profile API, Search Console Indexing API, OHC Event Mesh.
- **Mobile UX Flow (375px First):**
    1. **Overview:** "Discovery" card on Home Dashboard showing "AI Visibility: Active".
    2. **Details:** Tap card to see "Where you appear" (list: ChatGPT, Gemini, etc.) with last-indexed timestamps.
    3. **Action:** "Boost Visibility" button to trigger a fresh scan and manifest generation.
- **AI Agent Integration Points:** Discovery Agent watches `SiteUpdated` events to regenerate JSON-LD and pings the Indexing worker.

**Implementation Prompt:**
Implement the "AI Discovery Agent." The agent should automatically generate and maintain high-fidelity Schema.org markup for the business (NAP - Name, Address, Phone, services, pricing). Create a backend worker to submit sitemaps to major AI search engines. Add a "Discovery" section to the mobile dashboard that uses plain language to report on visibility status.

**Priority:** P1
**Estimated Scope:** Medium
