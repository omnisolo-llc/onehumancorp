# Research Report: Architectural Design for the OHC Website & Storefront Builder

## Problem Statement
Small business owners like Carlos the Handyman or Maya the Baker need a professional web presence to attract customers and accept orders. However, they lack the technical skills to build a website, the budget to hire a developer, and the time to learn complex platforms. Existing website builders are often too generic, require too much configuration, and don't natively integrate with booking, scheduling, and payment systems right out of the box, especially on a mobile device. They just want a beautiful, functional storefront that works seamlessly from their phone.

## Research Findings
Our user personas demand extreme simplicity.
- **Carlos (Handyman)** needs a service listing with pricing, a booking calendar, and an inquiry form.
- **Maya (Baker)** needs a beautiful product catalog with image galleries and a custom order deposit flow.
- **Priya (Boutique)** needs product variants, inventory sync, and a clean checkout.

Currently, if users piece these together, they end up with a fragmented experience (e.g., Carrd for landing page + Calendly for booking + Stripe for payments). OHC must unify this into a single cohesive builder that defaults to "live" status very quickly.

### Competitive Analysis
- **Shopify**: Excellent for e-commerce but overwhelming for services (Carlos, Leo) or simple food pre-orders (Fatima). High learning curve; mobile app is mostly for management, not initial setup.
- **Wix**: Very flexible, but too complex ("blank canvas syndrome"). Not truly mobile-first for the *builder* experience.
- **Squarespace**: Beautiful templates, but rigid. Requires desktop to build effectively.
- **GoDaddy**: Fast setup but very limited customization and poor aesthetic defaults.
- **OHC**: Differentiates by offering a mobile-first, component-based builder that is heavily AI-assisted. The "Marketing & Advertising" agent handles the initial generation, and the user simply tweaks content blocks on a 375px screen.

### Data & References
- Studies show SMBs abandon website creation if the process takes more than 1 hour.
- Over 70% of local searches are performed on mobile devices, meaning the resulting site MUST be mobile-first. OHC's builder mandates a 375px-first approach.

## Architectural Design

### System Overview

```mermaid
graph TD
    subgraph Frontend "Tauri App (Mobile-First)"
        UI[Builder UI]
        Preview[Live Preview 375px]
        State[App State]
    end

    subgraph Backend "Rust + Bazel Backend"
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
1. **AI Generation Step:** User inputs basic info. "Marketing & Advertising" agent generates a functional preview within 10 seconds.
2. **Editor View (375px):** Shows the live preview. A bottom sheet allows adding "Blocks". Tapping any element opens an inline editor using the native mobile keyboard.
3. **Block Library:** Scrollable list of semantic blocks: Hero, Product Grid, Service List, Booking Calendar, Contact Form, Testimonials.
4. **Publishing Flow:** Big "Publish" button triggers the workflow, provisioning an OHC subdomain or mapping a custom domain.

### Mobile UX Constraints
- Operable with one thumb.
- "Move Up/Down" arrows instead of drag-and-drop.
- Touch targets ≥ 44x44px.
- Glassmorphism UI tokens used throughout the editor panels.

### AI Agent Integration Points
- **The Promoter (Marketing & Advertising):** Initializes the draft website, generates SEO meta tags, and suggests alt text for images.
- **The Advisor (Business Advisory):** Scans the live site and suggests conversion improvements.

### Key Decisions
- **Block-Based, Not Pixel-Perfect:** Prevents users from breaking the mobile layout and ensures accessibility.
- **AI-First Initialization:** No blank pages to solve "blank canvas syndrome".
- **Integrated SEO:** The AI handles meta tags entirely.
- **Automated SSL & CDN:** Security is automatic, even for free tiers.
