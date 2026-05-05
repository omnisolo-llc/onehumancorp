# Issue Brief: Website & Storefront Builder Architecture

## Title
Website & Storefront Builder Architecture: AI-Driven Drag-and-Drop Generation

## Problem Statement
Small business owners (e.g., Maya the Baker, Carlos the Handyman) are overwhelmed by the complexity of traditional website builders like Shopify or Wix. They do not want to manage domains, understand DNS, or fiddle with padding and margins. They need a simple, intuitive, mobile-first builder that automatically creates a premium, functional storefront (with bookings, products, and a contact form) in under 10 minutes. The builder must seamlessly integrate with AI agents for content generation, SEO, and ongoing optimization, hiding all technical complexities.

## Research Report
- **Goal**: Design a block-based architecture for the drag-and-drop website builder that prioritizes absolute simplicity and AI automation.
- **Findings**:
  - Existing solutions (Wix, Squarespace) offer a blank canvas or complex templates, which leads to "setup paralysis".
  - OHC's approach focuses on "Instant Generation" using "The Advisor" and "The Promoter" AI agents to create a fully functional draft based on a single prompt.
  - The builder must support a "Draft" and "Live" state, allowing safe experimentation before publishing.
  - SEO should be entirely automated by "The Promoter" agent.
  - Custom domains and SSL certificates should be provisioned automatically behind the scenes (e.g., via Cloudflare or similar API) without user intervention.
- **Competitive Analysis**: Shopify's theme editor is too complex for non-technical users. GoDaddy's Airo is fast but produces generic, limited sites. OHC's builder will combine speed with premium, functional blocks tailored to specific business types (e.g., booking blocks for handymen, product grids for bakers).

## Design Doc

### High-Level Architecture
- **Content Blocks**: The builder will use a predefined set of premium, mobile-optimized blocks (Hero, Product Grid, Service List, Booking Calendar, Testimonials, Contact Form). Users cannot break the design; they can only reorder or configure specific block parameters.
- **State Management**:
  - `Draft State`: Modifications are saved to a `page_drafts` table (linked to `tenant_id`).
  - `Live State`: Upon clicking "Publish," the draft is serialized into optimized JSON/HTML and deployed to the CDN.
- **AI Integration**:
  - "The Promoter" agent pre-fills blocks with AI-generated copy and images based on the user's initial onboarding prompt.
  - "The Promoter" automatically generates meta tags, schema markup, and sitemaps for SEO based on the published content.
- **Infrastructure**: Custom domain provisioning and SSL certificate generation are handled via background jobs (e.g., interacting with a DNS provider API) triggered when a user upgrades their tier.

### Architecture Diagram
```mermaid
graph TD
    A[User (Mobile/Desktop)] -->|Drag & Drop / Edit| B(Builder UI Component)
    B -->|Save Draft| C[(page_drafts Table)]
    B -->|Click Publish| D(Publishing Service)
    D -->|Read Draft| C
    D -->|Generate Static Assets| E[Asset Generator]
    E -->|Upload to CDN| F[(CloudFront / Cloudflare)]
    F -->|Serve Live Site| G[Customer]
    D -->|Trigger SEO Update| H(The Promoter Agent)
    H -->|Update Meta/Schema| C
```

### Mobile UX Flow (375px First)
1. **Dashboard**: User taps "Edit Website".
2. **Preview**: The current live site is shown. User taps "Edit" to enter Draft mode.
3. **Block Menu**: A bottom sheet appears with available blocks (e.g., "Add Products", "Add Booking").
4. **Configuration**: Tapping a block opens a native modal (using standard mobile keyboards) to edit text, select products from inventory, or swap images.
5. **AI Assist**: An "AI Sparkle" button is available on text fields to auto-rewrite or generate content.
6. **Publish**: A persistent "Publish" button at the top right deploys changes instantly.

### Key Design Decisions
- **No Free-Form CSS**: To maintain the "Premium" aesthetic (Glassmorphism, correct typography), users cannot alter raw CSS or layout structure. They can only select predefined themes (color palettes, font pairings).
- **Mobile-First Editing**: The editing interface is designed primarily for a 375px screen, prioritizing tap-to-edit and simple block reordering (e.g., using up/down arrows instead of complex drag-and-drop on mobile).

## Implementation Prompt
Implement the Website & Storefront Builder core services and UI components. Develop the backend API to handle saving block-based page drafts and publishing them to a live state. Create the mobile-first (375px) UI for adding, reordering, and configuring predefined content blocks (Hero, Product Grid, Contact). Integrate the "AI Sparkle" feature within text blocks by calling the "The Promoter" agent's generation API. Ensure all components use OHC premium design tokens and support optimistic UI updates. Include E2E tests verifying a user can edit a draft, use the AI copywriter, and successfully publish the changes.

## Priority
P0

## Estimated Scope
Large
