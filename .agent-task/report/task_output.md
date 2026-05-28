issue_title: "Implement Autonomous Service Portfolio & Case Study Generator"
issue_description: |
  # Title: Autonomous Service Portfolio & Case Study Generator

  ## Problem Statement
  Service-based small business owners like Carlos (Handyman) and Leo (Music Tutor) rely heavily on "proof of work" to win new clients. However, after finishing an exhausting day of physical labor or back-to-back lessons, the last thing they want to do is sit down at a laptop, upload photos, write marketing copy, and update a website portfolio. As a result, their online storefronts remain stale, and they lose potential customers who want to see recent examples of their work. They need an invisible system that automatically generates beautiful, professional case studies directly from their daily workflow (like snapping a "finished" photo and texting it to the client) without requiring them to use a site builder.

  ## Research Report
  *   **Current Architecture Limits:** OHC's storefront builder requires manual intervention to add new portfolio items, mirroring the high-friction experience of legacy platforms.
  *   **Competitor Analysis:**
      *   *Wix & Squarespace:* Provide beautiful portfolio templates but require the user to manually crop images, write descriptions, and manage layouts on a desktop.
      *   *Instagram/TikTok:* The default "portfolio" for many SMBs, but it's disconnected from the actual booking and payment flow, making conversion harder.
      *   *Houzz/Thumbtack:* Industry-specific directories that force merchants into a competitive marketplace rather than owning their brand.
  *   **Discovery:** OHC can bridge this gap by listening to the Omnichannel Inbox or Job Completion events. When Carlos texts a customer a photo of a finished fence, the AI Marketing Agent can proactively draft a case study, format the before/after pictures, and ask for a 1-tap approval to publish it directly to his OHC storefront.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Small Biz Owner (Mobile)
      participant Inbox as Omnichannel AI Inbox
      participant Agent as Marketing Agent
      participant Storage as Multi-Tenant Blob Storage
      participant Edge as Edge-Cached Storefront

      Owner->>Inbox: Sends "All done!" + Photo to Customer
      Inbox->>Agent: Event: Job Completed with Media
      Agent->>Agent: Extract project context (Quote, Location, Service)
      Agent->>Agent: Generate SEO-friendly case study copy
      Agent->>Storage: Optimize and store images
      Agent->>Owner: Push: "Publish new case study for Fence Install?"
      Owner->>Agent: 1-Tap Approve (Mobile App)
      Agent->>Edge: Revalidate & publish portfolio page
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1.  **The Trigger (Push Notification):** Carlos finishes a job, texts the customer a photo via OHC. Five minutes later: "✨ Drafted a new portfolio post for 'Cedar Fence Install'. Tap to review."
  2.  **The Review Card (Mobile App):**
      *   Clean, Translucent Glass card overlays the dashboard.
      *   **Hero Image:** The photo he just took, automatically color-corrected and cropped to look professional.
      *   **Generated Copy:** "Beautiful new cedar privacy fence installed in downtown area. Completed on time and on budget."
      *   **Action Buttons:** Massive primary button: "Publish to Website". Ghost button: "Edit".
  3.  **The Live View:** Tapping "Publish" instantly transitions to a celebratory animation. The live customer-facing portfolio page is instantly updated via Edge Caching.

  ### Key Design Decisions
  *   **Zero-Friction Creation:** The feature relies on ambient data collection (photos sent in chat, invoices paid) rather than asking the user to fill out a form.
  *   **Edge-Caching:** Portfolio updates must be visible immediately on the public internet, requiring tight integration with the storefront's CDN invalidation pipeline.
  *   **Multi-Tenant Privacy:** The Marketing Agent must only use data from the specific tenant's jobs and must scrub any PII (Customer name, exact address) before generating public case study copy.

  ### AI Agent Integration Points
  *   **Marketing Agent:** Analyzes the image (is it a good quality photo of the work?), extracts the context from the associated invoice/quote, and writes professional, SEO-optimized copy.
  *   **Customer Success Agent:** If the customer replies to the "All done" text with a positive review, it routes that review to be attached to the case study automatically.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Build the "Autonomous Service Portfolio Generator". The system must listen to communication or invoice-completion events that contain media, use an LLM to generate a professional case study (title, description, and optimized images), and present it to the business owner for 1-tap approval on their mobile device.

  **User Journey (CUJ):**
  1. User completes a job and marks the invoice as paid, attaching a "finished" photo to the customer thread.
  2. The system automatically drafts a portfolio post summarizing the service provided.
  3. The user receives a push notification and approves the draft with one tap.
  4. The post is instantly visible on their public storefront.

  **Acceptance Criteria:**
  *   Event listener triggers correctly when jobs complete with media.
  *   AI generation scrubs PII from the public-facing draft.
  *   Approval action correctly invalidates the storefront cache and publishes the new content.
  *   UI follows the 375px mobile-first Glassmorphism design system.
  *   Strict multi-tenant isolation ensures data from one business never leaks into another's portfolio.
  *   Do not prescribe specific database schemas or API endpoints; let the internal system details be optimized for resilience and performance.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
