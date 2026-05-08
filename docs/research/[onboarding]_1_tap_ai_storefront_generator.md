# Title: 1-Tap AI Storefront Generator

## Problem Statement
Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by complex website builders. Platforms like Shopify and Wix require users to manually select templates, map out navigation, upload placeholder images, and configure basic settings before seeing a usable store. This setup process is intimidating and time-consuming, preventing many from moving beyond Instagram DMs or word-of-mouth. They need a system that translates a single sentence (e.g., "I'm a baker in Austin selling custom cakes") into a fully functional, mobile-ready store in seconds.

## Research Report
- **Competitive Landscape**:
  - **Shopify**: Setup is highly manual. Shopify Sidekick provides chat-based help but does not autonomously generate the store. Users frequently complain about the steep learning curve (source: 1-star App Store reviews mention "too complicated for beginners").
  - **Wix**: Wix ADI generates a site via a questionnaire, which is better, but it's a one-time wizard rather than a continuous, invisible agent.
  - **Durable/Hocoos**: Emerging AI-native builders generate sites in 30 seconds but lack the deep business management (inventory, POS) backend that SMBs eventually need.
- **User Pain Points**: Analyzing Reddit (r/smallbusiness, r/ecommerce) reveals a recurring theme: "I just want a simple site to take orders, but I get stuck designing it."
- **Opportunity**: OHC can leapfrog incumbents by combining a Durable-style 1-tap generation experience with a robust, invisible business management backend.

## Design Doc
- **High-Level Architecture**:
  - **Input Layer**: A simple mobile UI (375px optimized) with a single text input or voice recording button.
  - **Agent Layer**: The "AutoDream" agent takes the user's unstructured input, extracts business type, location, and key offerings, and generates a structured storefront configuration (theme, placeholder text, suggested catalog structure).
  - **Data Model**: Storefront configuration is mapped to the core `Business`, `Catalog`, and `Theme` entities.
- **Mobile UX Flow (375px first)**:
  1. User opens the app.
  2. Prompt: "Tell me about your business." User types/speaks: "I run a mobile dog grooming service in Seattle."
  3. Loading screen: "Generating your store..." (progress indicators for theme selection, catalog generation).
  4. User is presented with a live, functional preview of their store.
- **AI Agent Integration Points**: The AI agent acts as the translation layer between the user's unstructured intent and the structured OHC database.

## Implementation Prompt
Create a "1-Tap Storefront" feature where a user can input a single sentence describing their business, and the system autonomously generates a fully configured, ready-to-publish storefront. The critical user journey starts at the onboarding screen and ends with a live, mobile-optimized store preview. Ensure the UI feels magical and requires zero technical configuration from the user. Do not prescribe specific database schemas or API contracts; focus on the seamless transition from intent to a functional storefront.

## Priority
P0

## Estimated Scope
Large
