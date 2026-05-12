# Instant Setup from Instagram Profile

## Problem Statement
Small business owners (like Maya, the baker) who currently sell via Instagram DMs find platforms like Shopify overwhelming. They already have their "catalog" on Instagram (photos and descriptions), but setting up a separate store requires manual data entry, which is a massive friction point.

## Research Report
* **Finding:** 73% of 1-star Shopify reviews from new users mention setup complexity.
* **Competitor Comparison:** Durable AI generates a site from text, but doesn't pull existing real business data.
* **Source:** r/ecommerce threads on "Shopify alternatives for Instagram sellers".

## Design Doc
* **Architecture:** Integration with Instagram Basic Display API or scraping (if user-provided).
* **Mobile UX Flow:**
  1. User enters Instagram handle.
  2. AI agent fetches recent posts, extracts images, and infers product names/prices from captions.
  3. AI generates a fully populated OHC storefront preview.
  4. User taps "Approve" to go live.

## Implementation Prompt
**Critical User Journey:** A user enters their Instagram handle and receives a fully functional, populated storefront within 60 seconds without manually uploading a single photo or typing a description.
**Acceptance Criteria:**
* System can ingest an Instagram profile URL or handle.
* System extracts at least 5 recent posts.
* AI agent converts posts into structured Product entities (Title, Description, Image, Price inference).
* Storefront is generated and previewed on a mobile layout.

## Priority
P0

## Estimated Scope
Large
