issue_title: "Implement Universal Autonomous Portfolio & Social Proof Mesh"
issue_description: |
  # [Architecture] Universal Autonomous Portfolio & Social Proof Mesh

  ## Problem Statement

  Small business owners in service, creative, and portfolio-driven industries (like Leo the music tutor or Carlos the handyman) rely heavily on word-of-mouth and visual proof of their work to acquire new clients. However, curating a portfolio and collecting testimonials is a highly manual, time-consuming process. Carlos finishes a basement repair and forgets to take an "after" photo or ask for a review. Leo has happy students but no simple way to showcase their progress to prospective clients. Currently, OneHumanCorp (OHC) handles the transactional side (booking, invoicing) but leaves a gap in leveraging successful transactions into automated marketing assets. Competitors like Wix and Squarespace require manual updates and disjointed review widgets, placing the burden of curation on the business owner. OHC needs an autonomous system that proactively requests, collects, curates, and publishes social proof (photos, reviews) immediately after a job is completed, transforming every happy customer into a marketing asset with zero manual effort from the owner.

  ## Research Report

  We analyzed how leading platforms handle portfolio curation and testimonial collection for service and creative micro-businesses. The findings indicate that manual portfolio management is a leading cause of stale websites ("marketing dread"), whereas automated request systems significantly boost conversion rates.

  ### Competitive Analysis

  | Platform | Portfolio Management | Testimonial Collection | Key Weakness (The OHC Opportunity) |
  |---|---|---|---|
  | **Squarespace** | Beautiful static templates | Manual text entry or clunky third-party widgets | The owner must manually ask for reviews, download photos, and update the site. |
  | **Wix** | Drag-and-drop builders | App Market integrations (e.g., Yotpo, Trustpilot) | Requires paying for and configuring third-party apps; no native AI auto-curation. |
  | **Shopify** | Focused on products, not services | App Market heavily skewed to product reviews | Not designed for service portfolios or before/after galleries. |
  | **GoDaddy** | Basic gallery blocks | Connected to Google Business Profile | Very limited customization and no proactive SMS-based media collection. |
  | **OHC (Target)** | **Edge-Cached Dynamic Gallery** | **Autonomous AI-SMS Collection** | **Must feel like magic: job done -> review requested -> site updated invisibly.** |

  ### Persona Pain Points

  *   **Carlos (Handyman):** "I do great work, but I'm exhausted at the end of the day. I never remember to ask the client to write a review, and my website hasn't been updated in 2 years."
  *   **Leo (Music Tutor):** "I want a link-in-bio page that shows my students playing at their recitals, but getting the videos from parents and putting them on a website is too much technical work."

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant B as Business Ledger (OHC)
      participant O as Operations AI Agent
      participant C as Customer (SMS/WhatsApp)
      participant M as Marketing AI Agent
      participant S as Edge Storefront (CDN)

      B->>O: Event: Job/Booking Completed
      O->>C: SMS: "Hi! Carlos finished the repair. Reply with a photo or a quick review!"
      C-->>O: SMS: Sends "Looks great!" + Photo
      O->>M: Forward collected assets & sentiment
      M->>M: AI Moderation & Auto-Crop/Enhance
      M->>B: Store Approved Portfolio Item & Review
      M->>S: Trigger Edge Cache Invalidation (Rebuild Gallery)
      S-->>C: Portfolio updated globally within seconds
  ```

  ### UI Wireframes & Mobile UX Flow (375px native)

  The experience is entirely mobile-first, designed around the concept of "1-Tap Approval" for the business owner.

  1.  **The Customer Collection Flow (SMS/WhatsApp):**
      *   No app required. The customer receives a conversational message from the business's AI agent.
      *   *UI:* Native SMS thread. "Thanks for choosing Carlos! Tap here to upload a quick photo of your new basement, or just reply with how we did!"
  2.  **The Owner Approval Dashboard (OHC App - 375px):**
      *   *Card-based layout (UniFi style).* A notification card appears on the main dashboard: "New Social Proof Collected."
      *   *UI:* A clean translucent glass card shows the customer's photo and text review.
      *   *Action:* Two massive, thumb-friendly buttons at the bottom: "Publish to Portfolio" (Primary) and "Hide" (Secondary).
  3.  **The Edge Storefront (Customer-Facing):**
      *   The portfolio page dynamically re-renders as a beautiful, masonry-style gallery.
      *   Reviews are overlaid on the relevant job photos, optimized for fast mobile loading.

  ### AI Agent Integration Points

  *   **Operations Department:** Monitors the ledger for `JobCompleted` or `InvoicePaid` events to trigger the collection workflow at the optimal time (e.g., 1 hour after a tutoring session, 1 day after a basement repair).
  *   **Marketing Department:** Handles the conversational AI with the customer. It understands context ("Carlos fixed your sink") and can gently prompt for a photo. It also performs automated image enhancement (brightness, cropping) and sentiment analysis to filter out negative reviews for private owner triage.

  ### Key Design Decisions

  *   **SMS/WhatsApp First for Customers:** We do not ask customers to log into a portal to leave a review. We meet them where they are (messaging apps) to maximize response rates.
  *   **Opt-In Curation (1-Tap):** The AI curates and prepares the portfolio update, but the business owner always has the final 1-tap approval before it goes live, ensuring brand safety without the manual labor of building the page.
  *   **Edge-Native Portfolio:** The public-facing portfolio is compiled to static assets and served via edge CDN for sub-100ms load times, critical for link-in-bio use cases.

  ## Implementation Prompt

  **To the Implementer Swarm:**

  Your task is to build the "Universal Autonomous Portfolio & Social Proof Mesh." This system must automatically request, collect, and publish customer reviews and photos upon the completion of a service or job.

  **Core User Journeys (CUJs):**
  1.  **The Trigger:** When a job is marked as "Complete" or an invoice is "Paid" in the OHC ledger, the system must automatically queue a delayed conversational SMS/WhatsApp message to the customer asking for feedback and a photo of the completed work.
  2.  **The Ingestion & Processing:** The system must receive the customer's reply (text and/or media), process the image (generate thumbnails, optimize for web), and analyze the sentiment of the text.
  3.  **The 1-Tap Approval:** The business owner must see a clean, actionable card in their mobile dashboard summarizing the collected social proof, with a single button to "Publish."
  4.  **The Edge Publish:** Upon approval, the data must be persisted, and the public-facing edge storefront portfolio must instantly reflect the new review/media.

  **Acceptance Criteria:**
  *   **Zero-Config Setup:** The business owner does not need to configure any third-party review widgets.
  *   **Mobile-First Owner UX:** The approval flow must be flawless on a 375px width screen, utilizing our design system's card layouts.
  *   **Performance:** The public portfolio must update within seconds of owner approval and serve via edge caching.
  *   **Graceful Degradation:** If the customer replies with only text and no photo, the system must generate a beautiful text-only review card for the portfolio.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []