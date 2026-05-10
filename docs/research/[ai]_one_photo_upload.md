# SMB Pain Point: Time-Consuming Product/Service Management

## Problem Statement
For non-technical business owners, adding a new product or service to their website is a daunting chore. They must take a photo, transfer it to a computer, navigate a complex dashboard, manually crop/resize the image, write a compelling, SEO-friendly description, and figure out pricing. This friction often results in stale websites and lost revenue, especially for visually driven businesses like bakers or boutique owners.

## Research Report
*   **The Reality:** Many micro-businesses operate exclusively from their phones.
*   **Competitor Approach:** Shopify and Wix require manual data entry for titles, descriptions, and SEO metadata. While some are introducing AI text generators, the user still has to initiate the generation process and manage the fields individually.
*   **The Gap:** There is no "magic" flow. The process is still multi-step and tedious.
*   **The OHC Solution:** The "One-Photo" upload. The owner snaps a photo on their phone, and the OHC agent handles the rest automatically.

## Design Doc
*   **Core Entities:** `Product`, `MediaAsset`, `AIJob`
*   **Integration Points:**
    *   Mobile device camera/gallery API.
    *   Minimax LLM (Vision capabilities) for image analysis.
    *   Image processing pipeline (for auto-cropping and compression).
*   **UX Flow (Mobile First):**
    1.  User taps a large "+" button on the home screen.
    2.  User takes a photo of a new item (e.g., a custom cake) or service outcome.
    3.  A "Generating..." screen appears briefly.
    4.  The app presents a draft product listing: The image is auto-cropped; a catchy title and an SEO-friendly description are generated based on the image analysis; a suggested price is provided (if historical data exists).
    5.  The user reviews, makes quick edits if necessary, and taps "Publish."

## Implementation Prompt
Design and implement the backend orchestration for the "One-Photo" upload flow. This requires an endpoint that accepts a raw image upload from the mobile client. The system must then orchestrate a series of tasks: 1) Image processing (resizing/cropping), 2) Submitting the image to a Vision-capable LLM to extract context, 3) Generating a title, description, and SEO metadata based on that context, and 4) Returning the compiled draft data to the client for review before final persistence to the database.

## Priority
P1

## Estimated Scope
Medium
