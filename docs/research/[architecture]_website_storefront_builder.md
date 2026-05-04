# Website & Storefront Builder Architecture

## Problem Statement

Small business owners—like Maya the home baker or Carlos the freelance handyman—want a professional online presence to sell their products and services, but building a website from scratch is intimidating. Existing tools like Shopify or Wix are too complex for non-technical users and often require a desktop to manage effectively. They demand technical decisions (hosting, SSL, DNS, SEO) that overwhelm everyday people.

Business owners need a drag-and-drop website builder that works flawlessly on a 375px mobile screen, acts as an all-in-one storefront, portfolio, and booking system, and lets the AI invisibly handle all the technical complexity. They want to go from zero to a published, beautiful site in under 10 minutes from their phone.

## Research Report

**Market Findings & Competitive Analysis:**
- **Shopify**: Highly robust for e-commerce but overwhelming for service businesses or pure portfolios. Mobile app is mainly for management, not design. Setup time: 30-60 min.
- **Wix / Squarespace**: Excellent drag-and-drop features but incredibly difficult to use on a mobile device. Geared toward semi-technical users or creative professionals. Requires heavy manual SEO tweaking.
- **GoDaddy**: Fast setup, but templates are generic and inflexible. Doesn't offer a truly dynamic AI agent experience to constantly optimize the site.
- **Pain Points**:
  - Non-technical users struggle with organizing content, writing copy, and handling SEO.
  - Mobile website builders are generally clunky, leading users to abandon the process.
  - Provisioning custom domains and SSL certificates is a major source of friction.

**Opportunity for OHC**: A completely mobile-first (375px baseline) site builder where AI generates the initial site based on a brief interview, provides high-conversion content blocks, auto-configures SEO, and seamlessly provisions subdomains or custom domains.

## Design Doc

### Architecture Diagram

```mermaid
flowchart TD
    subgraph Mobile Interface (375px First)
        A[Site Builder UI]
        B[AI Assistant / The Promoter]
        A <--> B
    end

    subgraph Application Layer
        C[Site Manager API]
        D[AI Content Generator]
        E[Publishing Engine]
        C --> D
        C --> E
    end

    subgraph Core Entities
        F[(Site Drafts & Templates)]
        G[(Live Site Configuration)]
        F --> G
    end

    subgraph External
        H[Domain Provisioning]
        I[SSL Provisioning]
        E --> H
        E --> I
    end

    A <--> C
```

### UI Wireframes & Screen Flow (375px First)
1. **Onboarding / Template Selection**: The AI asks 3 questions ("What is your business?", "What is your style?", "What are you selling?"). The system generates a personalized, glassmorphism-styled preview.
2. **Editor View**: A scrollable preview of the site with an "Add Block" floating action button (FAB).
3. **Block Selection Modal**: User chooses from intuitive block types (e.g., Hero, Product Grid, Service List, Testimonials, Booking Calendar, Contact Form).
4. **Block Editor**: Native mobile keyboard support to edit text, upload images (auto-compressed), and toggle settings (e.g., "Show Sold Out").
5. **Publishing Flow**: A single "Publish" button. The system displays a loading animation while it finalizes SEO, custom domain/subdomain linkage, and SSL generation, leading to a "Your Site is Live!" success screen.

### Content Blocks
- **Hero**: Eye-catching image/video with a title, subtitle, and primary CTA.
- **Product Grid**: Dynamic list syncing with inventory; supports variants.
- **Service List / Booking Calendar**: Displays services with prices and an integrated time-slot picker.
- **Text & Media**: For "About Us" or story sections.
- **Testimonials**: Auto-pulled from customer success tools or manually added.
- **Contact Form**: Directs inquiries to the shared inbox.

### Mobile UX Flow
- The entire builder operates via drag-and-drop reordering (long-press to move) and tapping to edit inline. No sidebars; properties are edited in bottom-sheet modals.
- Real-time preview is the actual editing surface.

### AI Agent Integration Points
- **The Promoter**: Generates the initial site design, writes compelling copy, suggests block additions, and continually optimizes on-page SEO meta tags invisibly.
- **The Manager**: Syncs the Product Grid block and Booking Calendar block seamlessly with live operations data.

### Key Design Decisions
- **Mobile-First Editing**: All site management occurs directly on the phone, leveraging native keyboards and touch interactions (bottom sheets, long-press).
- **Invisible SEO**: Users never manually input meta tags. The AI extracts keywords from the business description and block content to populate SEO metadata.
- **Draft to Live Separation**: Edits are auto-saved to a draft state. A specific "Publish" action pushes changes live, ensuring visitors never see half-finished work.
- **Automated Infrastructure**: Custom domains and SSL certificates are provisioned in the background during the publish action without user intervention.

## Implementation Prompt

**Task**: Implement the Website & Storefront Builder.

**User-Facing Outcome**:
As a small business owner, I can open the mobile app, answer a few questions, and have an AI generate a beautiful website for my business. I can easily add content blocks (Hero, Products, Booking Calendar) by tapping on my phone, edit the text, and hit "Publish." My site becomes instantly live with automatically configured SEO and SSL.

**Critical User Journeys (CUJ)**:
1. User logs in, selects "Create Website," and answers the AI's prompts.
2. User is presented with an editable draft of their site.
3. User adds a "Product Grid" block, and edits the "Hero" text using their mobile keyboard.
4. User taps "Publish." The app displays a progress screen, then confirms the site is live on a custom or OHC subdomain, fully secured.

**Acceptance Criteria**:
- The UI must perfectly fit a 375px mobile screen (no horizontal scrolling).
- Editing must occur via tap-to-edit inline or bottom-sheet overlays.
- A clear separation must exist between the "Draft" state and the "Live" state.
- Adding blocks (Hero, Product Grid, Booking Calendar, etc.) must reflect dynamically in the preview.
- Upon publishing, SEO meta tags must be auto-generated by the AI, and domain/SSL must be automatically configured (mock external provisioning for now).
- 100% E2E test coverage in Playwright/slint matching the expected business flows.

## Priority
`P0` (Critical - Core to the platform value proposition)

## Estimated Scope
Large
