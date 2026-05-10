**Title**: Autonomous Product Cataloging ("Magic Inventory") via Photo

**Problem Statement**:
Adding products to an online store is tedious and time-consuming. Users like Priya (boutique owner) or Maya (baker) have to take photos, transfer them to a computer, write SEO-friendly descriptions, set prices, and manage inventory counts. This friction prevents them from keeping their online presence up-to-date with their physical reality.

**Research Report**:
Current platforms (Shopify, Wix) offer AI text generation *after* a user has started creating a product. GoDaddy's Airo helps with initial setup but lacks ongoing magic. The primary friction point is the multi-step process. Deskless owners want a mobile-first "point, shoot, and publish" workflow. By reducing the time-to-publish from 10 minutes per item to 30 seconds, OHC can secure significant loyalty.

**Design Doc**:
*   **Architecture Flow**:
    1.  Mobile client captures an image and sends it to the OHC API (`/api/products/magic-upload`).
    2.  The backend routes the image to a vision-capable LLM (e.g., Gemini Pro Vision or GPT-4V integration).
    3.  The agent extracts: Product Name, Category, Detailed Description, Suggested Price (based on market data or user history), and attributes (color, material).
    4.  A new Product entity is created in a "Draft" state.
    5.  The extracted data is pushed to the client via WebSockets or polling for user review.
*   **Mobile UX Flow (375px first)**:
    *   User opens the OHC app, taps a prominent "+" button, and selects "Scan Item".
    *   Camera opens. User snaps a photo of a new dress.
    *   A shimmering loading state ("AI is analyzing...") appears.
    *   A card pops up with the generated title, description, and price.
    *   User taps "Publish" (or edits a field if needed). Done.

**Implementation Prompt**:
Implement an endpoint and background agent task that accepts an image upload, processes it using a vision LLM to extract product details (name, description, category, suggested price), and creates a draft product record in the database.
*   **Critical User Journey (CUJ)**: Priya takes a picture of a new scented candle. Within 10 seconds, the app suggests the title "Hand-Poured Lavender Breeze Candle", writes a compelling 3-sentence description highlighting its calming properties, suggests a price of $24, and marks it ready to publish.
*   **Acceptance Criteria**: The system accurately identifies common objects, generates coherent descriptions, and successfully persists the draft product to the database.

**Priority**: P1
**Estimated Scope**: Medium
