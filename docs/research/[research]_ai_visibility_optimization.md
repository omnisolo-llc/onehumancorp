# Issue Brief: AI Visibility & GEO (Generative Engine Optimization)

## Problem Statement
Traditional SEO (Search Engine Optimization) is becoming secondary to GEO (Generative Engine Optimization)—how a business appears in AI search results like ChatGPT, Gemini, and Perplexity. Small business owners have no idea how to optimize for this "AI-first" discovery layer.

## Research Report
- **Durable.co Advantage:** Offers a "Weekly AI visibility ranking" to show if ChatGPT is recommending the business.
- **Market Trend:** Users are increasingly using LLMs to ask "What's the best bakery near me?" or "Who can fix my sink in Austin?"
- **Opportunity:** OHC can provide a built-in "AI Discovery Agent" that ensures the business metadata is structured perfectly for LLM crawlers and generative search.

| Strategy | Traditional SEO | OHC GEO Agent |
| :--- | :--- | :--- |
| **Focus** | Keywords & Backlinks | Vibe, Clarity & Schema |
| **Target** | Google Search Bot | LLM Crawlers (GPT-5, Gemini) |
| **Owner Effort** | High (Manual) | Zero (Background) |

## Design Doc
### High-Level Architecture
- **Discovery Agent:** Periodically scans the business's public profile and cross-references it against generative search "best practices" (Structured data, schema.org, plain-language clarity).
- **Visibility Report:** A simple "Generative Score" (0-100) displayed in the Analytics section of the Tauri app.
- **Auto-Optimization:** Agent suggests or auto-applies content changes to improve the business's "vibe" for AI models.

### Implementation Prompt
Create a "Generative Visibility" tool for "The Promoter" (Marketing). This tool should analyze the business website content and provide a report on how likely it is to be cited by LLMs. Include specific actionable steps for the owner to improve their "AI searchability."

## Priority
P1

## Estimated Scope
Medium
