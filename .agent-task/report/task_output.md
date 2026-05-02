# [architecture] Website & Storefront Builder Architecture

## Problem Statement
Small business owners—whether they are a baker like Maya or a handyman like Carlos—need a fast, intuitive way to establish an online presence. Existing website builders (Shopify, Wix, Squarespace) offer a blank canvas or complex templates that require hours of customization, technical knowledge (DNS, SEO), and design sensibility. These platforms fail the "zero technical knowledge required" mandate. OHC needs an invisible, AI-driven builder that asks minimal questions and instantly generates a beautiful, mobile-first storefront that works seamlessly with the KAIROS Orchestrator.

## Research Report
Our analysis of the website builder market highlights a significant opportunity for AI integration:
- **Shopify & Wix:** Require 30-60 minutes of setup, assuming the user understands domain mapping, theme customization, and SEO basics. Their AI features are mostly isolated to text generation or basic image creation.
- **Squarespace:** Focuses heavily on design but still expects the user to manually arrange blocks and configure settings.
- **OHC Opportunity:** The OHC platform will not just "assist" in building; it will "do" the building. By leveraging the **Marketing & Advertising Agent (The Promoter)**, the system will instantly assemble a complete storefront from a few simple inputs (Business Name, Category). The drag-and-drop builder is only necessary for minor post-generation tweaks, not the primary creation flow.

## Design Doc

### Architecture Diagram
```mermaid
flowchart TD
    subgraph Inputs
        Wizard[Onboarding Wizard]
        Agent[The Promoter Agent]
        User[User Adjustments]
    end

    subgraph Core Engine
        Engine[Storefront Generator]
        Blocks[Content Block Registry]
        Theme[Theme Engine (Tokens)]
        SEO[Auto-SEO Generator]
    end

    subgraph Storage & Serving
        DB[(PostgreSQL - JSONB)]
        CDN[CDN / CloudFront]
        SSL[Auto-SSL Provisioning]
    end

    Wizard --> Agent
    Agent --> Engine
    User --> Engine

    Engine --> Blocks
    Engine --> Theme
    Engine --> SEO

    Engine --> DB
    DB --> CDN
    CDN <--> SSL
```

### Key Design Decisions
1. **Content Blocks as Primitives:**
   - The builder uses a predefined set of semantic blocks: Hero, Product Grid, Service List, Booking Calendar, Testimonials, and Contact Form.
   - Users cannot break the design; they can only reorder or configure blocks.
2. **AI-Driven Generation:**
   - *The Promoter* agent selects the appropriate template and populates it with generated copy, placeholder (or user-provided) images, and predefined block structures based on the business type (e.g., Maya gets a Product Grid, Carlos gets a Service List + Booking Calendar).
3. **Draft to Live Publishing:**
   - Changes are saved instantly as "Draft" in PostgreSQL (JSONB payload representing the block structure).
   - "Publishing" triggers a background job that optimizes assets, updates the CDN cache, and makes the JSONB payload the active live version.
4. **Zero-Config Infrastructure:**
   - **SEO:** Generated automatically from the business profile and content blocks (structured data, meta tags).
   - **Domains & SSL:** OHC subdomains provided by default. Custom domains use an automated Let's Encrypt pipeline for zero-click SSL.

### UI Wireframes Description
- **Storefront Editor (Mobile-First, 375px):**
  - **Live Preview:** A full-screen preview of the site as it looks on a phone.
  - **Floating Action Button (FAB):** "Add Section".
  - **Bottom Sheet:** Tapping a section (e.g., Hero) opens a bottom sheet with simple controls (Change Image, Edit Title) instead of a complex sidebar.
- **Theme Switcher:**
  - One-tap buttons for premium themes (e.g., "Glassmorphism", "Minimal", "Bold"). These apply strict OHC Premium Tokens (colors, fonts, blurs) globally.

### Mobile UX Flow
1. **Creation:** User completes the 3-step onboarding wizard.
2. **Generation:** Loading screen ("The Promoter is building your site...").
3. **Review:** User is dropped into the Storefront Editor with a fully generated site.
4. **Edit:** User taps the Hero image to upload a real photo.
5. **Publish:** User taps "Publish". The site is instantly live on their OHC subdomain.

## Implementation Prompt
**Task:** Implement the core Storefront Engine and Content Block Registry for the AI-driven website builder.
**CUJ:** A non-technical user (e.g., Carlos the handyman) completes onboarding. The system must automatically generate a mobile-first website containing a Hero section, Service List, and Booking Calendar, all styled with the default OHC Premium Tokens. The user must be able to view the live preview and publish it to an OHC subdomain without encountering any technical configuration.
**Acceptance Criteria:**
- Define the JSONB schema for storing the content block structure.
- Implement the rendering engine that translates the JSONB schema into the final frontend output (mobile-first).
- Integrate with *The Promoter* agent to auto-generate the initial JSONB payload based on business type.
- E2E tests must verify the complete flow: onboarding submission -> AI generation -> draft save -> publish -> live site accessibility.

## Priority
P0

## Estimated Scope
Large
