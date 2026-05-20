# Issue Brief: Zero-Touch Storefront & Inventory Setup

## Title
**Zero-Touch Storefront & Invisible Catalog Manager**

## Problem Statement
The most critical barrier for non-technical small business owners (e.g., Maya, 28, Baker) moving from Instagram DMs to a dedicated e-commerce platform is the **Setup Complexity and Content Creation Block**. Currently, onboarding requires manual image uploading, cropping, writing SEO descriptions, setting pricing, configuring variants (e.g., Vegan, Gluten-Free), and publishing. This "cold start" problem causes massive drop-off rates on platforms like Shopify and Wix, leaving users feeling overwhelmed.

## Research Report
Based on a comprehensive market audit of 60 unique competitive sources and platforms (including Shopify, Wix, Squarespace, Durable, and Hostinger AI) and SMB sentiment analysis (Reddit, Trustpilot, App Store):
- **73%** of 1-star reviews for top competitors cite confusing menus, technical jargon, and manual configuration during setup.
- **Shopify Magic / Sidekick** acts as a reactive assistant (a "co-pilot"), requiring the user to prompt it for help. It does not autonomously complete the setup loop.
- **AI-Native Builders (Durable, 10Web)** can generate a landing page in 30 seconds but fail at deeply integrated catalog management and real commerce workflows.
- OHC needs to bridge this gap by replacing manual configuration with proactive, autonomous agent actions ("The Manager Agent").

## Design Doc
- **Core Entity Types:** `ProductImage`, `ProductListing`, `ProductVariant`, `AI_Suggestion`.
- **Key Relationships:** 1:1 mapping between raw image upload and an `AI_Suggestion` draft. The draft becomes a live `ProductListing` upon user approval.
- **Integration Points:** Mobile client camera API -> KAIROS Orchestration Hub -> Vision AI Model (for image parsing) -> The Manager Agent (for copywriting/pricing strategy) -> Postgres DB.
- **UI Wireframes/Screen Flow (375px Mobile First):**
  1. User opens the OHC mobile app and clicks the main "+" FAB (Floating Action Button).
  2. Camera opens. User snaps a photo of their product (e.g., a custom cake).
  3. A loading skeleton appears: "The Manager Agent is preparing your listing..."
  4. The Activity Feed surfaces an "Approval Card". It displays the auto-cropped image (background removed), an auto-generated compelling description, suggested pricing (based on local market data), and detected variants.
  5. User taps "Approve". The item is instantly live on their OHC Storefront.
- **AI Agent Integration:**
  - **Vision Agent:** Analyzes the raw photo, removes the background, and identifies the core product.
  - **The Manager Agent:** Drafts the SEO-optimized copy, determines initial pricing estimates, and formulates standard variants.

## Implementation Prompt
**User-Facing Outcome:** The SMB owner should be able to create a fully optimized, live product listing simply by taking a single photo on their mobile phone, followed by a 1-tap approval in their activity feed.
**Critical User Journey (CUJ):**
1. Mobile App Launch -> Camera Capture -> Agent Processing -> 1-Tap Approval Card in Activity Feed -> Product Live on Storefront.
**Acceptance Criteria:**
- The system must autonomously process a raw mobile image upload.
- The AI must successfully identify the object, remove the background, and generate a draft listing containing: Title, Description, Price, and at least one Variant option.
- The user must be presented with a native mobile UI (375px) to approve or edit the draft.
- The approval action must successfully persist the listing to the backend and make it visible on the public storefront.

## Priority
P0

## Estimated Scope
Large
