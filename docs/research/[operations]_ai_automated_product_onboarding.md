# Issue Brief: AI Automated Product Onboarding (The Operations Manager)

## 1. Context & Problem
**Pain Point Addressed:** #2 Product Description Fatigue.
Small business owners, particularly those with frequently changing inventory (like boutique owners or bakers), find writing engaging, SEO-optimized descriptions for every new item incredibly time-consuming. This friction often results in delayed online listings, sparse catalogs, and lost sales opportunities.

## 2. Objective
Create a seamless, mobile-first flow where a user uploading a single photo of a product triggers the "Operations Manager" agent to automatically generate a complete product listing (description, pricing, categorization) and the "Promoter" (Marketing) agent to draft a launch post for social media.

## 3. Scope & Requirements

### 3.1. Photo-Triggered Generation
*   **Input:** A single user-uploaded photo from their mobile device.
*   **Computer Vision Integration:** The AI must analyze the image to identify the product, its features, and potential variations (e.g., recognizing a "Vegan Choc Cake").
*   **Automated Listing Creation:**
    *   **Description:** Generate a full, engaging, and SEO-optimized product description.
    *   **Pricing:** Suggest a realistic price based on historical data, similar products, or market trends.
    *   **Categorization:** Automatically assign the product to the correct categories/collections within the catalog.

### 3.2. Marketing Automation (The Promoter)
*   **Event-Driven Trigger:** Upon the successful creation of a new product listing (`NewProductAdded` event), automatically notify the Marketing agent.
*   **Content Drafting:** The Marketing agent drafts a social media post (e.g., for Instagram/Facebook) announcing the new product.
*   **Cross-Channel Synergy:** Ensure the tone of the post matches the business's branding and references the newly generated product details.

### 3.3. User Experience (Mobile First)
*   **1-Tap Approval:** Both the product listing and the marketing post must be presented to the user for quick, 1-tap approval.
*   **Editability:** Allow the user to easily tweak the generated description, price, or social post text before finalizing.
*   **Zero Complex Wizards:** Bypass traditional, multi-step onboarding forms. The entire process should feel like a conversation or a rapid review queue.

### 3.4. Architecture & Safety
*   **Multi-Tenant Isolation:** Ensure all generated data (products, prices, drafts) is strictly scoped to the user's `tenant_id`.
*   **Idempotency:** Prevent duplicate product creations or social posts if the user accidentally taps "Approve" multiple times or if network issues occur.

## 4. Expected Impact
*   **Time to Market:** Reduce the time it takes to list a new product from minutes/hours to under 30 seconds.
*   **Catalog Richness:** Increase the overall quality and SEO performance of product catalogs across the platform.
*   **Marketing Consistency:** Drive more traffic by ensuring every new product is accompanied by a social media announcement.

## 5. Implementation Notes
*   Integrate with the existing media storage service to handle the initial photo upload.
*   Utilize a vision-capable LLM provider for the initial image analysis.
*   Ensure the `NewProductAdded` event is reliably dispatched and consumed by the Marketing department via the background job queue.
