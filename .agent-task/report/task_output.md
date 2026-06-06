issue_title: "Autonomous SEO & Local Discovery Agent"
issue_description: |
  # OHC Agent Solutions: Autonomous SEO & Local Discovery Agent

  ## Problem Statement
  Small business owners like Carlos (Handyman) and Fatima (Food Cart Operator) struggle to get their business discovered locally on search engines like Google. Existing platforms like Shopify or Wix require users to understand concepts like meta tags, alt text, structured data, and keyword research, turning SEO into a manual, technical chore that most micro-SMEs simply abandon. They need customers to find them effortlessly, but they do not have the time or technical expertise to manage local SEO.

  ## Research Report
  - **The Gap**: Traditional builders (Squarespace, Wix, Shopify) provide SEO "tools" and fields, but rely entirely on the user to fill them out correctly.
  - **Competitor Tools**: Shopify offers basic automated titles but largely relies on third-party apps for deep SEO. Wix has an "SEO Wizard" which still requires significant user input. Link-in-bio tools lack SEO capabilities entirely.
  - **OHC Differentiation**: OHC's "Promoter Agent" must treat SEO not as a checklist for the user, but as an invisible, autonomous background process. The agent automatically infers context from the user's products/services and location, generating localized SEO metadata, structured data (Schema.org), and Google Business Profile syncs without any human input other than a 1-tap approval.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Product : has
      Tenant ||--o{ Service : has
      Tenant ||--|| BusinessProfile : manages
      BusinessProfile ||--o{ SEOData : contains
      SEOData {
          string meta_title
          string meta_description
          json structured_data
          string generated_keywords
      }
      AgentJob ||--|| SEOData : updates
      AgentJob {
          uuid id
          string type
          string status
          json payload
      }
  ```

  ```mermaid
  sequenceDiagram
      participant User
      participant ProductService
      participant PromoterAgent
      participant LLMProvider
      participant GoogleBusinessAPI

      User->>ProductService: Create/Update Product
      ProductService-->>PromoterAgent: Trigger Event: EntityUpdated
      PromoterAgent->>LLMProvider: Send Product Data + Location Context
      LLMProvider-->>PromoterAgent: Return Generated Meta Tags & Schema
      PromoterAgent->>PromoterAgent: Update Tenant SEOData
      PromoterAgent->>User: Push Mobile Approval Card
      User->>PromoterAgent: "Approve & Publish"
      PromoterAgent->>GoogleBusinessAPI: Sync to Google Profile (if connected)
  ```

  ### UI Wireframes & Mobile UX
  - **Mobile Approval Card (375px)**: Surfaces in the Agent Feed. "The Promoter Agent noticed you added a new plumbing service. I've optimized it so locals searching for 'plumbers near me' can find it. Want me to publish?"
  - **Actions**: "Approve" (Large 44px touch target) or "Edit" (reveals advanced options for the 1% who want them).
  - **Zero Jargon**: Terms like "Meta Tags", "H1", and "Schema" are completely hidden from the primary interface.

  ### AI Agent Integration Points
  - **Trigger**: System events for catalog changes, location updates, or new testimonials.
  - **LLM Pipeline**: Generates localized, intent-driven meta descriptions and `Product`/`LocalBusiness` JSON-LD schema based on the catalog and business profile context.
  - **Google Business Profile Integration**: Uses "The Promoter" to auto-sync business hours, new products, and services directly to Google Search/Maps.

  ## Implementation Prompt
  Implement the "Autonomous SEO & Local Discovery" module for the Promoter Agent:
  - Create a background worker that listens for `EntityUpdated` events (products, services, business details).
  - Integrate with the LLMProvider interface (Gemini Pro) to generate SEO-optimized meta tags and JSON-LD structured data based on the entity's context and the tenant's location.
  - Implement the data storage for the generated SEO content (e.g., in a new `seo_metadata` table or as an extension to existing entities).
  - Create the mobile-first approval workflow: expose an API endpoint that surfaces the proposed SEO changes as an actionable card for the user's dashboard feed.
  - Optional/Stretch: Implement the sync mechanism to Google Business Profile for seamless local search visibility.
  - *Acceptance Criteria*: A non-technical user adds a new product, and within 1 minute, the agent surfaces a perfectly generated local SEO payload for approval. The UI must be fully functional on a 375px viewport with no technical jargon.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
