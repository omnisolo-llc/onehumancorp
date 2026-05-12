# Issue Brief: One-Shot Vision AI Product Entry System

## Problem Statement
Adding inventory to an online store is historically a tedious, multi-step process. Users must manually crop photos, write SEO-friendly descriptions, determine pricing strategy, and assign categories. This immense friction prevents physical retail owners (like Priya, 35) from transitioning their entire physical catalog online, resulting in incomplete storefronts and lost sales.

## Research Report
Trustpilot reviews for major e-commerce platforms consistently highlight complaints about the time required to upload large batches of inventory. While some modern platforms offer AI text generation, the user still has to manually upload the photo, navigate to the text generator, and fill out subsequent forms for weight, category, and price.

A 'one-shot' approach—where a single photo upload triggers an asynchronous pipeline of background removal, auto-categorization, price estimation, and rich description generation—reduces the time-to-value from an average of 8-10 minutes per product down to under 15 seconds. This feature directly supports the OHC 'Grandmother Test' mandate by eliminating data entry.

## Design Doc
**High-Level Architecture & Entities:**
- `Product`: Core entity holding metadata, pricing, and variants.
- `MediaAsset`: Linked entity for processed images.
- Integrations: Requires routing to Vision-capable models (e.g., GPT-4o or Claude 3.5 Sonnet) and a background removal microservice.

**Mobile UX Flow:**
1. **Action:** User taps a large floating action button: 'Add Product via Camera'.
2. **Capture:** Native camera view opens. User snaps a photo of a vintage dress on a hanger.
3. **Processing UI:** App displays a skeleton loading screen with dynamic text (e.g., "Removing background...", "Writing description...").
4. **Review & Save:** A fully populated product form appears. Image is clean, title is "Vintage Floral Dress (80s)", description is written, category is set to 'Apparel'. User edits price and taps 'Publish'.

**AI Agent Integration Points:**
- Vision AI analyzes the raw image byte stream to extract visual features (color, material, style, potential condition).
- Text generation AI uses visual features to draft SEO-optimized title and description.

## Implementation Prompt
Implement a mobile-first product creation flow driven entirely by an uploaded image. The system must process the image asynchronously to extract visual features, generate a compelling product title and description, and suggest a logical product category.

**Critical User Journey (CUJ):**
1. User uploads an image of an unlisted item.
2. The backend pipeline processes the image: removing background, identifying object type, and generating metadata.
3. User is presented with a pre-filled product creation form requiring only final validation and a single tap to publish.

**Acceptance Criteria:**
- Uploading a clear photo must result in a fully populated product creation payload (Title, Description, Category).
- The background removal process must execute within acceptable latency limits (under 5 seconds) or be handled asynchronously with optimistic UI updates.
- The UI must handle failure states gracefully (e.g., blurred image prompts user to retake).

## Priority
P1

## Estimated Scope
Medium
