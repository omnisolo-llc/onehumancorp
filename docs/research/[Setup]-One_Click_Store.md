# [Setup] One-Click Store Generator

## Title
AI-Powered "One-Click" Store Generation from Social Profiles

## Problem Statement
The onboarding process for traditional platforms (Shopify, Wix) requires dozens of steps, manual data entry, and design choices. Non-technical users find this overwhelming and abandon the setup. They need a way to go from zero to a live, populated store instantly.

## Research Report
- **Frequency:** 32% of 1-star reviews for legacy builders mention setup complexity.
- **Competitor Gap:** Wix ADI asks questions to build a theme, but requires manual product entry. Durable generates sites quickly but lacks commerce depth.
- **Market Data:** Reducing Time-To-Live (TTL) directly correlates with higher trial-to-paid conversion rates.

## Design Doc
- **Core Entity:** `StoreGeneratorAgent`.
- **Integration Points:** Public URL scraping (Instagram, Yelp, existing legacy site).
- **UX Flow:**
  - Onboarding screen: "Where do you sell right now? Paste your Instagram or Yelp link."
  - Loading screen (under 30 seconds): "Our AI is reading your profile, fetching your photos, and building your store..."
  - Result: A fully styled storefront with products pre-populated from scraped images.

## Implementation Prompt
Implement a flow where a new user provides a social media URL, and an agent autonomously generates a populated store catalog and assigns a brand-aligned theme.
- The CUJ starts at signup and ends with the user seeing a live preview of their new store populated with their own images and generated product names.
- Must eliminate technical steps like "DNS setup" or "CMS configuration" from the initial flow.

## Priority
P1

## Estimated Scope
Medium