issue_title: "[Architecture] Invisible Magic Catalog - Zero-Friction Inventory"
issue_description: |
  # Architecture Brief: Invisible Magic Catalog - Autonomous Zero-Click Catalog Generation via AI Agents

  ## Problem Statement
  For OneHumanCorp (OHC)'s core personas—especially **Maya (baker, 28)** and **Priya (boutique owner, 35)**—adding new inventory to an online store is historically one of the most agonizing, high-friction points of running a business. They must take photos, crop them, write compelling product descriptions, decide on pricing, configure variants (size/color), and manage SEO metadata. This creates a massive "Content Creation Block" (identified in our top 10 SMB pain points).
  Currently, competitors like Shopify offer "AI writing assistants" that require prompting and editing. We need a solution that feels like magic: an invisible "Teammate" that takes a raw photo sent from a phone and completely creates the live product listing autonomously, reducing the setup time from hours to seconds.

  ## Research Report
  ### Competitive Landscape
  *   **Shopify:** Offers "Magic" text generation. Still requires manual photo uploading, cropping, and form-filling. High friction on mobile.
  *   **Wix:** Basic ADI (Artificial Design Intelligence) for setup, but adding inventory remains a traditional manual form.
  *   **GoDaddy:** Basic branding generation, no autonomous catalog entry.

  ### Market Data
  *   **73%** of 1-star reviews for legacy platforms cite overwhelming setup and complex menus.
  *   **Solopreneurs** lose hours per week on data entry and inventory management.
  *   The **"Generative Engine Optimization (GEO)"** trend means we need structured, rich, AI-friendly data for products to be discovered organically, which non-technical users cannot generate themselves.

  ### Opportunity
  We leapfrog competitors by fully embracing the "Teammate" philosophy. By completely eliminating the inventory data entry form and replacing it with an autonomous AI pipeline triggered by an image upload, we hit our "zero -> live in under 10 minutes" mandate and solve the biggest bottleneck to retention and monetization.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant User as Maya (Mobile 375px)
      participant Edge as Edge Gateway
      participant EventMesh as NATS Event Mesh
      participant VisionAgent as The Visualizer (Vision AI)
      participant WriterAgent as The Promoter (Marketing AI)
      participant ManagerAgent as The Vigilant Manager (Ops AI)
      participant ActionFeed as OHC Dashboard Feed
      participant Storefront as Edge-Cached Storefront

      User->>Edge: Uploads raw image of custom cake
      Edge->>EventMesh: Publish `image.uploaded` event
      EventMesh->>VisionAgent: Trigger Vision Analysis
      VisionAgent-->>WriterAgent: Pass detected object, variants, estimated price
      WriterAgent-->>ManagerAgent: Drafts description, SEO tags, structured data
      ManagerAgent->>ActionFeed: Queue action: "Approve New Cake Listing"
      User->>ActionFeed: 1-Tap "Approve & Publish"
      ActionFeed->>Storefront: Publish to live catalog & Sync Tap-to-Pay POS
  ```

  ### UI Wireframes (375px Mobile-First) & Mobile UX Flow
  **Screen 1: The Magic Button (Dashboard)**
  *   Clean, macOS-style Translucent Glass dashboard card.
  *   A single, prominent primary button: `[ 📸 Add Product via Photo ]`
  *   No complicated sidebars. Just the essential daily stats and the magic button.

  **Screen 2: Loading / Processing (The "Teammate" at work)**
  *   Skeleton UI with a gentle shimmer effect.
  *   Playful, plain-language status text:
      *   *"Analyzing your photo..."*
      *   *"Writing a catchy description..."*
      *   *"Setting up inventory..."*

  **Screen 3: The 1-Tap Approval (Action Feed)**
  *   A clean summary card:
      *   **Image:** Automatically cropped and enhanced version of the uploaded photo.
      *   **Title:** e.g., "Artisan Vegan Strawberry Cake"
      *   **Price:** Suggested based on market data (editable).
      *   **Description:** A rich, engaging 2-sentence description.
  *   Buttons: `[ Publish to Store ]` (Primary) or `[ Edit Details ]` (Secondary).

  **Grandmother Test Verification:** A user who only knows how to use the standard iOS/Android Camera app can successfully list a product in under 30 seconds.

  ### AI Agent Integration Points
  *   **The Visualizer (Vision AI):** Analyzes the image to extract product type, color, materials, and potential variants. Enhances/crops the image automatically (background removal optional but recommended).
  *   **The Promoter (Marketing AI):** Generates high-converting, brand-aligned product descriptions and SEO metadata based on the Vision AI's output.
  *   **The Vigilant Manager (Ops AI):** Structures the catalog entry, suggests pricing, and synchronizes the state across the multi-tenant online storefront and the in-person Tap-to-Pay POS.

  ### Key Design Decisions and Why
  *   **Approval Queue over Direct Publish:** We queue the generated listing in an "Action Required" feed instead of publishing it blindly. This maintains user trust and control while keeping friction near zero.
  *   **No Forms by Default:** We hide all technical fields (SKU, weight, SEO title, meta description) behind an "Advanced Settings" switch. Maya just sees the photo, the title, and the price.
  *   **Event-Driven Architecture:** By utilizing the event mesh, we decouple the heavy AI processing from the user's synchronous request, ensuring the mobile app never freezes or times out during generation.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to build the "Invisible Magic Catalog" feature. The Core User Journey (CUJ) is as follows:
  A user on a mobile device (375px viewport) clicks "Add Product", uploads a single photo from their camera roll, and within moments receives a fully fleshed-out product listing draft in their action feed, which they can publish with one tap.

  **Acceptance Criteria:**
  *   **Mobile-First UX:** The entire flow must be flawlessly usable on a 375px screen. The UI must follow our macOS glassmorphism / UniFi modular card design system.
  *   **Zero Forms:** The user must not be required to type a description, title, or price to get the first draft. The AI agents must generate these autonomously based on the uploaded image.
  *   **Agent Handoff:** The system must properly orchestrate the handoff between Vision analysis and text generation via the event mesh, avoiding long-running synchronous blocking requests.
  *   **1-Tap Approval:** The generated product must land in an "Approval Queue" or "Action Feed". The user must be able to publish it with a single tap.
  *   **Grandmother Test:** The interface must use plain language. Technical terms like "SEO", "Variants", or "SKUs" must be hidden behind an "Advanced Settings" toggle.
  *   **Performance:** The initial file upload must be fast, and the background processing must provide engaging loading feedback if it takes more than a few seconds.

  *(Note: You are free to design the exact database schemas, API endpoints, and function signatures required to fulfill this CUJ. Ensure strict multi-tenant isolation and secure identity validation are maintained throughout the event flow.)*
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
