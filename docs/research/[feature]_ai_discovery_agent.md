# [feature] AI Discovery Agent (GEO)

## Title
The AI Discovery Agent (Generative Engine Optimization)

## Problem Statement
Small business owners often build websites but struggle to get traffic because traditional SEO is a "black box" and highly competitive. With the rise of AI search engines (ChatGPT, Gemini, Perplexity), a new optimization frontier (GEO) exists, but it is too technical for standard users ("Invisible Discovery").

## Research Report
*   **Gap:** "I built it, but nobody came." SEO is seen as a "black art" and traditional tools are complex.
*   **Differentiation:** The agent automatically optimizes structured data and content specifically for LLM crawlers, ensuring the business is recommended in AI-driven local queries.
*   **Outcome:** Automated high-intent traffic from AI search without manual keyword tuning.
*   **Evidence:** "Invisible Discovery" is ranked #4 in the Top 10 SMB Pain Points (52% frequency).

## Design Doc
*   **Entities:** SEOProfile, GEOMetrics, SearchQueryInsight.
*   **Key Relationships:** SEOProfile is linked to the Store. GEOMetrics tracks performance over time.
*   **UI/UX (Mobile-First 375px):**
    *   Dashboard includes a "Discovery Insights" card.
    *   Displays plain-language updates: "Your store was recommended 12 times by ChatGPT this week for 'vegan cakes near me'."
    *   1-tap approval for the agent to update business descriptions based on emerging search trends.
*   **AI Agent Integration:** A background agent continuously analyzes trending local queries and automatically updates the store's structured data (schema.org) and meta-descriptions to align with LLM indexing preferences.

## Implementation Prompt
Implement a discovery agent that proactively optimizes the store's public-facing data for Generative Engine Optimization (GEO). The agent should run periodically, analyze the store's offerings, and generate/update structured JSON-LD data and meta tags to improve visibility in AI search tools. Expose these updates to the user in the mobile dashboard via a simple insights feed. Focus on the automated data generation and the dashboard presentation.

## Priority
P2

## Estimated Scope
Medium
