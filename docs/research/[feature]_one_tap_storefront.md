# Zero-UI Storefront Generation

## Title
One-Tap AI Storefront Generator

## Problem Statement
The primary barrier to entry for non-technical users (like Carlos the handyman or Fatima the food cart owner) is the initial setup. Shopify and Wix require users to choose themes, map domains, set up navigation menus, and configure payment gateways before they can make a single sale. This process takes days and causes massive drop-off.

## Research Report
- **Sources:** YouTube ("how to start shopify store" top comments), GoDaddy App reviews, Wix churn data proxies.
- **Data:** "Setup Overwhelm" is the #1 pain point.
- **Competitive Gap:** Wix ADI asks questions but still drops the user into a complex drag-and-drop editor. Durable is fast but shallow.
- **Finding:** Users don't want to *build* a website. They want a website *built for them*.

## Design Doc
### Architecture
- **Ingestion Engine:** Accept unstructured text (e.g., "I'm Carlos, I fix sinks in Austin") or an uploaded menu image.
- **Generation Pipeline:** Use LLMs to determine business type, generate appropriate copy, select a pre-defined semantic theme (Glassmorphism UI), and populate initial catalog items.
- **Provisioning:** Automatically spin up the required OHC resources (database tenant, routing rules, default Stripe connect setup).

### Mobile UX Flow (375px)
1. **Chat Onboarding:** A friendly conversational interface asks 3 questions: "What do you sell?", "Where are you located?", "Upload a photo of your work".
2. **Loading State:** "Agent is building your business..." (progress bar).
3. **The Reveal:** A fully functional, mobile-optimized storefront is presented.
4. **Action:** A prominent "Accept First Payment" button is immediately available.

## Implementation Prompt
**User-Facing Outcome:** A user downloads the OHC app, types a sentence about their business, and within 60 seconds is handed a live URL with a fully functional storefront, booking system, or catalog, completely bypassing any drag-and-drop website builders or complex configuration dashboards.

**Acceptance Criteria:**
- Input a single string of plain text describing a business.
- Output a live, functional storefront URL.
- The generated storefront must include at least 2 AI-generated products/services based on the prompt.
- Must be fully mobile-responsive natively.

## Priority
P0

## Estimated Scope
Medium
