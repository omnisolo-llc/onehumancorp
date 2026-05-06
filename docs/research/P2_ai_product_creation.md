# [Product Gap] One-Click AI Product Creation

## Title
Implement One-Click AI Product Creation from Photos

## Problem Statement
Adding products to an online store is tedious. Users (like Maya or Priya) have to take a photo, write a compelling title, write a description, figure out pricing, and manage inventory settings. This friction prevents them from updating their store regularly.

## Research Report
*   **Competitor Landscape:** Shopify and Wix offer AI text generation for descriptions, but the user still has to initiate the process step-by-step.
*   **User Pain Point Data:** "Writing product descriptions" is frequently cited as a time-consuming chore in e-commerce forums.
*   **OHC Advantage:** By taking a mobile-first approach, OHC can allow a user to simply snap a picture from the OHC app. The AI automatically analyzes the image, identifies the product, drafts the title and description, and suggests a price.

## Design Doc
*   **Entities:** `Product`, `ProductImage`, `AIGenerationLog`.
*   **Architecture:**
    *   Integration with a multimodal LLM (e.g., GPT-4o or Claude 3.5 Sonnet) capable of image analysis.
    *   Upload pipeline for image handling.
    *   Background worker to process the image and generate the product metadata.
*   **UI Wireframe/Flow (375px first):**
    *   **Screen 1: Inventory.** A large "Quick Add (Camera)" FAB (Floating Action Button).
    *   **Screen 2: Camera/Upload.** User takes a photo or selects from gallery.
    *   **Screen 3: AI Processing.** A loading state ("AI is analyzing your product...").
    *   **Screen 4: Product Draft.** The AI-generated title, description, and suggested price are pre-filled in editable fields. User taps "Save & Publish."

## Implementation Prompt
Create a "Quick Add" feature for products that uses multimodal AI. When a user uploads an image, pass it to an LLM to automatically generate a product title, a compelling description, and a suggested price category. Present these generated fields to the user for review and immediate publishing. This must work seamlessly on mobile viewports.

## Priority
P2

## Estimated Scope
Small
