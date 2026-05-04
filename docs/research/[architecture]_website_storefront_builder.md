# [Architecture] Website & Storefront Builder

## Title
Architectural Design for the OHC Website & Storefront Builder

## Problem Statement
Small business owners like Carlos the Handyman or Maya the Baker need a professional web presence to attract customers and accept orders. However, they lack the technical skills to build a website, the budget to hire a developer, and the time to learn complex platforms. Existing website builders are often too generic, require too much configuration, and don't natively integrate with booking, scheduling, and payment systems right out of the box, especially on a mobile device. They just want a beautiful, functional storefront that works seamlessly from their phone.

## Research Report

**Findings:**
Our user personas demand extreme simplicity.
- Carlos (Handyman) needs a service listing with pricing, a booking calendar, and an inquiry form.
- Maya (Baker) needs a beautiful product catalog with image galleries and a custom order deposit flow.
- Priya (Boutique) needs product variants, inventory sync, and a clean checkout.

Currently, if users piece these together, they end up with a fragmented experience (e.g., Carrd for landing page + Calendly for booking + Stripe for payments). OHC must unify this into a single cohesive builder that defaults to "live" status very quickly.

**Competitive Analysis:**
- **Shopify**: Excellent for e-commerce but overwhelming for services (Carlos, Leo) or simple food pre-orders (Fatima). High learning curve; mobile app is mostly for management, not initial setup.
- **Wix**: Very flexible, but too complex ("blank canvas syndrome"). Not truly mobile-first for the *builder* experience.
- **Squarespace**: Beautiful templates, but rigid. Requires desktop to build effectively.
- **GoDaddy**: Fast setup but very limited customization and poor aesthetic defaults.
- **OHC**: Differentiates by offering a mobile-first, component-based builder that is heavily AI-assisted. The "Marketing & Advertising" agent handles the initial generation, and the user simply tweaks content blocks on a 375px screen.

**Data & References:**
- Studies show SMBs abandon website creation if the process takes more than 1 hour.
- Over 70% of local searches are performed on mobile devices, meaning the resulting site MUST be mobile-first. OHC's builder mandates a 375px-first approach.

## Design Doc

### Architecture Diagram

```mermaid
graph TD
    subgraph Frontend "Flutter App (Mobile-First)"
        UI[Builder UI]
        Preview[Live Preview 375px]
        State[Riverpod State]
    end

    subgraph Backend "Go + Bazel Backend"
        API[Builder API gRPC/REST]
        ThemeEngine[Theme & Rendering Engine]
        PubSub[Publishing Workflow Engine]
        DomainMgr[Domain & SSL Provisioning]
    end

    subgraph AI "AI Agent Departments"
        Marketing[Marketing & Advertising Agent]
    end

    subgraph Storage
        DB[(PostgreSQL - Tenant DB)]
        CDN[Cloudflare / CDN]
        StorageGCS[GCS/MinIO Object Storage]
    end

    UI --> State
    State <--> API
    UI --> Preview

    API --> ThemeEngine
    API --> PubSub
    API --> DomainMgr

    ThemeEngine --> DB
    PubSub --> DB
    PubSub --> CDN

    Marketing -.-> API : Auto-generates initial layout

    DomainMgr --> CDN : Maps custom domains & provisions SSL
```

### UI Wireframes & Screen Flow (375px First)

1. **AI Generation Step:**
   - User inputs basic info (Name, Industry, Goal).
   - "Marketing & Advertising" AI agent generates a functional preview within 10 seconds.
2. **Editor View (375px):**
   - The screen shows the live 375px preview of the site.
   - At the bottom, a floating action button (FAB) or bottom sheet allows adding "Blocks".
   - Tapping any element (text, image) opens an inline editor using the native mobile keyboard.
3. **Block Library:**
   - A scrollable list of pre-configured blocks: Hero, Product Grid (linked to inventory), Service List, Booking Calendar (linked to Leo/Carlos's availability), Contact Form, Testimonials.
4. **Publishing Flow:**
   - Big "Publish" button at the top right.
   - Triggers the Publishing Workflow (draft -> live).
   - Prompts for Domain (free OHC subdomain or connect custom domain).

### Mobile UX Flow
- The entire builder is designed to be operated with one thumb.
- Drag-and-drop is replaced with "Move Up/Down" arrows or accessible reorder handles suitable for touch targets (≥ 44x44px).
- Image uploads go straight from the phone's camera roll or direct camera capture, immediately compressed to WebP.
- Glassmorphism UI tokens used throughout the editor panels (backdrop-filter: blur(20px)).

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising)**:
  - Initializes the draft website based on onboarding inputs.
  - Generates SEO meta tags (title, description, structured data) automatically based on page content.
  - Suggests alt text for uploaded images.
- **The Advisor (Business Advisory)**:
  - Scans the live site and suggests improvements (e.g., "Add a booking block to your home page to increase conversions").

### Key Design Decisions and Why
- **Block-Based, Not Pixel-Perfect**: Users cannot freely position elements. They stack semantic blocks. *Why?* Prevents users from breaking the mobile layout. Ensures accessibility and performance.
- **AI-First Initialization**: The user never sees a blank page. *Why?* Solves "blank canvas syndrome".
- **Integrated SEO**: No "SEO Settings" tab for the user. *Why?* Non-technical users don't know what meta tags are. The AI handles it entirely.
- **Automated SSL & CDN**: Every site, even on the free subdomain, gets instant SSL via the CDN provider. Custom domains map directly to the CDN. *Why?* Security is non-negotiable, and users shouldn't know what SSL is.

## Implementation Prompt

**Outcome:**
Build the "Website & Storefront Builder" feature. A non-technical user must be able to open the app, have an AI generate a starting layout, modify text and images, add a booking or product block, and publish the site to a live URL, entirely from a 375px mobile screen.

**Critical User Journeys (CUJ):**
1. User clicks "Edit Website". The AI generates a draft if one doesn't exist.
2. User taps a text block and edits it using the native keyboard.
3. User adds a "Product Grid" block, which automatically populates with their existing inventory.
4. User clicks "Publish". The system provisions an OHC subdomain (e.g., `maya-cakes.ohc.app`) and makes the site live.

**Acceptance Criteria:**
- The builder UI must render perfectly on a 375px width screen.
- Touch targets for editing and reordering blocks must be at least 44x44px.
- The UI must use the OHC Glassmorphism design tokens.
- Publishing must be instantaneous, moving the state from draft to live.
- The resulting live site must be accessible and responsive.
- AI must automatically generate SEO metadata when the site is published.

## Priority
P0

## Estimated Scope
Large