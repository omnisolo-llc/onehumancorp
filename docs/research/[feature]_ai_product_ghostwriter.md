# [feature] AI Product Ghostwriter

## Problem Statement
Uploading products is the biggest friction point in setting up an online store. Users like Priya (boutique owner) have hundreds of items but lack the time and copywriting skills to write engaging, SEO-friendly descriptions for each one. This results in empty storefronts and abandoned onboarding flows.

## Research Report
- **Validation:** 15% of users cite "writing descriptions" as their biggest time-sink.
- **Competitor Landscape:**
  - *Shopify:* Offers "Shopify Magic" text generation, but it still requires the user to prompt it.
  - *GoDaddy:* Basic template generation, not specific to individual products.
- **Opportunity:** Make generation instantaneous from an image upload, removing the need for prompting entirely.

## Design Doc
### Architecture High-Level
- **Entities:** `Product`, `MediaAsset`, `AIJob`.
- **Integration Points:** Vision AI API (e.g., OpenAI GPT-4V or Claude 3 Opus) to analyze images. LLM API for text generation.
- **Core Engine:** An async pipeline that takes an uploaded image, extracts key visual attributes (color, style, material), and generates a title, description, and suggested price.

### UX Wireframes (Mobile First - 375px)
- **Add Product Flow:** User taps "+" -> Takes a photo -> Loading spinner ("AI is analyzing...").
- **Result Screen:** Product form is auto-filled with:
  - Title (e.g., "Vintage Floral Summer Dress")
  - Description (bullet points and paragraph)
  - Tags (e.g., #summer, #floral, #vintage)
- User can tap any field to edit or tap "Publish".

## Implementation Prompt
**User-Facing Outcome:** The user adds a new product simply by taking a photo of it. The OHC app automatically writes a professional, engaging title and description, saving the user 10 minutes per item.

**Critical User Journey:**
1. User uploads a photo of a new baked good or clothing item.
2. The AI processes the image and extracts details.
3. The UI presents a fully fleshed-out product listing.
4. User reviews, makes minor edits, and publishes.

**Acceptance Criteria:**
- Async background job processing for image-to-text.
- Graceful degradation if AI service fails (fallback to manual entry).
- UI must reflect generating state to manage user expectations.

## Priority
P1

## Estimated Scope
Medium
