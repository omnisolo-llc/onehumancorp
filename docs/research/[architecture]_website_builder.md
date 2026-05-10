# Title: Implement AI-First, Block-Based Website & Storefront Builder

## Problem Statement
Small business owners (like Maya the baker or Carlos the handyman) need a professional online presence to grow their business. However, existing solutions (Shopify, Wix, Squarespace) often require desktop environments for creation, present overwhelming template choices, and force users to write their own copy. This violates the "Grandmother Test" and our mobile-first mandate. We need a builder that allows users to launch a complete, beautiful site directly from their phone in under 10 minutes, using AI to do the heavy lifting.

## Research Report
An analysis of the competitive landscape reveals a significant gap in True Mobile-First Creation coupled with AI Delegation.
- **Shopify/Wix:** Often require falling back to a desktop for full site creation or complex customizations.
- **Squarespace:** Offers "Visual Excellence" but their Fluid Engine can be confusing to manage on mobile.
- **Opportunity:** OHC will use strict, beautifully designed, natively responsive content blocks. "The Promoter" AI agent will generate the initial structure and copy, meaning users refine rather than create from scratch.

## Design Doc

### Architecture Diagram
```mermaid
graph TD
    subgraph Client (Mobile App/Web)
        UI[Slint UI Components]
        B[Block Editor]
    end

    subgraph OHC Backend
        API[Builder API]
        DB[(PostgreSQL - JSONB)]
        CDN[Edge CDN]
    end

    subgraph KAIROS Orchestrator
        Promo[The Promoter AI]
        Inv[Inventory DB]
        Cal[Calendar DB]
    end

    UI --> B
    B <--> API
    API <--> DB
    API --> CDN

    Promo -->|Generates Initial Site JSON| DB
    Promo -->|Suggests Content Updates| API

    UI -.->|Dynamic Content| Inv
    UI -.->|Dynamic Content| Cal

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class UI,B,API,DB,CDN,Promo,Inv,Cal premium;
```

### Content Blocks
The builder uses discrete, swappable blocks that act as pre-defined UI components.
- **Hero:** Main image, Headline (AI-generated), Subtitle, CTA (e.g., "Order Now").
- **Product Grid:** Dynamically pulls from the Inventory database.
- **Services/Booking:** Integrates with the Calendar/Scheduling system.
- **Menu:** For food/beverage businesses (e.g., Fatima), easily updated from a phone.
- **Testimonials/Reviews:** Pulled automatically by the Customer Success Agent ("The Ambassador").
- **Contact/Lead Gen:** Simple forms routing directly to the KAIROS shared task list.
- **Text:** General text.

### Template Engine & Customization
- **Theme Variables:** Colors, typography (Outfit for headings, Inter for body), and border-radius are defined globally as CSS variables/design tokens. Changing the "vibe" updates all blocks instantly.
- **Glassmorphism:** The Visual Excellence Mandate requires subtle blur and transparency effects (e.g., `backdrop-filter: blur(20px) saturate(200%)`) on overlays and modals.

### Publishing Workflow
- **Draft → Live Publishing:** A unified state. Drafts are viewable via a secure preview link. Publishing pushes the static assets to the edge CDN. 1-Tap "Publish" button pushes the site live to the OHC subdomain or custom domain.

### SEO
- **SEO Automation:** Meta titles, descriptions, and alt tags are generated invisibly by the AI.

### Custom Domains & SSL
- **Seamless provisioning of custom domains.**
- **Automatic SSL certificate generation** (e.g., via Let's Encrypt) for all tiers above Free.

### Mobile UX Flow (375px First)
1. **Onboarding:** User answers 3 simple questions via the setup wizard (e.g., "What do you sell?", "What's your vibe?").
2. **Generation:** "The Promoter" AI generates a complete draft site with placeholder text and images.
3. **Editing:** The user enters the Block Editor. They see a vertical stack of content blocks.
4. **Refining:** Tapping a block opens a simple property sheet (e.g., change image, edit text, or tap "Rewrite with AI").
5. **Publishing:** 1-Tap "Publish" button pushes the site live.

### AI Integration Points
- **"The Promoter" (Marketing & Advertising):**
  - Generates the initial JSON structure of the site during onboarding.
  - Generates SEO metadata (titles, descriptions, alt text) invisibly.
  - Suggests seasonal updates (e.g., "Add a holiday banner") via the KAIROS Orchestrator for 1-Tap Approval.

### Key Design Decisions
- **Strict Block Constraints:** No free-form drag-and-drop. Users build sites by stacking predefined blocks (Hero, Product Grid, Service Booking, etc.) to guarantee 100% mobile responsiveness and Visual Excellence.
- **JSONB Storage:** The site structure is stored as a JSONB object in PostgreSQL, allowing for flexible schema evolution and easy serialization to the edge CDN.
- **Unified Editor:** The editor interface is exactly the same as the final rendered output, just with interactive overlay controls.

## Implementation Prompt
Implement the core engine for the Website & Storefront Builder.
- **Outcome:** A user can create, edit, and publish a multi-block website entirely from a mobile device (375px viewport).
- **CUJ (Critical User Journey):**
  1. The user starts with a generated draft site (represented as JSON).
  2. The user can add a new "Text" block and a "Product Grid" block.
  3. The user can edit the text in the Text block.
  4. The user taps "Publish", making the site available at a public URL.
- **Acceptance Criteria:**
  - The builder must use strict content blocks; do not implement free-form positioning.
  - The UI must adhere to the Visual Excellence Mandate (Glassmorphism, Outfit/Inter typography).
  - The data model must support serializing the block structure to and from the database.
  - The editor must be fully functional on a 375px screen.

## Priority
P0

## Estimated Scope
Large