# Generative Engine Optimization (GEO) Core

## Problem Statement
"I built the store, but no one is coming." Non-technical business owners view traditional SEO as a confusing "black art." They don't know how to optimize metadata, leading to "Invisible Discovery" and a lack of organic traffic.

## Research Report
*   **Invisible Discovery (52% frequency):** Creating a site is easy; getting traffic is hard. Standard SEO tools in Wix/Shopify are too complex for beginners.
*   **AI Search Shift:** Traditional Google Search is being supplemented by LLM crawlers (ChatGPT, Gemini). Optimizing for these engines (GEO) requires rich, structured data rather than just keyword stuffing.
*   **Competitor Gap:** Current platforms rely on legacy SEO plugins. OHC can automatically handle GEO, making AI discovery a built-in advantage.

## Design Doc
*   **Architecture:** A background agent automatically runs whenever a product or service is updated. It translates standard product data into rich, structured JSON-LD format specifically optimized for AI crawlers and semantic search.
*   **UI Flow:** Radically simple. A single toggle switch in the settings UI: "Enable AI Search Visibility (On/Off)" with a brief, plain-language explanation of its benefit.
*   **AI Integration:** LLM-assisted generation of rich metadata, descriptions, and semantic tags.

## Implementation Prompt
Create the background agent logic that automatically translates standard product/service data into rich, LLM-optimized metadata (JSON-LD) upon any update. Implement the simple toggle in the settings UI. Ensure the process is entirely invisible to the user beyond the single toggle switch.

## Priority
P1

## Estimated Scope
Medium
