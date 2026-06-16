issue_title: "Implement Multi-Channel Customer Identity Resolution Architecture"
issue_description: |
  **Title**: Multi-Channel Customer Identity Resolution Architecture

  **Problem Statement**:
  Small business owners (like Carlos the handyman or Maya the baker) receive inquiries and bookings from multiple isolated channels (Instagram DMs, WhatsApp, Email, Web Forms). Currently, these interactions create fragmented customer records, forcing the owner to manually piece together a customer's history. Without a unified identity graph, AI agents (like The Ambassador) cannot proactively draft accurate, context-aware responses or follow-ups because they lack the full picture of the customer's prior purchases, preferences, and conversations.

  **Research Report**:
  - *Shopify*: Handles unified customer profiles well within its own ecosystem (email, shop app), but struggles to link external social identities without heavy third-party CRM integrations (like Klaviyo or HubSpot).
  - *Wix/Squarespace*: Provide basic contact lists but do not autonomously merge identities based on probabilistic matching (e.g., matching a phone number from a WhatsApp message to an email from a past booking).
  - *Codebase Audit*: The current OHC services directory (`src/server/services/`) includes `chat`, `booking`, and `billing`, but lacks a dedicated `identity` or `customer_graph` service to resolve and merge multi-channel interactions into a single tenant-scoped `Customer` entity.
  - *OHC Opportunity*: Implement a robust Customer Identity Resolution Engine that deterministically and probabilistically merges fragmented records into a Unified Customer Graph. This enables OHC agents to provide true omnichannel support seamlessly.

  **Design Doc**:
  - *Architecture Diagram*:
    ```mermaid
    graph TD
        A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
        C[WhatsApp] -->|Webhook| B
        D[Email/Web] -->|API| B
        B --> E{Identity Resolution Engine}
        E -->|Deterministic Match| F[Unified Customer Graph DB]
        E -->|Probabilistic Match| G[Merge Suggestions Queue]
        F --> H[The Ambassador Agent]
        G --> I[Mobile App Feed 375px]
        I -->|Owner Approves Merge| F
    ```
  - *Mobile UX Flow (375px First)*:
    When the engine detects a highly probable match (e.g., same name and similar location but different email), it surfaces a card in the owner's Mobile Feed: "Is Sarah from Instagram the same Sarah who booked yesterday? [Yes, Merge] [No]".
  - *AI Agent Integration Points*:
    The Ambassador Agent will query the `Unified Customer Graph DB` instead of isolated chat logs, allowing it to say "Hi Sarah, do you want to re-order the cake you got last week?" when Sarah DMs on Instagram.
  - *Key Design Decisions*:
    - **Tenant Isolation**: All identity resolution must occur strictly within the boundaries of a single `tenant_id` (using row-level security).
    - **Immutable Audit Trail**: When merging records, the system must retain references to the original channel-specific identities (e.g., `instagram_id`, `whatsapp_phone`) for outbound routing.

  **Implementation Prompt**:
  *User-Facing Outcome*: As a business owner, when a customer texts me on WhatsApp, the OHC app automatically recognizes them from their past email booking and shows their entire history in one view.
  *CUJ & Acceptance Criteria*:
  1. A webhook payload arrives with an Instagram handle.
  2. The Identity Resolution Engine queries the database and finds no deterministic match but finds a strong probabilistic match based on name.
  3. A "Merge Suggestion" is created.
  4. The owner views the suggestion on a 375px mobile screen and taps "Merge".
  5. The system unifies the records; subsequent queries to the Customer Graph return interactions from both Instagram and the previous channels.
  6. Acceptance: Unit tests covering merge logic, and Playwright E2E tests for the owner's merge approval flow.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
