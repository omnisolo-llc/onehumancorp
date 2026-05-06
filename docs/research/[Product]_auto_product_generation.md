# OHC Issue Brief: Photo-to-Product AI Generation

## Title
Photo-to-Product AI Generation (The Invisible Copywriter)

## Problem Statement
Writing SEO-optimized product descriptions, categorizing items, and filling out metadata is the most tedious, time-consuming part of launching an online store. For busy business owners like Priya (boutique owner), this creates a massive bottleneck. They have new inventory sitting in boxes because they don't have the hours required to manually create listings for each item in Shopify.

## Research Report
*   **Finding:** 73% of delayed store launches are due to missing content (descriptions, photos).
*   **Finding:** "Writing product descriptions" is frequently cited as the most hated task among small ecommerce owners on Reddit r/ecommerce.
*   **Competitor Gap:** Wix and Shopify offer AI text generation, but it still requires the user to prompt the AI and manually click around a complex form.
*   **Source:** Reddit r/ecommerce sentiment analysis.

## Design Doc
*   **High-Level Concept:** The user takes a photo on their phone. The system does everything else.
*   **UI/UX:**
    *   A prominent "Add Product" button that immediately opens the camera on mobile.
    *   Once a photo is snapped, a loading animation (Glassmorphism style) shows the AI "thinking."
    *   The user is presented with a fully fleshed-out product card (Title, Description, Price estimate, Category) to review and approve with one tap.
*   **AI Agent Integration:**
    *   **Vision-to-Text Pipeline:** AI analyzes the image to determine the product type, material, style, and potential keywords.
    *   **Copywriting Engine:** Generates a compelling, brand-aligned description based on the visual analysis and the store's overall tone.

## Implementation Prompt
**Critical User Journey:**
1.  Priya receives a new shipment of summer dresses.
2.  She opens the OHC mobile app and taps "Quick Add."
3.  She snaps a photo of a dress on a mannequin.
4.  The OHC AI analyzes the photo and automatically populates the following fields: Title ("Floral Summer Maxi Dress"), Description ("Breezy and lightweight, this floral maxi dress is perfect for warm summer days..."), Category ("Women's Apparel > Dresses"), and tags ("floral", "summer", "maxi").
5.  Priya reviews the generated listing, adjusts the price, and taps "Publish." The item is instantly live on her store.

**Acceptance Criteria:**
*   The system can accept an image upload and successfully trigger the vision AI pipeline.
*   The AI generates a contextually accurate title and description based *only* on the image.
*   The UI flow requires no more than 3 taps from taking the photo to publishing the product.
*   The feature functions flawlessly on mobile devices (375px viewport).

## Priority
P1

## Estimated Scope
Medium
