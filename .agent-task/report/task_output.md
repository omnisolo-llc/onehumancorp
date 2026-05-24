issue_title: "[architecture]_autonomous_spatial_quoting_and_job_site_context_engine"
issue_description: |
  # Architecture Brief: Autonomous Spatial Quoting and Job-Site Context Engine

  ## Problem Statement
  Service professionals like **Carlos (handyman, 42)** lose significant billable hours traveling to job sites simply to take measurements, assess the environment, and manually calculate material costs before they can even issue a quote. Current solutions (Jobber, Housecall Pro) are just digital clipboards—they still require the user to input the measurements and build the quote line-by-line. Carlos needs an intelligent system where a potential client can simply record a 10-second video of their space (e.g., a broken fence or an empty room to paint), and the AI automatically extracts spatial dimensions, identifies required materials, and drafts a precise, localized quote for Carlos to approve with one tap.

  ## Research Report
  ### Competitive Analysis
  *   **Jobber / Housecall Pro:** Excellent for scheduling and invoicing, but quoting is manual. The business owner must input line items and calculate material costs manually.
  *   **Hover / Magicplan:** Great for 3D modeling and floor plans, but too complex for a simple handyman job and not integrated into an end-to-end small business payment/booking flow.
  *   **OneHumanCorp Opportunity:** Integrate computer vision (spatial analysis) directly into the customer inquiry flow. When a customer messages Carlos via the OHC Inbox, the Sales AI replies: "Could you send a quick video of the broken fence?" The system analyzes the video, calculates square footage, queries local material costs (e.g., Home Depot API context), and prepares a draft quote.

  ### Business Journey Mapping (Carlos)
  1.  **Acquisition:** Customer finds Carlos via his OHC-generated social link.
  2.  **Onboarding/Inquiry:** Customer requests a quote for a painting job via OHC chat.
  3.  **Activation:** The *Customer Success Agent* asks for a short video of the room.
  4.  **Spatial Analysis:** The KAIROS Orchestrator routes the video to the *Operations Agent* (Spatial Context Engine) to extract dimensions and detect obstacles.
  5.  **Quote Generation:** The *Sales Agent* cross-references local paint costs and Carlos's hourly rate to draft the quote.
  6.  **Revenue:** Carlos receives a push notification: "New Quote Drafted for 12x15 Room Paint ($450). Tap to approve and send."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (Mobile)
      participant O as OHC Omnichannel Inbox
      participant CS as Customer Success Agent
      participant SE as Spatial Engine (Ops Agent)
      participant SA as Sales Agent
      participant B as Business Owner (Carlos)

      C->>O: "I need my living room painted."
      O->>CS: New Inquiry Event
      CS->>C: "Hi! Carlos can help. Please send a quick video of the room."
      C->>O: Uploads Video (30s)
      O->>SE: Analyze Spatial Dimensions (Video Context)
      SE-->>SA: Extracted Data: 200 sqft, 2 windows, drywall
      SA->>SA: Calculate Materials + Labor Rate
      SA->>B: Push Notification: "Quote Ready for Review"
      B->>SA: 1-Tap Approve
      SA->>C: Sends Formal OHC Quote with Payment Link
  ```

  ### Mobile UX Flow (375px)
  *   **View 1: Push Notification:** macOS-style Translucent Glass banner. "New Job Request: Painting. AI drafted quote for $450. [Review]"
  *   **View 2: Quote Review Card:** Clean, UniFi modular card layout.
      *   *Header:* Customer Name & Job Type.
      *   *Media:* Auto-playing muted looping thumbnail of the customer's video.
      *   *Insights:* AI detected: "Approx 200 sqft. 2 windows. Requires 2 gallons of paint."
      *   *Financials:* Labor ($200) + Materials ($100) + Margin = $450 total.
      *   *Actions:* Massive primary button: "Approve & Send". Secondary button: "Edit Items".

  ### Security & Multi-Tenancy
  *   Customer media (videos, photos) must be strictly isolated to the specific `Tenant_ID` in the blob storage bucket to ensure absolute Zero-Trust privacy.
  *   Spatial calculations and extracted context must be attached to a temporary `Lead_ID` that is aggressively garbage-collected if the quote is rejected or expires, minimizing storage overhead.

  ## Implementation Prompt
  Implementer Agent:
  Please build the `SpatialQuoteDraftingService` within the core AI Sales department.
  1.  Create an event listener that triggers when a customer uploads media to an active quote request thread.
  2.  Design the service interface to accept the media and return structured spatial/material insights (e.g., dimensions, item counts).
  3.  Integrate the insight data with the existing `QuoteBuilder` to automatically append estimated line items based on the tenant's configured labor rate and local material estimations.
  4.  Ensure the generated draft quote is flagged as `pending_owner_approval` and emits a notification event for the mobile frontend to consume.
  Do not prescribe specific computer vision libraries or database ORMs; focus on the business logic, event boundaries, and robust error handling if the media is unanalyzable.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture-design]
assignees: []