# [AI] Instant Product Description Generator

## Title
Instant AI Product Description and SEO Generator

## Problem Statement
Uploading new products is a major friction point. Small business owners hate writing product descriptions and don't understand SEO. It takes too long, so they delay adding new inventory.

## Research Report
- **Competitor Landscape**:
  - Shopify has a "magic text" generator but it still requires manual prompting.
  - Wix provides basic suggestions.
- **User Pain Points**:
  - "I have 50 new items to list and just thinking about writing the descriptions makes me want to quit." (Reddit r/ecommerce).
- **Differentiation**:
  - OHC will allow merchants to take a photo of the product; the AI agent will identify it, write a compelling description, generate SEO tags, and categorize it automatically.

## Design Doc
- **Architecture**:
  - Entity: `Product`, `SEOMetadata`.
  - Integration: Vision LLM (e.g., GPT-4 Vision) to analyze product images.
- **UI Wireframes/Flow**:
  - Mobile UX (375px): "Add Product via Photo".
  - Camera takes photo -> Loading state ("AI is writing...") -> Screen shows generated Title, Description, Price Suggestion, and Tags.
  - User taps "Publish".

## Implementation Prompt
Implement the Instant Product Description Generator. The Critical User Journey starts when a merchant uploads a photo of a new product. The system should use an AI vision model to analyze the image, generate a title, description, and SEO metadata, and present it for approval.
- **Acceptance Criteria**:
  - Image upload triggers AI vision analysis.
  - AI generates text and metadata fields.
  - User can edit generated text before publishing.

## Priority
P1

## Estimated Scope
Medium
