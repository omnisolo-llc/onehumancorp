# 🔮 Oracle Issue Brief: Zero-Click Product Uploader

## Title
Implement Photo-to-Storefront AI Product Generation

## Problem Statement
Small business owners (like Maya the baker) hate writing product descriptions, categorizing items, and figuring out pricing strategies. "I have no time to write product descriptions" is a top 6 pain point. The friction of manually typing out details on a mobile keyboard prevents them from adding inventory and growing their online store.

## Research Report
- **Top Pain Point**: "I don't know how to build a website" and "I have no time to write product descriptions."
- **AI Differentiation**: Competitors offer basic AI copywriting (Shopify Sidekick), but it still requires the user to prompt it. OHC must offer a "Zero-Click" experience where a photo drives the entire process.
- **Target Persona**: Maya (Baker), Priya (Boutique owner).

## Design Doc
- **High-level Architecture**:
  - `Product` entity: needs fields for title, description, price, category, and image URL.
  - **AI Agent Integration**: An image processing pipeline where the LLM analyzes an uploaded image to extract product details.
- **UI Flow (Mobile First - 375px)**:
  - User taps "Add Product".
  - Camera opens. User snaps a photo of a cake.
  - Loading spinner ("AI is analyzing...").
  - Screen presents a fully drafted product: Title ("Artisan Strawberry Shortcake"), engaging description, suggested price, and category.
  - User taps "Approve & Publish".

## Implementation Prompt
Build the AI-driven product creation flow.
- When a user uploads an image, the system should invoke the builtin AI agent to generate a product title, description, and suggested category based on the image contents.
- The UI must allow the user to review these generated details before saving them to the database.
- The Critical User Journey: User uploads photo -> AI generates metadata -> User approves -> Product is live on their storefront.

## Priority
P1

## Estimated Scope
Medium
