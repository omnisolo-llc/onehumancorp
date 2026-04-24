# Website & Storefront Builder Architecture

## 1. Problem Statement
Many small business owners (like Maya the baker, Carlos the handyman, and Fatima the food cart operator) lack the technical skills to build a functional, beautiful website. Existing tools like Shopify, Wix, and Squarespace present a steep learning curve, requiring hours of setup, theme configuration, and content writing. OHC needs a zero-friction, drag-and-drop website builder that is usable natively on a mobile phone (375px width) and leverages AI to do the heavy lifting. The system must allow users to go from idea to a live, beautiful storefront in under 10 minutes.

## 2. Research & Competitive Analysis
- **Shopify**: Excellent for pure e-commerce but overwhelming for service businesses or pure portfolios. Setup takes 30-60 minutes minimum. Not natively mobile-first for management.
- **Wix/Squarespace**: Flexible but complex drag-and-drop interfaces that do not translate well to mobile creation. Require desktop access for serious editing.
- **GoDaddy/Zyro**: Faster setup with AI generation, but limited customization and rigid templates.

**OHC's Unfair Advantage**: The OHC builder is entirely mobile-first. AI agents generate the initial structure, copy, and images. The user only needs to perform minor tweaks using a simplified block-based editor. All output adheres to the OHC Premium Token library (Glassmorphism, Outfit/Inter typography).

## 3. Design Doc

### 3.1 Content Blocks & Layout
The builder uses a rigid block system to guarantee responsive design and aesthetic excellence.
- **Hero Block**: Main image/video, headline, subheadline, primary CTA.
- **Product Grid**: Syncs automatically with the inventory/service catalog.
- **Text & Media Block**: For "About Us" or custom content.
- **Testimonials Block**: Auto-populated by the Customer Success agent.
- **Booking Calendar Block**: For service businesses (syncs with Operations agent).
- **Contact Form Block**: Routes inquiries to the Sales/Customer Success agents.

### 3.2 Mobile UX Flow
1. **Onboarding**: AI asks 3 questions (Business name, industry, main goal).
2. **Generation**: "The Promoter" (Marketing Agent) generates a complete multi-page site.
3. **Editing (375px native)**: User taps any block to edit. The interface slides up a native bottom sheet with limited, curated options (e.g., change image, rewrite text via AI, swap layout variant).
4. **Publishing**: 1-tap publish. The system handles DNS provisioning or subdomains automatically.

### 3.3 Architecture Diagram

```mermaid
sequenceDiagram
    participant User
    participant App as Mobile App (Flutter)
    participant API as OHC API
    participant Agent as Marketing Agent
    participant CDN as CloudFront CDN
    participant DB as OHC-SIP DB

    User->>App: Submits onboarding answers
    App->>API: Generate Site Draft
    API->>Agent: Prompt: Generate site structure & copy
    Agent-->>API: JSONB Site Definition
    API->>DB: Save Site Draft (tenant_id)
    API-->>App: Render Preview
    User->>App: Tap "Publish"
    App->>API: Publish Site
    API->>CDN: Invalidate Cache / Deploy Static Assets
    API->>DB: Update Status to LIVE
    API-->>App: Success & Shareable Link

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class User,App,API,Agent,CDN,DB premium;
```

### 3.4 Key Design Decisions
- **JSONB Representation**: The entire site structure is stored as a JSONB payload in PostgreSQL. This allows the AI to easily parse and modify the structure without complex relational schema migrations.
- **No Custom CSS**: To maintain the "Aesthetic Excellence" core value, users cannot inject custom CSS. All styling is controlled by theme toggles that map to the OHC Premium Token library.
- **Automated SEO**: The Marketing Agent automatically generates meta tags, sitemaps, and alt text for all images. The user is never exposed to SEO jargon.
- **Subdomains by Default**: Free tier users get an `[name].ohc.app` subdomain immediately. Pro users can connect custom domains with automated Let's Encrypt SSL provisioning.

## 4. Implementation Prompt
Implement the backend foundation for the Website Builder. Create the API endpoints and database migrations to store and retrieve the JSONB site definition for a tenant. Implement the AI generation trigger where "The Promoter" agent receives basic business details and returns a fully populated JSONB site structure using the defined content blocks. Ensure all interactions are scoped by `tenant_id`.

## 5. Metadata
- **Priority**: P0
- **Estimated Scope**: Large
