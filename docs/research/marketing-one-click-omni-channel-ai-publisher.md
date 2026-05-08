# [Marketing] One-Click Omni-Channel AI Publisher

**Priority:** P1 | **Estimated Scope:** Large

## Problem Statement
Boutique owners (like Priya) have to manually update inventory, take photos, write descriptions, and post to Instagram, TikTok, and their website separately. It's overwhelming and technical.

## Research Report
**Findings:**
- Content creation is a massive blocker. GoDaddy Airo attempts this but results are generic.
- Real owners use ChatGPT manually, then copy-paste to 4 different apps.
- 60% of product descriptions on SMB sites are blank or unoptimized.
**Evidence:** App Store reviews for Shopify mobile app mention 'takes too long to add a product'.

## Design Doc
**Architecture:**
- Entity Types: `ProductMedia`, `SocialPost`, `PublishingChannel`
- Key Relationships: 1 `ProductMedia` generates N `SocialPost`s across `PublishingChannel`s.
- Mobile UX: Owner uploads a photo from their phone. AI generates the title, description, and 3 social posts. Owner hits 'Publish All'.
- AI Integration: Vision model to describe the photo, LLM to generate channel-specific copy.

## Implementation Prompt
Create a 'Magic Upload' flow on mobile. The user selects a photo, and the system automatically generates the product listing (title, price suggestion, description) AND formats social media posts for Instagram and Facebook, ready to be scheduled or published immediately.
