issue_title: "Architecture: Autonomous Hyper-Local Direct Mail & Physical Neighborhood Marketing Engine"
issue_description: |
  # Autonomous Hyper-Local Direct Mail & Physical Neighborhood Marketing Engine

  ## Problem Statement
  Service providers like Carlos (The Freelance Handyman) and Leo (The Music Tutor) heavily rely on hyper-local word-of-mouth. When Carlos fixes plumbing at a specific address, the highest-converting marketing action is notifying the immediate neighbors ("We just did a great job at your neighbor's house. Need a handyman?"). Currently, Carlos has no time to print and distribute flyers, and digital ads (Facebook/Google) are too broad and expensive for micro-neighborhood targeting. OHC currently lacks a bridge between digital operations and physical-world localized marketing.

  ## Research Report
  - **Market Gap:** Shopify and Wix are purely digital. They don't help a handyman get local jobs. Physical marketing platforms (like VistaPrint or direct mail houses) are completely disconnected from the operational and booking software.
  - **Opportunity:** By integrating a Direct Mail API (like Lob or PostGrid) directly into the OHC operations ledger, we can fully automate physical marketing.
  - **Trigger:** When Carlos marks a job as "Complete" and receives the final deposit at a specific address, the AI Marketing agent automatically generates a localized postcard.
  - **Execution:** The postcard includes a generated message ("Hi neighbor! We just completed a repair down the street...") and a unique, trackable QR code. The Direct Mail API dispatches this to a 50-house radius around the job site.
  - **Competitor Landscape:** No major SMB platform offers zero-touch, job-triggered physical direct mail. This creates an "Unfair Advantage" for OHC in the home services sector.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Carlos as Business Owner
      participant OHC_Ops as Operations Agent
      participant OHC_Mktg as Marketing Agent
      participant OHC_DB as OHC Ledger
      participant Lob as Direct Mail API (Lob)
      participant Neighbor as Local Neighbor

      Carlos->>OHC_Ops: Mark job complete & collect final payment (123 Main St)
      OHC_Ops->>OHC_DB: Update job status (COMPLETED)
      OHC_Ops->>OHC_Mktg: Emit JobCompletedEvent(Location)
      OHC_Mktg->>OHC_Mktg: Generate localized copy & unique QR tracking code
      OHC_Mktg->>Lob: POST /v1/postcards (Radius=50 homes)
      Lob-->>OHC_Mktg: Dispatch confirmation
      Lob->>Neighbor: Deliver physical postcard
      Neighbor->>OHC_Mktg: Scan QR Code (Link-in-bio / Booking Page)
      OHC_Mktg->>Carlos: Weekly Briefing: "Your physical mailers got 3 new leads!"
  ```

  ### Mobile UX Flow (375px)
  1. **Settings / Marketing:** A simple toggle "Auto-Mail Neighbors after Job (Cost: ~$0.50/card)". User selects radius (e.g., 50 homes) and budget cap per week.
  2. **Job Completion:** When Carlos taps "Complete Job" on his mobile app, a small toast appears: "Glassmorphism card: 📍 Mailed 50 neighbors nearby."
  3. **Analytics Dashboard:** A UniFi-style card showing "Physical Leads" tracking scans from the unique QR codes.

  ### AI Department Coordination
  - **Operations Agent:** Emits the location and job context when a task is completed.
  - **Marketing Agent:** Ingests the context, uses the LLM to write a hyper-relevant postcard, generates the dynamic QR code linking to the booking funnel, and calls the Lob API.
  - **Finance Agent:** Deducts the micro-transaction cost ($0.50 per card) from the tenant's ledger and updates the weekly expense report.
  - **Business Advisory Agent:** Correlates QR scans to booked revenue and advises if the ROI is positive ("Direct mail generated $500 this month on a $25 spend. Keep it on!").

  ## Implementation Prompt
  Implement the **Autonomous Hyper-Local Direct Mail Engine**.
  1. Integrate the `Lob` API (or similar Direct Mail API) client into the backend services.
  2. Update the `JobCompletedEvent` to trigger the `MarketingAgent`.
  3. Create a workflow in the `MarketingAgent` that generates a postcard HTML template (with a dynamic QR code) and dispatches it via the API to neighbors within a specified radius of the job address.
  4. Add a settings toggle in the mobile-first frontend for "Neighborhood Auto-Marketing" including budget limits.
  5. Create a `DirectMailLedger` table to track API costs and attribute QR code scans back to specific mail campaigns.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
