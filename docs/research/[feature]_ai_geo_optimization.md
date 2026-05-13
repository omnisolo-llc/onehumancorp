# Feature Mission: AI GEO (Generative Engine Optimization) for Local Discovery

## Problem Statement
Traditional SEO is a "black box" for founders like Carlos (handyman, 42). He knows he needs a website but doesn't know how to make it appear when someone asks ChatGPT, "Who is the most reliable handyman in Austin?" or "Find me a local plumber with 24/7 service."

## Research Report
- **Market Trend:** Users are shifting from Google Search to AI-powered discovery (ChatGPT, Perplexity, Gemini).
- **Competitor Audit:** Durable.co offers a basic "AI Visibility" report, but it's passive. Wix/Shopify are still focused on legacy keyword-based SEO.
- **Gap:** No platform currently *autonomously* optimizes the business's structured data (JSON-LD, OpenGraph, Schema.org) specifically for LLM ingestion in a way that requires zero user input.

## Design Doc
### High-Level Architecture
- **GEO Agent:** A background worker that periodically "audits" the business website as if it were an LLM crawler.
- **Optimization Swarm:** Automatically rewrites meta descriptions and structures "vibe-based" data (e.g., "Reliable", "24/7", "Local Favorite") into machine-readable schemas.
- **GEO Scorecard:** A simple "AI Discoverability" dial in the dashboard showing the business's presence in top LLM models.

### AI Agent Integration
- **The Promoter (Marketing):** Responsible for the GEO Agent. It publishes "Optimization Events" to the mesh.
- **The Advisor (Advisory):** Provides the user with a monthly "AI Discovery Report" in plain language.

## Implementation Prompt
Develop an autonomous "GEO Agent" for "The Promoter". This agent should scan the business's public profile and automatically generate and inject optimized Schema.org metadata tailored for LLM crawlers. Create a simple "AI Discoverability Score" component for the management UI that visualizes how well the business is indexed by ChatGPT, Gemini, and Claude.

## Priority
P0

## Estimated Scope
Large
