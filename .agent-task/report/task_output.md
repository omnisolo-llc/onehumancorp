# [architecture]_website_storefront_builder_architecture.md

## Title
Website & Storefront Builder Architecture - Drag-and-Drop Editor for Non-Technical Users

## Problem Statement
Small business owners like Maya (baker), Carlos (handyman), and Priya (boutique owner) need professional, highly functional websites and storefronts to operate and grow their businesses. However, existing platforms (Shopify, Wix, Squarespace) require substantial time investment, a baseline level of technical knowledge, or paying a developer. The current "easy" builders still expose concepts like margins, padding, DNS records, and responsive breakpoints, which are overwhelming and lead to analysis paralysis. Our users need a truly "zero-knowledge" website builder that guarantees aesthetic excellence out of the box, handles mobile responsiveness automatically, and integrates seamlessly with AI agents to build the site for them or allow simple block-based editing.

## Research Report
### Competitive Analysis
- **Shopify:** Powerful, but complex theme editor. Users often abandon the DIY route and buy premium themes or hire experts. Assumes desktop-first design.
- **Wix/Squarespace:** Block-based and visually appealing, but users still struggle with structural layout (e.g., placing elements off-grid on Wix). Squarespace enforces grids better but still requires understanding sections vs. blocks.
- **GoDaddy:** Simpler, but outputs generic, visually outdated sites. Customization is heavily restricted.
- **Notion/Linktree:** Very easy to use but lacks the depth required for true e-commerce or booking sites.

### Key Findings
1.  **Templates vs. Blocks:** Users are more successful when they start with a fully fleshed-out template tailored to their business type, rather than a blank canvas.
2.  **Mobile First:** 80%+ of OHC's target users will manage their business via a mobile device. The builder must provide a seamless drag-and-drop or tap-to-add experience on a 375px screen.
3.  **Guardrails are Essential:** Users want their site to "look good" but often break designs by using incompatible colors or unstructured layouts. A rigid, block-based system with a unified design system (Glassmorphism, OHC premium tokens) prevents "ugly" sites.
4.  **AI as the Designer:** The user shouldn't *have* to build the site. "The Promoter" AI agent should generate the initial layout, copy, and images based on a quick onboarding conversation.

## Design Doc

### Architecture Diagram
```mermaid
graph TD
    subgraph Frontend "Flutter UI (Mobile & Web)"
        BE[Builder Engine]
        CP[Content Palette]
        PV[Live Preview Render]
        TM[Theme Manager]
    end

    subgraph Backend "Go + PostgreSQL"
        API[Builder API]
        SM[State Management - pg]
        SE[SEO Engine]
        PUB[Publishing Service]
        DP[Domain Provisioning]
    end

    subgraph AI "Agent Services"
        PR["Marketing & Advertising ('The Promoter')"]
    end

    subgraph Infrastructure
        CDN[Cloudflare / CloudFront]
        OBJ[GCS / MinIO Object Storage]
    end

    CP --> BE
    TM --> BE
    BE <--> PV
    BE -- Save Draft / Publish --> API
    API --> SM
    API --> PUB
    PUB --> SE
    PUB --> DP
    PUB -- Build Static/SSR Assets --> OBJ
    OBJ --> CDN
    PR -- Generate Layout & Content --> BE
    SM -- Load Draft --> BE
```

### UI Wireframes & Mobile UX Flow (375px focus)
1.  **The Canvas:** The screen is primarily the live preview of the site, scaling to the 375px viewport.
2.  **The Action Bar (Bottom):** A persistent bottom sheet or floating action button (FAB) for "Add Block", "Theme", "Settings", and "Publish".
3.  **Adding a Block:** Tapping "Add Block" opens a modal categorized by purpose (e.g., "Sell a Product", "Take a Booking", "Show Testimonials", "Text/Images").
4.  **Editing a Block:** Tapping a block on the canvas opens its specific editor panel (e.g., editing the Hero block opens text fields for Headline, Subheadline, and a button to replace the background image). No margin/padding controls; only content and semantic toggles (e.g., "Align Left/Center").
5.  **Rearranging:** Long-press on a block to drag it up or down the vertical stack.
6.  **AI Generation:** A prominent "AI, do this for me" button in the block selector.

### Key Design Decisions
1.  **Strict Block-Based System:** We will NOT support arbitrary absolute positioning or unstructured grid layouts. Pages are constructed as a vertical stack of pre-defined, semantic blocks (Hero, Product Grid, Booking Calendar, Testimonials, FAQ, Contact Form). This ensures 100% predictable mobile responsiveness and visual consistency.
2.  **Separation of Content and Style:** The underlying data structure stores *what* the block contains (JSON), not *how* it looks. The Theme Engine (applying OHC design tokens) renders the JSON. This allows instantaneous, full-site theme switching without breaking layouts.
3.  **Draft vs. Live State:** The backend maintains distinct `draft_version` and `live_version` records for pages. Auto-save updates the draft. Publishing points the live URL to the current draft state.
4.  **Invisible SEO:** Users do not configure `<meta>` tags manually. The Publishing Service automatically extracts keywords, headings, and descriptions from the content blocks and generates optimized metadata, sitemaps, and structured data (JSON-LD) tailored to the business type.
5.  **Automated Custom Domains & SSL:** When a user upgrades to Starter/Pro, the Domain Provisioning service interacts with DNS providers (via API) to configure CNAME/A records and automatically provisions Let's Encrypt SSL certificates. The user only types "mycoolbakery.com".

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising):**
    - **Initial Generation:** During onboarding, the agent generates the initial full-page layout, copywriting, and selects stock/placeholder images based on the user's business category and brief description.
    - **Block Generation:** When a user adds a new block (e.g., "Testimonial"), they can ask the AI to "write a testimonial from a happy customer who bought a vegan chocolate cake."
    - **SEO Optimization:** Before publishing, the agent analyzes the draft and suggests or automatically rewrites headings for better search visibility.
- **The Advisor (Business Advisory):**
    - Reviews published site analytics and suggests builder actions (e.g., "Your 'Contact Us' block is at the very bottom. Moving it up might increase leads.").

## Implementation Prompt
**Objective:** Implement the backend API and database schema to support the block-based Website Builder, and create the core Flutter UI components for the drag-and-drop mobile editor.

**User Journey (CUJ):**
1. User logs into the OHC app and navigates to the "Website Builder" tab.
2. User sees a live preview of their current draft site (generated by AI during onboarding).
3. User taps "Add Block", selects a "Product Grid" block.
4. User taps the new block and selects 3 products from their existing inventory to display.
5. User drags the "Product Grid" block to be immediately below the "Hero" block.
6. User taps "Publish".
7. The live website immediately reflects the changes.

**Acceptance Criteria:**
- The backend must provide REST/gRPC endpoints for loading drafts, saving drafts (auto-save), and publishing.
- The UI must implement a vertical list of blocks that can be reordered via drag-and-drop on a 375px mobile screen.
- The state of the page (order of blocks and their content payload) must be persisted reliably.
- The "Publish" action must successfully promote the draft state to the live state.
- **Architecture adherence:** The implementation must enforce the strict block-based structure defined in the design doc, preventing arbitrary styling overrides by the user.

## Priority
P0 (Critical)

## Estimated Scope
Large