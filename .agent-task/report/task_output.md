issue_title: "AI-Powered Multi-Modal Quote & Estimation Flow"
issue_description: |
  ## Problem Statement
  Field service operators like Carlos (Handyman) heavily rely on their mobile phones (Android) to manage customer interactions, quotes, and estimates. Currently, generating an accurate quote requires taking a photo, manually reviewing it, and then switching between an email/messaging app and an invoicing tool to type out the details, line items, and pricing. This process is slow, prone to errors, and highly disruptive to their workflow when on the road or at a customer site with spotty connectivity. There is no seamless, offline-first way to capture job context via image and turn it instantly into an actionable, customer-ready quote using AI.

  ## Research Report
  ### Competitive Analysis
  - **Square Invoices:** Supports generating invoices on the go, but lacks native AI integration to generate line items automatically from job site photos or verbal notes.
  - **Jobber / Housecall Pro:** Great for scheduling and dispatch, but still require manual entry for quotes. They do not leverage multi-modal AI to translate a picture of a broken pipe into a structured estimate.
  - **Shopify / Wix:** Primarily focused on physical and digital goods rather than complex field service quoting based on environmental context.

  ### Findings & Opportunities
  - Mobile-first field operators need a "point-and-shoot" quoting experience. They point the camera at the problem, take a photo (or a short video/audio note), and the AI assistant instantly drafts a quote with line items, estimated labor, parts, and a deposit request.
  - The process must be fully resilient to network conditions (Offline-first / CRDT caching). Carlos might be in a customer's basement with zero reception when taking the photo.
  - Integration with the Business Advisory and Customer Success AI agents ensures that the follow-up message matches the business's tone and automatically sends the finalized quote when the device is back online.

  ## Design Doc
  ### Mobile UX Flow (375px Base)
  1. **Capture Mode:** The OHC app home screen features a prominent "New Quote" FAB (Floating Action Button). Tapping it opens a camera/multimodal capture screen.
  2. **Context Input:** Carlos takes a photo of a broken water heater and optionally adds a voice note: "Needs a new 50-gallon gas heater and 2 hours labor."
  3. **Offline Queue (If disconnected):** A subtle amber "Offline Mode - Quote pending sync" pill appears.
  4. **AI Generation (When online):** The app uploads the context. The Sales & Revenue Assistant processes the image and audio, matching it against Carlos's typical pricing matrix (RAG).
  5. **Review & Approve:** A translucent Glassmorphism card appears presenting the drafted quote:
     - Line Item 1: 50-gallon Gas Water Heater (Parts) - $X
     - Line Item 2: Labor (2 hours) - $Y
     - Total: $Z
  6. **Action:** Carlos taps "Approve & Send". The Customer Assistant drafts the SMS/Email, and the quote is dispatched with a Stripe Payment Link for the deposit.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      actor Carlos
      participant MobileApp as OHC Mobile App (Flutter)
      participant LocalCache as Local SQLite (CRDT)
      participant Gateway as OHC Gateway (Go)
      participant AIAgent as Sales & Operations Agent (Gemini)
      participant Ledger as OHC Ledger / DB

      Carlos->>MobileApp: Take Photo & Voice Note
      MobileApp->>LocalCache: Save Draft Quote Context
      alt Offline
          MobileApp-->>Carlos: Show "Pending Sync" Pill
      end
      MobileApp->>Gateway: Sync Context (when online)
      Gateway->>AIAgent: Process Image + Audio + Pricing RAG
      AIAgent-->>Gateway: Return Structured Quote Draft
      Gateway->>Ledger: Store Draft Quote
      Gateway-->>MobileApp: Push Draft for Review
      Carlos->>MobileApp: Approve & Send
      MobileApp->>Gateway: Confirm Quote
      Gateway->>Customer: Dispatch SMS + Payment Link
  ```

  ### AI Agent Integration Points
  - **Operations Assistant:** Handles the background queueing and retry logic if the device goes offline.
  - **Sales & Revenue Assistant:** Uses Gemini Pro Vision to analyze the photo and audio note, querying the local tenant database (RAG) for matching part prices and labor rates.
  - **Customer Assistant:** Drafts the localized, friendly message accompanying the quote.

  ## Implementation Prompt
  **User Facing Outcome:** Carlos can open the OHC app, tap "New Quote," take a photo of a job site (e.g., a damaged wall), dictate a short note, and receive a fully drafted quote with line items and pricing within seconds. The app must allow him to do this even in a basement with no cellular service, queuing the request and generating the quote automatically once he reconnects.

  **Critical User Journey (CUJ) / Acceptance Criteria:**
  - Build a responsive (375px mobile-first) UI for the Quote Capture screen, including camera integration and audio recording placeholders.
  - Implement the optimistic offline UI: When network is down, show a clear "Offline Mode" indicator and save the quote context to local SQLite via CRDT.
  - Integrate with the AI Backend: When online, the backend should accept the multi-modal payload (image + text/audio), invoke the Sales Assistant (Gemini) to parse the job requirements, and return a structured estimate.
  - Display the generated quote in a premium Translucent Glass card for Carlos to review, edit, or approve.
  - Upon approval, the system must generate a deposit payment link (Stripe) and transition the task to the unified inbox feed.
  - Provide at least 5 complete Playwright E2E tests verifying the online flow, the offline optimistic queuing, and the AI draft review step. ZERO mock data in the UI.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []