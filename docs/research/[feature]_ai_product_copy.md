# [feature] AI Product Copywriter

## Problem Statement
Fatima (Food Cart Owner) and Priya (Boutique Owner) struggle to write engaging, SEO-optimized product descriptions for their inventory. Writing takes too much time, and English may not be their first language. They need a way to instantly generate high-quality product listings from just a photo.

## Research Report
Reddit (r/ecommerce) and Trustpilot reviews show that adding inventory is the largest bottleneck to launching a store. Owners spend an average of 15-20 minutes per product writing descriptions and configuring SEO tags. Competitors offer basic AI text generation, but it still requires the user to input detailed prompts and parameters.

## Design Doc
*   **Architecture**: A mobile-first flow where the user uploads an image. A vision-capable LLM analyzes the image, extracts product details (color, type, potential material), and generates a title, description, and SEO metadata.
*   **UX Flow**: The user taps "Add Product" and takes a photo with their phone camera. The UI displays a "Generating..." state. The AI returns a fully populated product form. The user reviews, optionally edits the price, and taps "Publish".
*   **Mobile UX**: The feature is heavily reliant on the native camera integration at the 375px viewport.

## Implementation Prompt
Implement a vision-based AI product generation tool. The user uploads a single image, and the system must return a complete product listing (title, description, inferred category, and SEO tags) without requiring any text input from the user initially.

## Priority
P1

## Estimated Scope
Medium
