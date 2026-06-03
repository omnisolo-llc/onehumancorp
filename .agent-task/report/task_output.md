issue_title: "[architecture]_autonomous_unified_brand_and_design_engine"
issue_description: |
  # Issue Brief: Autonomous Unified Brand and Design Engine

  ## Problem Statement
  Small business owners like Priya (Boutique Owner, 35) and Maya (Home Baker, 28) struggle immensely with maintaining a cohesive visual brand across all their touchpoints. They lack the design expertise and the time to ensure their storefront, social media posts, invoices, and physical receipts all look professional and aligned. Currently, they have to use separate tools (Canva for social, Shopify/Wix for storefront, Stripe for invoices) and manually try to match colors and fonts. This results in an inconsistent brand presence, diminishing trust and making them look "small time." They need an invisible design assistant that automatically establishes a core brand identity and flawlessly applies it across every single artifact the business generates, from the 375px mobile storefront to the printed thermal receipt.

  ## Research Report
  - **Shopify:** Provides storefront themes but relies heavily on third-party apps for cohesive social media asset generation. Invoice customization is basic and decoupled from the main theme engine.
  - **Wix:** Offers an AI logo maker and some integrated marketing tools, but the visual consistency across different channels still requires manual intervention and design sense.
  - **Canva:** Excellent for asset creation but completely disconnected from the actual business operations (inventory, checkout, invoicing).
  - **OHC Opportunity:** Leverage our AI agent architecture (specifically the "Marketing & Advertising" and "Operations" departments) to create a unified design system generator. The AI should derive a brand identity (colors, typography, tone) from a simple onboarding chat or logo upload and enforce it universally.

  ## Design Doc
  - **Architecture Diagram (Mental Model):**
    `BrandIdentityRecord` -> `AssetGenerationMesh` -> [Storefront Theme, Invoice Template, Social Media Post, Thermal Receipt Layout, Email Template]
  - **Mobile UX Flow (375px):**
    1.  User enters the "Brand" tab in the OHC app.
    2.  User sees their current "Brand Kit" (Logo, Primary Color, Secondary Color, Font Pairing).
    3.  A "Generate New Look" button allows the AI to suggest entirely new cohesive themes based on their business type (e.g., "Earthy & Organic" for a vegan bakery).
    4.  All previews are shown dynamically applied to mockups of a storefront, an Instagram post, and an invoice, right on the phone screen.
  - **AI Agent Integration Points:**
    -   *Marketing & Advertising (The Promoter):* Uses the Brand Kit to generate on-brand social media posts and storefront layouts.
    -   *Operations (The Manager):* Applies the Brand Kit to transactional emails and physical/digital receipts.
    -   *Finance & Payments (The Accountant):* Applies the Brand Kit to generated invoices and payment links.
  - **Key Design Decisions:**
    -   Centralized `BrandIdentity` entity in the multi-tenant database.
    -   All rendering engines (Flutter frontend, PDF invoice generator, HTML email templates) must pull styling from this central record.
    -   AI generation must output structured design tokens (hex codes, font family names) rather than just flattened images.

  ## Implementation Prompt
  Implement the Autonomous Unified Brand and Design Engine. The user-facing outcome should be a "Brand" settings page where the user can define or let AI generate their core visual identity (logo, colors, typography). This identity must then dynamically propagate to update the styling of the user's public storefront, their generated invoices, and their transactional emails. Create a Playwright E2E test that validates that changing the primary brand color in the settings page successfully updates the primary button color on the public storefront.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
