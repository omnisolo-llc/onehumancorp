# Issue Brief: Website & Storefront Builder Architecture

## Title
Website & Storefront Builder Architecture: A Mobile-First, AI-Driven Drag-and-Drop Experience

## Problem Statement
Small business owners—from bakers to handymen—need a professional online presence to sell products, book services, and build trust. However, existing website builders (like Shopify, Wix, and Squarespace) are often too complex, requiring manual layout adjustments, SEO configuration, and technical domain setup. Our users need a "zero-to-live" experience where they can launch a beautiful, fully functional storefront in under 10 minutes directly from their mobile phone, with AI handling the design, content, and publishing complexity invisibly.

## Research Report
- **Goal**: Design a website and storefront builder architecture that guarantees a mobile-first, grandmother-friendly experience, powered by autonomous AI.
- **Findings & Competitive Analysis**:
  - **Shopify**: Excellent e-commerce capabilities but the storefront builder can be overwhelming on mobile. Themes often require desktop intervention for deep customization. (Payload Size proxy: ~421KB)
  - **Wix**: Highly customizable desktop drag-and-drop, but mobile translation is often clunky. Too many choices for non-technical users. (Payload Size proxy: ~212KB)
  - **Squarespace**: Beautiful templates, but rigid. Requires significant time investment to curate content and structure. (Payload Size proxy: ~162KB)
  - **OHC Advantage**: OHC will use AI to automatically generate content blocks (hero, products, booking, testimonials) based on the business type and persona (e.g., Carlos the handyman gets a service listing and booking calendar; Maya the baker gets a visual catalog). The user only makes high-level choices (e.g., "make it more playful") while AI handles the layout, SEO, and responsive design. Constrain the design space to predefined, highly polished blocks that auto-arrange beautifully on any device.

## Design Doc

### Architecture Diagram
```mermaid
graph TD
    A[Mobile App UI] -->|Selects Theme & Intent| B(AI Layout Generator)
    B -->|Assembles| C{Content Blocks}
    C --> D[Hero Block]
    C --> E[Product/Service Grid]
    C --> F[Booking/Contact Form]
    C --> G[Testimonials]
    B -->|Applies| H(Design Tokens)
    H -->|Preview| I[Draft Storefront]
    I -->|Publish| J[Live Site CDN]
    I -->|Auto-Configures| K[SEO & Custom Domain]
```

### UI Wireframes & Screen Flow (Mobile 375px)
1. **Onboarding Intent**: User selects business type (e.g., "Food Cart") and primary goal (e.g., "Take Pre-orders").
2. **AI Generation**: A subtle loading state showing "Our AI is building your menu and storefront..."
3. **Editor Preview**: A live preview of the generated site. At the bottom, a contextual action bar:
   - "Change Theme" (Swipes through high-quality visual styles)
   - "Edit Text" (Opens a simple text input overlay)
   - "Manage Products/Services" (Deep links to the catalog manager)
4. **Publishing**: A single "Go Live" button. The app handles domain provisioning (OHC subdomain or custom domain based on tier) and SSL invisibly.

### Key Design Decisions
- **Mobile-First Editor**: The editor is not a free-form drag-and-drop canvas. It is a block-based assembler. Users add, remove, or reorder strictly defined, highly polished blocks.
- **AI-Driven Content**: The Marketing & Advertising AI department automatically writes SEO-optimized copy, generates placeholder images (or enhances uploaded ones), and suggests layout improvements based on seasonal trends.
- **Zero-Configuration Publishing**: Publishing a site requires zero knowledge of DNS, CDN, or SSL. The platform abstracts all infrastructure.
- **Persona-Specific Defaults**:
  - **Maya (Baker)**: Defaults to a visual catalog block and a custom order deposit form.
  - **Carlos (Handyman)**: Defaults to a service list block and an availability calendar block.
  - **Priya (Boutique)**: Defaults to an inventory-synced product grid.

## Implementation Prompt
Implement the Website & Storefront Builder feature. Focus on delivering a block-based layout engine accessible entirely via mobile. Ensure the AI integration can assemble complete pages based on user intent and business type. The user should be able to preview and publish the site with a single tap, with the platform handling all underlying infrastructure (hosting, SEO metadata, domains). Ensure all design tokens follow the visual excellence mandate.

## Priority
P0

## Estimated Scope
Large
