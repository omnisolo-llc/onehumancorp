issue_title: "Agentic Mobile-First Client Portal & Real-Time Approval Mesh"
issue_description: |
  ## Title: Agentic Mobile-First Client Portal & Real-Time Approval Mesh

  ## Problem Statement
  Service-based small business owners (like Nora the agency principal or Carlos the field service owner) struggle to keep clients updated and secure approvals without constant back-and-forth emails, SMS, or phone calls. When a proposal needs approval, a design needs feedback, or a repair needs sign-off on an unexpected cost, the process is manual and disjointed. Existing tools require clients to download apps, create accounts, or navigate complex portals, leading to friction and delayed decisions. Owners need a frictionless, zero-login, mobile-first way to share work progress, gather approvals, and collect payments, orchestrated invisibly by their AI assistant.

  ## Research Report
  - **Current Capabilities**: OHC has basic quoting and unified booking capabilities, but lacks a persistent, client-facing space for ongoing projects or services where they can view history, current status, and pending action items without navigating complex auth flows.
  - **Competitive Analysis**:
    - *Shopify / Wix / Squarespace*: Primarily built for transactional commerce. They lack native project management or approval workflows for service businesses. Client portals usually require clunky third-party apps with separate logins.
    - *HoneyBook / Dubsado*: Strong in client portals, but often feel heavy and require client logins. They rely on manual trigger points rather than autonomous AI orchestration.
    - *Basecamp / Asana / ClickUp*: Built for team collaboration, far too complex for simple client-facing approvals.
  - **Gap Identified**: A "Zero-Login" Mobile-First Client Portal driven by "Magic Links." When the owner or an agent needs client input, the client receives an SMS/Email with a secure, ephemeral link. This link opens a translucent, app-like mobile web view (375px optimized) showing exactly what needs attention (e.g., "Approve $50 parts cost", "Review Logo Draft").
  - **Strategic Advantage**: By combining OHC's AI agents with a frictionless client portal, we can autonomously resolve operational blockers (like pending approvals) and accelerate the quote-to-cash cycle without the owner having to manually chase clients.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CLIENT_PORTAL_SESSION : manages
      CLIENT_PORTAL_SESSION ||--|{ APPROVAL_REQUEST : contains
      CLIENT_PORTAL_SESSION ||--o{ MESSAGE_THREAD : hosts
      APPROVAL_REQUEST ||--o{ QUOTE : references
      APPROVAL_REQUEST ||--o{ ASSET : references

      TENANT {
          string id PK
          string name
      }
      CLIENT_PORTAL_SESSION {
          string id PK
          string tenant_id FK
          string customer_id FK
          string magic_token
          datetime expires_at
      }
      APPROVAL_REQUEST {
          string id PK
          string session_id FK
          string status "Pending | Approved | Rejected"
          string type "Quote | Design | ChangeOrder"
          string description
      }
      ASSET {
          string id PK
          string url
          string type
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Trigger**: The Operations Agent (or Nora) identifies a need for client approval (e.g., a revised quote). The agent autonomously generates a secure "Magic Link" and sends it to the client via SMS.
  2. **Access**: The client taps the link and instantly opens the Mobile-First Client Portal in their browser. No login required. The UI is built using macOS-style Translucent Glass materials.
  3. **Action Hub**: The screen displays a clear, singular call-to-action card at the top: "Action Required: Approve Revised Quote."
  4. **Review & Approve**: Tapping the card opens a half-sheet modal detailing the changes. The client taps "Approve" and signs with a finger swipe.
  5. **Resolution**: The portal updates to a success state. In the background, the Operations Agent receives the approval event, advances the project status in the OHC engine, and notifies Nora via her daily brief.

  ### AI Agent Integration Points
  - **The Vigilant Manager (Operations)**: Identifies blockers requiring client input. Automatically generates Approval Requests and orchestrates the sending of Magic Links.
  - **The Silent Ambassador (Customer Success)**: Monitors pending Approval Requests. If a client hasn't responded in 24 hours, it autonomously sends a gentle, context-aware nudge via SMS.
  - **The Business Advisor**: Analyzes approval turnaround times and suggests process improvements (e.g., "Clients take 2 days to approve quotes over $1000. Consider offering a split payment option in the initial quote.").

  ### Key Design Decisions
  - **Zero-Login via Magic Links**: Reduces friction to zero. Security is maintained via ephemeral, scoped tokens tied to specific customer sessions and rate-limited.
  - **Agent-Driven Orchestration**: The portal is not a static destination; it is an active surface populated by AI agents based on real-time operational needs.
  - **Edge-Cached Delivery**: Portal assets and initial states must be edge-cached for instant loading, crucial for mobile users on cellular networks.

  ## Implementation Prompt
  Implement the Agentic Mobile-First Client Portal & Real-Time Approval Mesh.
  The system must provide a mechanism to generate secure, ephemeral "Magic Links" that grant zero-login access to a client-specific, mobile-optimized web view. This portal should display pending "Approval Requests" (e.g., for quotes, change orders, or design assets).
  Ensure the UI uses macOS-style Translucent Glass materials and is strictly optimized for a 375px viewport.
  The backend must emit events upon client approval that can be consumed by the AI Operations Agent to autonomously advance project state.
  Acceptance criteria include: successful generation of a magic link, rendering of the portal without authentication walls, successful submission of an approval by the client, and verification that the approval event is recorded in the tenant's ledger and triggers the appropriate agent response.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
