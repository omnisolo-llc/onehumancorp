# [mobile_setup] AI-Guided Mobile Onboarding

## Problem Statement
Non-technical small business owners (like Maya the baker or Fatima the food cart owner) find legacy platforms like Shopify too complex to set up. They require a desktop computer, multiple hours of configuration, and an understanding of web design concepts. These users need to be able to launch a fully functional store from their smartphone in under 10 minutes, using only natural language and photos.

## Research Report
Market research indicates that "Complex Setup on Mobile" is the #1 pain point for SMBs, appearing in 82% of negative reviews for legacy platforms. While tools like Wix ADI exist, they still require significant manual tweaking. OHC can leapfrog this by utilizing our autonomous agent architecture to completely generate the store based on a conversational UI.
*Sources: Competitor gap analysis and Trustpilot reviews (Shopify, Wix).*

## Design Doc
- **High-Level Architecture**:
  - A mobile-first UI wizard (Slint-based) that asks 3-5 simple questions (Business Name, What do you sell, Upload a photo).
  - An `Agent` (e.g., Nova or a specialized onboarding agent) processes the natural language input and images.
  - The Agent automatically populates the `CatalogItem` and `Tenant` records via the MCP bridge.
- **Mobile UX Flow (375px first)**:
  - Screen 1: "What kind of business are you building?" (Text/Voice input)
  - Screen 2: "Upload a photo of your product/service."
  - Screen 3: "Generating your store..." (Progress animation, <300ms transitions, Glassmorphism).
  - Screen 4: Store is live. Link generated.
- **AI Agent Integration Points**: The onboarding flow directly calls the LLM service to generate descriptions, categorize items, and set initial pricing based on the uploaded photo and text.

## Implementation Prompt
Implement a mobile-first AI onboarding wizard for the OHC platform. The Critical User Journey (CUJ) is: A user opens the app, is greeted by an AI agent, answers a few questions about their business via chat, uploads one photo, and the system automatically generates their tenant profile, a fully written product description, and a live store link. The UI must adhere to OHC Premium Design Standards (Glassmorphism, mobile-first 375px, fast animations). Do not prescribe specific database schemas or API contracts; focus on fulfilling the end-to-end user experience.

## Priority
P0

## Estimated Scope
Large