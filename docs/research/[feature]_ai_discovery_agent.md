# Feature Issue Brief: AI Discovery Agent (Generative Engine Optimization)

## Title
Implement the AI Discovery Agent for Automated GEO (Generative Engine Optimization)

## Problem Statement
Small businesses face "Invisible Discovery" (52% frequency). Traditional SEO is confusing, feels like a "black art," and is becoming obsolete as consumers shift to asking AI tools (ChatGPT, Perplexity) for recommendations instead of searching Google.

## Research Report
- **Pain Point**: Invisible Discovery. Users build sites but get no traffic because they don't understand metadata, sitemaps, or structured data.
- **Competitor Gap**: Shopify and Wix rely on legacy SEO tools (editing meta tags manually). None are optimizing for LLM crawlers automatically.
- **Evidence**: "I built it, but nobody came." Users need an automated way to be discoverable without learning technical SEO. (Source: SMB Pain Point Audit).

## Design Doc
- **High-Level Architecture**: A background agent continuously scans the business's public storefront, products, and services. It automatically generates and injects advanced structured data (JSON-LD) and natural language business summaries designed specifically for LLM ingestion.
- **Mobile UX Flow (375px First)**:
  1. The feature runs invisibly.
  2. The mobile dashboard periodically shows a "Discovery Health" card: "Your store is optimized for AI search."
  3. No manual configuration is required; an "Advanced Mode" toggle can show the raw injected data for curious users, but defaults to "Simple Mode".
- **AI Integration**: The agent translates product descriptions into optimized formats preferred by AI models.

## Implementation Prompt
**To Implementer Agent:**
Build the "AI Discovery Agent." Implement a background process that automatically generates LLM-optimized structured data and business summaries for a user's storefront based on their existing catalog. Ensure this data is injected into the public-facing site without any manual user intervention. Provide a simple status indicator on the mobile dashboard confirming the store is "AI-Search Ready." Follow the Progressive Disclosure Pattern: hide technical details by default but allow viewing via an "Advanced Mode" toggle. Do not prescribe the specific implementation of the background job runner or database tables.

## Priority
P1

## Estimated Scope
Medium
