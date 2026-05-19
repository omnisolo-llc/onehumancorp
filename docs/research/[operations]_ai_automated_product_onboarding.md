# [Operations] AI Automated Product Onboarding

## Title
AI Automated Product Onboarding (The Operations Manager)

## Problem Statement
Adding a new product or service online is tedious. Non-technical owners struggle with writing engaging, SEO-optimized descriptions, pricing strategy, and categorization. Uploading a photo should be the only step required.

## Research Report
Based on the OHC Market Dominance Research Report, "Product Description Fatigue" is the #2 pain point. Competitors like Wix and Shopify require filling out 10+ fields (Title, Description, Price, SKU, Category, Tags, Shipping Weight) just to list an item.
**Cloud vs. Standalone Capability:**
- **Cloud:** Full utilization of vision-capable LLMs to analyze the uploaded image, cross-reference market pricing, and generate rich copy.
- **Standalone:** Relies on smaller local vision models or queues the image analysis until an internet connection is established, providing a draft that the user can refine later.

## Design Doc
**Target Viewport (375px native mobile first):**
- **The Trigger:** A massive, inviting "Add Product" button that immediately opens the camera/gallery.
- **The Magic Moment (Loading State):** While analyzing, show a premium shimmer effect or animated processing state ("The Operations Manager is analyzing your photo...").
- **The Review Screen:** A clean, glassmorphic card presenting the generated content:
  - Suggested Title (Editable)
  - Suggested Price (Editable, with a "Why this price?" tooltip)
  - Generated Description (Editable)
  - Categories (Auto-selected tags)
- **1-Tap Action:** "Publish Product".
- **Post-Publish Handoff:** A toast notification: "Product live! The Promoter is drafting an Instagram post..."

## Implementation Prompt
Build a mobile-first upload flow where submitting an image triggers a vision model analysis. The system should return a structured JSON response containing a title, description, price, and category. Display these suggestions in a clean, editable form matching the OHC design system. Ensure a smooth, animated loading state during analysis.

## Priority
High (P0)

## Estimated Scope
2 weeks (Vision API integration + UI flow + State management)
