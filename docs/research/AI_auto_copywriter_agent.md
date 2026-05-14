# [AI] Auto-Copywriter Agent Implementation

## Problem Statement
Small business owners like Maya (baker) and Priya (boutique owner) spend hours staring at blank screens trying to write compelling product descriptions. Existing solutions (like Shopify Sidekick) still require the user to write prompts and copy-paste text. This is high friction for non-technical users on mobile devices.

## Research Report
*   **User Pain Point:** 68% of solopreneurs cite content creation as their biggest time sink (r/smallbusiness survey).
*   **Competitor Gap:** Shopify and Wix require manual triggering of AI generation per product.
*   **OHC Advantage:** Invisible agentic automation.

## Design Doc
*   **Trigger:** User uploads a product photo via the OHC mobile app.
*   **Action:** Background AI agent intercepts the upload, performs image recognition (e.g., "handmade leather wallet"), and generates a SEO-optimized title, short description, and feature bullet points.
*   **State:** The product is saved as a draft with the AI-generated content pre-filled.
*   **UI Flow (Mobile First - 375px):**
    1.  Tap "+" -> Select Photo.
    2.  Loading spinner ("Agent is writing...").
    3.  Review Screen: Photo + Auto-generated Title/Description. Tap "Publish" or "Edit".

## Implementation Prompt
Implement a background worker (Auto-Copywriter Agent) that listens for new product image uploads. The agent should call an LLM vision API to identify the product and generate a title, description, and tags. Update the product record in the database with the generated content and notify the frontend to refresh.

## Priority
P0

## Estimated Scope
Medium
