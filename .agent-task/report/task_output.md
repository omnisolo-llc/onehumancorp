# [Architecture] Unified Social Micro-Storefront Engine

## Title
Unified Social Micro-Storefront Engine

## Problem Statement
For mobile-first solopreneurs and creators like Leo (music tutor) and Maya (baker), traditional e-commerce storefronts are too complex and fragmented. Their customers discover them via social media (Instagram, TikTok) and expect a frictionless, instant purchasing or booking experience without leaving the social app browser. Currently, redirecting users to a traditional, heavy website causes massive drop-offs due to slow loading, complex navigation, and broken mobile UX. They need a zero-config, hyper-optimized "link-in-bio" micro-storefront that loads instantly on mobile, supports 1-tap checkout/booking (Apple Pay/Google Pay), and feels native to the social platform.

## Research Report
**Market Analysis & Competitive Landscape:**
- **Linktree & Beacons:** Dominate the "link-in-bio" space but are fundamentally link aggregators. Their commerce features are bolted-on, lacking deep inventory or unified booking integration.
- **Shopify Starter:** Offers social selling but is still rooted in a complex, multi-page e-commerce architecture. The in-app browser experience can be heavy.
- **Stan Store:** Popular among creators for its 1-tap digital downloads and calendar bookings directly in the link-in-bio, but lacks robust physical product capabilities, deep multi-tenant POS integration, and conversational AI automation.
- **User Pain Points:** Social media browsers (Instagram/TikTok in-app browsers) have strict limitations (e.g., restricted cookies, poor performance). A heavy traditional PWA or Next.js site will fail here. The conversion rate drops by 20% for every second of load time.

**Findings:**
OHC requires an edge-cached, dynamically generated micro-storefront architecture tailored for 375px viewports. It must unify digital downloads, service bookings, and physical products into a single, scrollable, native-feeling feed.

## Design Doc

### Architecture Diagram
```mermaid
graph TD;
    SocialApp[Instagram/TikTok In-App Browser] --> EdgeCDN[Edge Delivery Network / CDN Cache];
    EdgeCDN --> MicroStorefrontUI[Pre-rendered Mobile Storefront 375px];

    MicroStorefrontUI --> API[Rust Server / Multi-tenant API];

    API --> BookingEngine[Unified Booking & Deposit Engine];
    API --> ProductCatalog[Digital / Physical Catalog];
    API --> CheckoutEngine[1-Tap Checkout Apple/Google Pay];

    API --> OHC_Agents[AI Department: Sales & Support];
    API --> Postgres[(Postgres Shared Ledger)];
    API --> Redis[(Redis Cache)];

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class SocialApp,EdgeCDN,MicroStorefrontUI,API,BookingEngine,ProductCatalog,CheckoutEngine,OHC_Agents,Postgres,Redis premium;
```

### UI Wireframes & Mobile UX Flow
**Viewport Target:** 375px (Mobile-First)

**Screen 1: The Micro-Storefront (Link-in-bio landing)**
- **Header:** Profile Photo, Name, Verified Badge, 1-sentence AI-generated bio.
- **Glassmorphism Design:** Subtle 20px blur background matching the creator's brand colors.
- **Content Feed (Scrollable vertically):**
  - Card 1: "Book a Lesson" (Shows next available time slot, tap to expand calendar).
  - Card 2: "Order Custom Cake" (Image gallery, $50 deposit button).
  - Card 3: "Download Digital Guide" (Instant buy with Apple Pay).
- **Floating Action Button:** Persistent "Chat with AI Assistant" for instant Q&A.

**Screen 2: 1-Tap In-App Checkout**
- Triggered instantly via a bottom-sheet modal.
- Native mobile pay integrations (Apple/Google Pay) bypass manual form entry.
- Success animation transitions to a digital receipt / calendar invite screen.

### AI Agent Integration Points
- **AI Storefront Generator:** Onboards the merchant by analyzing their Instagram handle to automatically generate the bio, color scheme, and initial product cards.
- **Conversational Sales Agent:** Acts as a proactive chat bubble on the storefront, answering questions like "Do you have vegan options?" and providing deep links to checkout.
- **Marketing Agent:** Auto-generates shareable Instagram Stories/Reels highlighting specific micro-storefront cards.

### Key Design Decisions
1. **Edge-First Delivery:** The micro-storefront HTML/CSS must be heavily cached at the edge (CDN) to ensure <500ms load times inside social media in-app browsers.
2. **Bottom-Sheet Modals:** Overriding page navigations with bottom-sheet modals keeps the user in the context of the social feed, reducing bounce rates.
3. **Unified Entity Representation:** Treat bookings, physical products, and digital downloads as polymorphism on a single "Shoppable Card" UI component to keep the interface simple.
4. **Zero-Trust Multi-Tenancy:** Each request through the micro-storefront API is strictly isolated by the `organization_id` resolved from the unique subdomain or link path.

## Implementation Prompt
**For Implementer Agents:**
Implement the Unified Social Micro-Storefront Engine. Focus on creating the data models, API endpoints, and mobile-first UI components for a link-in-bio style storefront.
1. **User Journey (CUJ):** Maya the baker needs to share a single link on her Instagram bio. When a customer taps it, they instantly see a mobile-optimized, fast-loading list of her custom cakes. The customer taps a cake, sees a bottom-sheet modal, and pays a deposit via 1-tap checkout.
2. **Acceptance Criteria:**
   - The UI must be optimized exclusively for 375px viewports (mobile-first). Apply macOS-style Translucent Glass materials.
   - The storefront must load dynamically based on the tenant's configuration.
   - Create unified components that can handle physical products, digital downloads, and bookings within the same visual feed.
   - Integrate a floating "Chat" button connected to the AI Sales Agent.
   - Ensure the architecture supports heavy edge caching (avoid reliance on heavy client-side JS rendering).

## Priority
P0

## Estimated Scope
Large
