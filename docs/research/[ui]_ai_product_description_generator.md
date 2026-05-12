# AI Product Description Generator

## Title
1-Tap AI Product Listing Generation from Photos

## Problem Statement
Adding new products to an online store is tedious for owners like Priya (boutique owner). She must take a photo, manually crop it, write an engaging description, set SEO tags, and input pricing—often typing all this on her phone. This friction causes product backlogs, meaning new inventory isn't immediately available online, directly hurting sales.

## Research Report
Adding products is a significant barrier to entry on platforms like Shopify and Wix. While some platforms offer AI text generation, it still requires the user to input bullet points or basic descriptions first. Users on r/ecommerce complain that writing SEO-optimized copy takes up to 30 minutes per product. OHC can leapfrog competitors by fully automating this: extracting metadata and drafting the entire listing purely from an uploaded image.

## Design Doc
**High-Level Architecture:**
- **Entity Types:** `Product`, `ProductImage`, `AIGeneratedListing`.
- **Key Relationships:** A `Product` has multiple `ProductImage`s.
- **Integration Points:** Vision AI service (e.g., GPT-4o Vision), Image processing pipeline (compression, auto-cropping).
- **Mobile UX Flow (375px first):**
  1. User taps "Add Product" and snaps a photo or uploads from the camera roll.
  2. A skeleton loading state appears while the Vision AI processes the image.
  3. The AI populates the product form with a generated Title, engaging Description, inferred Category, and SEO tags.
  4. The user inputs the Price (the only required manual step) and taps "Publish".
- **AI Agent Integration:** A pipeline orchestrator receives the image, sends it to a vision model to identify the object, style, and features, and then prompts an LLM to generate market-ready copy based on the business's predefined brand voice.

## Implementation Prompt
Implement a feature in the mobile product creation flow that allows users to generate a complete product listing from a single image. Upon image upload, use a Vision AI to analyze the photo and automatically fill in the product title, description, and category. The generated text should be editable before the user saves the product.
Acceptance Criteria:
- Uploading an image automatically triggers the AI generation process.
- Title and description fields are populated within 5 seconds.
- The user can seamlessly edit the generated text before publishing.

## Priority
P1

## Estimated Scope
Medium
