# Issue Brief: Website & Storefront Builder Architecture

### Title
Drag-and-Drop Website & Storefront Builder Architecture for OHC

### Problem Statement
Small business owners, such as bakers, handymen, and boutique owners, often lack the technical expertise to build and manage a website or online storefront. Traditional website builders like Shopify, Wix, and Squarespace still present a learning curve with their multitude of options, settings, and somewhat complex interfaces. Our non-technical users need a radically simple, mobile-first builder that abstracts away technical details (like hosting, SEO, and SSL) and allows them to launch a beautiful, functional storefront in under 10 minutes using intuitive drag-and-drop mechanics.

### Research Report
A successful website builder for our target audience must focus on simplicity, speed, and aesthetic excellence by default. It must support both physical/digital goods and service bookings.

**Market Context & Competitor Analysis:**
Our primary competitors target slightly more tech-savvy users or require more time investment. OHC's unique value proposition is the combination of extreme simplicity (zero technical knowledge), mobile-first management, and AI doing the heavy lifting.

**Competitive Comparison:**

| Feature | OHC (Target) | Shopify | Wix | Squarespace | GoDaddy |
|---|---|---|---|---|---|
| Target User | **Non-technical (Zero knowledge)** | SMB/Tech-savvy | Semi-technical | Creative professional | Basic user |
| Setup Time | **< 10 min** | 30-60 min | 20-40 min | 30-60 min | 20-40 min |
| Management Device | **Mobile-first (100% functionality)** | Desktop primary, mobile app limited | Desktop primary, mobile app limited | Desktop primary | Desktop primary |
| AI Integration | **Deep (Handles design, SEO, copy invisibly)** | Chatbot (Sidekick) | Generative (Wix AI) | Limited | Basic (Airo) |
| Architecture | **All-in-one (Store + Bookings + Portfolio)** | Store-focused | Complex all-in-one | Portfolio + Store | Basic all-in-one |
| Customization | **Curated blocks, premium defaults** | High complexity | High complexity | Moderate complexity | Low complexity |

**Quadrant Chart: Market Positioning**

```mermaid
quadrantChart
    title Market Positioning: Ease of Use vs. Mobile Management
    x-axis Low Mobile Functionality --> High Mobile Functionality
    y-axis High Complexity --> Low Complexity
    quadrant-1 Ideal Target (OHC)
    quadrant-2 Simple but Desktop Focused
    quadrant-3 Complex & Desktop Focused
    quadrant-4 Powerful Mobile App
    "Shopify": [0.3, 0.4]
    "Wix": [0.4, 0.3]
    "Squarespace": [0.2, 0.6]
    "GoDaddy": [0.5, 0.7]
    "OHC": [0.9, 0.9]
```

### Design Doc

**Overview:**
The Website & Storefront Builder will utilize a block-based architecture. Users assemble pages by stacking predefined, fully responsive content blocks. The system prevents users from breaking the design by enforcing design tokens (Glassmorphism, Outfit/Inter typography, specific spacing constraints). AI ("The Promoter") assists in generating initial content and optimizing for SEO.

**Core Content Blocks:**
1.  **Hero Block:** Large image/video background, headline, tagline (AI-generated option), primary Call to Action (CTA).
2.  **Product Grid:** Displays physical/digital products. Syncs automatically with inventory.
3.  **Service/Booking Block:** Integration with the booking calendar. Displays available time slots.
4.  **Text & Image Block:** For about us, policies, or general information.
5.  **Testimonial Block:** Displays customer reviews.
6.  **Contact Form Block:** Simple form for inquiries.

**Key Mechanics:**
*   **Templates:** Users start by selecting an AI-recommended template based on their business type. Templates are pre-configured combinations of blocks and theme settings.
*   **Publishing Flow:** Users work in a "Draft" state. Clicking "Publish" triggers a background job to compile the site, optimize assets, and deploy to the CDN.
*   **SEO Automation:** "The Promoter" agent automatically generates meta titles, descriptions, and alt text for images based on the content blocks.
*   **Infrastructure (Abstracted):** Custom domains are provisioned automatically via a background service (e.g., integrating with a provider API). SSL certificates are provisioned and renewed automatically (e.g., Let's Encrypt integration). The user simply enters the domain they own or want to buy.

**Architecture Diagram:**

```mermaid
graph TD
    subgraph Frontend Builder (Flutter/PWA)
        UI[Mobile-First UI 375px] --> BlockEditor[Block Editor Drag & Drop]
        BlockEditor --> Preview[Live Preview Engine]
        BlockEditor --> ThemeConfig[Theme Configuration Tokens]
    end

    subgraph Backend API (Go/Bazel)
        API[Builder API Gateway] --> PageService[Page & Block Management]
        API --> PublishService[Publishing Workflow]
        API --> DomainService[Domain & SSL Management]
    end

    subgraph AI Agents
        Promoter[The Promoter Agent] --> SEO[SEO Optimization]
        Promoter --> CopyGen[Copy Generation]
    end

    subgraph Infrastructure
        PublishService --> Storage[GCS/MinIO Storage]
        Storage --> CDN[CDN Cloudflare/CloudFront]
        DomainService --> Registrar[External Registrar API]
    end

    UI -- REST/JSON --> API
    PageService -- CRUD --> DB[(PostgreSQL)]
    PublishService -- Trigger --> Storage
    BlockEditor -. Request Copy .-> Promoter
```

### Implementation Prompt
Implement the backend and frontend foundation for the block-based Website Builder.
1.  **Backend:** Create the database schema and API endpoints to support saving a page composed of an ordered list of blocks. A block should have a type (e.g., 'hero', 'product_grid') and a JSON payload for its specific configuration.
2.  **Frontend:** Develop the mobile-first (375px) Flutter UI for the block editor. Implement the drag-and-drop reordering functionality for at least three basic block types: Hero, Text, and Product Grid. Ensure the UI enforces the OHC design system (Glassmorphism, specific typography).
3.  **Acceptance Criteria:** A user can create a new page, add a Hero block and a Text block, reorder them via drag-and-drop on a mobile-sized screen, and save the page successfully to the backend. The saved structure must accurately reflect the visual order.

### Priority
P0

### Estimated Scope
Large
