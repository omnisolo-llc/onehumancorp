# [feature] AI Discovery Agent (Generative Engine Optimization)

## Title
Implement Proactive AI Discovery Agent for Generative Engine Optimization (GEO)

## Problem Statement
Small business owners (like Carlos the Handyman or Maya the Baker) consistently fail at traditional SEO. They do not understand technical jargon like "meta tags," "schema markup," or "sitemaps," resulting in their businesses being invisible online ("I built it, but nobody came"). As search shifts from traditional links to AI-driven answers (ChatGPT, Gemini, Perplexity), SMBs are entirely unequipped to ensure their business is recommended by these LLMs.

## Research Report
- **Pain Point Mapping:** Directly addresses "Invisible Discovery" (Ranked #4, affecting 52% of surveyed SMBs).
- **Competitive Landscape:** Shopify and Wix rely on legacy SEO tools requiring manual input. Durable offers basic "AI visibility" but lacks ongoing optimization.
- **Strategic Opportunity:** OHC can leapfrog competitors by ignoring legacy SEO and focusing natively on GEO (Generative Engine Optimization). By automatically structuring business data for LLM consumption, OHC ensures its merchants are the default recommendations in AI queries (e.g., "Recommend a good local baker in Austin").

## Design Doc
### High-Level Architecture
- **Entity & Trigger:** The Discovery Agent is triggered whenever core business information changes (e.g., new product added, business hours updated, location changed).
- **Core Action:** The agent automatically compiles a comprehensive, LLM-friendly profile of the business. It injects structured schema data (JSON-LD) into the storefront implicitly without user intervention.
- **UI/UX Flow:**
  - The feature is completely invisible during setup.
  - The user receives a proactive notification in their Dashboard Feed: *"Your business profile has been optimized for AI search engines."*
  - A "Visibility" tab simply shows: "Status: Optimized for AI" with plain-language metrics (e.g., "Your business was recommended 12 times this week").

## Implementation Prompt
Implement the AI Discovery Agent as a background service within the OHC ecosystem. The agent should listen for relevant business updates and automatically generate and inject the necessary structured data formats to optimize the merchant's storefront for LLM crawlers. Ensure the user-facing output is purely informational and zero-configuration, providing a simple, plain-language summary of their visibility status in the mobile dashboard.

## Priority
P1

## Estimated Scope
Medium
