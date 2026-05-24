# [onboarding] Invisible AI Storefront Generator

## Problem Statement
Small business owners (like Maya the baker) drop off during the initial setup phase of current platforms. They are overwhelmed by drag-and-drop theme editors, DNS configurations, and building an initial product catalog. The "time-to-live" is measured in hours or days.

## Research Report
Based on reviews from Shopify, Wix, and user feedback on r/smallbusiness, the steepest learning curve is theme customization. While AI builders like Durable offer quick text/image generation, they often produce shallow sites without functional business engines (like inventory management or bookings) attached.

## Design Doc
**Architecture & Key Relationships:**
*   **User Input Interface:** Mobile-first chat UI.
*   **Agentic Orchestrator:** Interprets the chat, extracting business type, name, services, and tone.
*   **Asset Generator:** Calls AI image and text generation APIs to create brand assets.
*   **Business Engine Configurator:** Automatically provisions the underlying database entities (Inventory Items, Service Catalog, Booking Rules) based on the business type.

**UX Flow:**
1.  User downloads OHC app.
2.  Chatbot asks: "What kind of business are you starting?"
3.  User: "I bake custom cakes."
4.  Chatbot asks 2-3 follow-up questions (name, style).
5.  *Agent Working State (Loading Screen)*
6.  User is presented with a fully functional, mobile-optimized storefront with sample inventory already populated.

## Implementation Prompt
Implement the Invisible AI Storefront Generator. The user should be able to complete a short conversational flow on their mobile device and receive a fully functional storefront in under 10 minutes. The Critical User Journey is the initial chat interaction leading directly to a live, transactional site. Acceptance criteria: The generated site must have at least one product/service available for purchase/booking immediately upon creation, without requiring the user to open a traditional editor.

## Priority
P0

## Estimated Scope
Large
