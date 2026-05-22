# [Feature] AI-Native Zero-Setup Storefront via Chat Onboarding

## Problem Statement
Small business owners (like Maya, the baker) are overwhelmed by the hundreds of settings, theme choices, and app installations required to launch a traditional e-commerce site. They need to sell quickly without technical overhead.

## Research Report
Competitors like Shopify require manual configuration of themes, shipping zones, and taxes, which often frustrates beginners. Rising AI competitors like Mixo and Hocoos can generate a landing page quickly using AI, but they lack the deep backend commerce features needed to actually run the business. OHC has an opportunity to combine instant AI generation with a robust agentic backend.

## Design Doc
- **User Flow (Mobile First, 375px)**:
  1. User opens the OHC app.
  2. The Onboarding Agent initiates a chat: "Hi! What are you selling today?"
  3. User replies (e.g., "Custom cakes") and uploads photos.
  4. The Onboarding Agent parses the intent and passes the data to the Agent Swarm.
  5. The Operations Agent instantly provisions a live URL with a checkout-ready product page, configures default local shipping rules, and integrates payments.
- **Key Relationships**: User -> Onboarding Agent -> Operations Agent -> Storefront/Checkout Service.
- **AI Integration**: NLP parsing of user intent and automatic asset generation/placement.

## Implementation Prompt
Create a critical user journey where a new user can go from downloading the app to having a live, purchasable product link in under 3 minutes. This must be achieved solely by the user chatting with an agent and uploading a photo. The system should automatically handle the underlying site creation and commerce setup invisibly.

## Priority
P0

## Estimated Scope
Large
