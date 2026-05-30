issue_title: "[research] Architecture for Zero-Config, Multi-Tenant Unified Domain & SSL Management Engine"
issue_description: |
  # Zero-Config, Multi-Tenant Unified Domain & SSL Management Engine

  ## Problem Statement
  One of the most complex tasks for a non-technical small business owner (like Maya the Baker or Carlos the Handyman) is configuring custom domains, modifying DNS A/CNAME records, and setting up SSL certificates. Traditional platforms (Shopify, Wix) often push users out to external registrars (GoDaddy, Namecheap) which involves overwhelming jargon. Even "connected" setups frequently break due to DNS propagation confusion. Our users need a completely invisible, zero-configuration engine where typing "mayascakes.com" instantly secures and connects the domain, backed by automatic SSL provisioning and renewal, without them ever seeing a DNS control panel.

  ## Research Report
  *   **Market Context & Competitor Gaps:**
      *   *Shopify:* Has improved native domain buying, but connecting external domains still requires manual DNS configuration (A records to IP, CNAME to shops.myshopify.com). Users frequently get stuck here.
      *   *Wix/Squarespace:* Similar friction. They offer in-house registrars but external connections involve technical documentation.
      *   *GoDaddy:* Owns the registrar market but their platform is cluttered with upsells.
  *   **OHC Advantage:** As a platform managing the entire stack via AI agents, OHC can leverage automated ACME challenges (Let's Encrypt) and cloud-native ingress controllers (like Traefik or Caddy) to handle multi-tenant routing dynamically. The Marketing & Advertising Agent can guide the user conversationally, abstracting the technical mechanics.
  *   **Identified Gap:** OHC currently lacks an autonomous architectural layer designed specifically for multi-tenant, edge-cached domain mapping and zero-touch SSL certificate lifecycle management.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Maya)
      participant Marketing Agent
      participant OHC Ingress Controller (Traefik/Caddy)
      participant Let's Encrypt
      participant Postgres (Tenant Ledger)

      User (Maya)->>Marketing Agent: "I want to use mayascakes.com"
      Marketing Agent->>Postgres (Tenant Ledger): Register Domain Intent
      Marketing Agent-->>User (Maya): "Great! Just point your nameservers to ns1.ohc.com"
      Note over OHC Ingress Controller, Let's Encrypt: Automated ACME Challenge (HTTP-01/DNS-01)
      OHC Ingress Controller->>Let's Encrypt: Request SSL Cert for mayascakes.com
      Let's Encrypt-->>OHC Ingress Controller: Issue SSL Cert
      OHC Ingress Controller->>Postgres (Tenant Ledger): Update Domain Status (Active, Secure)
  ```

  ### Core Capabilities
  1.  **Automated SSL Provisioning:** Integration with Let's Encrypt for automatic creation and renewal of certificates for all tenant domains.
  2.  **Dynamic Edge Routing:** An ingress controller capable of routing incoming traffic to the correct tenant application based on the Host header, tightly integrated with the multi-tenant caching layer.
  3.  **Conversational Setup:** The Marketing Agent handles the "DNS" conversation, providing simple, context-aware instructions (e.g., "Change your nameservers") instead of listing raw A/CNAME records.

  ### Mobile UX Flow
  1.  **Domain Input:** A simple text field: "What's your website address?" (375px optimized).
  2.  **Status Indicator:** A clear, color-coded status indicator (e.g., "Connecting...", "Secure & Live") using premium glassmorphism design.
  3.  **AI Assistant Chat:** A chat interface where the Marketing Agent guides the user through the process.

  ### AI Agent Integration Points
  *   **Marketing & Advertising Agent:** The primary interface for the user, managing the conversational flow and simplifying instructions.
  *   **Operations Agent:** Monitors domain status and alerts the user if DNS settings are changed externally or if SSL renewal fails.

  ## Implementation Prompt
  Design and implement the core data models and service logic for the Zero-Config Domain & SSL Management Engine.
  1.  **Data Models:** Create schemas for `TenantDomain` (domain name, tenant ID, status, SSL expiry) with strict multi-tenant isolation.
  2.  **Service Layer:** Implement a backend service (Go/Rust) that interfaces with an ingress controller API (e.g., Caddy API) to dynamically add/remove domains and trigger SSL provisioning.
  3.  **Frontend Component:** Develop a simple UI component allowing a user to input a domain name, showing real-time connection status.

issue_priority: P0
issue_estimated_scope: Large
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
