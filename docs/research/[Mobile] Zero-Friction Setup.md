# [Mobile] Zero-Friction Setup

## Problem Statement
Small business owners like Maya (baker, 28) and Carlos (handyman, 42) run their businesses from their phones. Competitors like Shopify and Squarespace require a desktop for initial setup and meaningful customization. The setup process is confusing, technical, and discouraging.

## Research Report
- **Competitor Audit:** Shopify's mobile app is rated poorly for setup. Wix has improved, but still feels cramped. Durable generates a site quickly but lacks deep business management on mobile.
- **Pain Point:** "I tried setting up a store on my phone and gave up after 20 minutes." (Common sentiment in App Store reviews for legacy builders).
- **Opportunity:** OHC must offer a 100% mobile-native onboarding experience where a functional business is live in under 10 minutes.

## Design Doc
- **Architecture:** The mobile app must be the primary client, communicating with backend microservices via GraphQL/REST.
- **UX Flow (375px first):**
  1. Conversational AI intro: "What kind of business are you starting?"
  2. Voice/Text input capture.
  3. Processing screen with engaging animations (Glassmorphism, entrance < 300ms).
  4. Instant preview of the generated site, products, and services.
  5. One-tap "Go Live" button.
- **AI Integration:** The backend uses LLMs to interpret the user's input and generate the JSON structure representing the site design, product catalog, and initial settings.

## Implementation Prompt
**Outcome:** A user can download the OHC app, speak a single sentence describing their business, and have a fully live, transactional website generated within 30 seconds.
**Critical User Journey:** App Launch -> AI Chat -> Site Generation -> Live URL.
**Acceptance Criteria:**
- The entire flow must be completable on a 375px width screen without horizontal scrolling.
- AI generation must complete in under 30 seconds.
- The resulting site must be immediately accessible via a public URL.

## Priority
P0

## Estimated Scope
Large
