issue_title: "[architecture] Proactive AI Tax, Licensing & Compliance Mesh"
issue_description: |
  # [architecture] Proactive AI Tax, Licensing & Compliance Mesh

  ## Problem Statement
  Small business owners like Fatima (Food Cart Operator) and Carlos (Freelance Handyman) face significant stress and confusion regarding local business compliance. Tax rules, local health permits, liability waivers, and generic contracts are highly localized, complex, and carry severe penalties for non-compliance. Existing platforms (Shopify, Wix) treat compliance as an afterthought or push the burden onto third-party apps, which requires the business owner to already know what they need. They need an invisible protector—an AI that proactively identifies required licenses based on their business type and location, generates legally sound custom contracts automatically, and tracks expiration dates seamlessly from their mobile device.

  ## Research Report
  Current SMB platforms fall short in integrated, proactive compliance:
  *   **Shopify/Wix:** Good at calculating sales tax at checkout, but terrible at helping a user understand what licenses they need to operate in their city. They rely on "Help Center" articles.
  *   **LegalZoom/RocketLawyer:** Too expensive and disconnected from the daily operations platform. A user has to remember to go there.
  *   **OHC's Opportunity:** By integrating a "Legal & Compliance Department" (The Protector) directly into the core operating mesh, OHC can analyze the user's setup (e.g., selling food in NYC) and proactively push notifications like "Your NYC Mobile Food Vendor License expires in 30 days" or auto-generate a liability waiver when a customer books a high-risk service.

  ## Design Doc

  ### Business Journey Mapping (Fatima the Food Cart Operator)
  1.  **Onboarding:** Fatima signs up for OHC and sets her business type to "Food & Beverage" and location to "New York, NY".
  2.  **AI Department Coordination:**
      *   *Legal & Compliance Agent* references local NYC regulations for food carts.
      *   It creates a "Compliance Checklist" tailored to her profile.
  3.  **Proactive Alerts:** The Agent detects from her profile that she needs a Mobile Food Vending License. It sends a push notification with a link to the official city application portal.
  4.  **Contract Generation:** When Carlos (Handyman) sets up a $500 job, the *Legal Agent* automatically generates a custom service contract protecting him from liability, presented to the customer during the deposit payment flow.

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT_PROFILE ||--o{ COMPLIANCE_REQUIREMENT : tracks
      COMPLIANCE_REQUIREMENT ||--o{ DOCUMENT_TEMPLATE : uses
      TENANT_PROFILE ||--o{ SIGNED_CONTRACT : generates

      TENANT_PROFILE {
          string id
          string business_type
          string jurisdiction
      }
      COMPLIANCE_REQUIREMENT {
          string id
          string type
          string status
          date expiration_date
      }
      SIGNED_CONTRACT {
          string id
          string related_entity_id
          string pdf_url
          boolean signed
      }
  ```

  ### Mobile UX Flow (375px Viewport)
  1.  **Compliance Dashboard (Card Layout):** A sleek, translucent card under "Settings" titled "Shield". It shows a simple traffic light system (Green = All Good, Yellow = Action Needed).
  2.  **Actionable Alerts:** "Your Food Vendor License expires in 14 days." Tapping the alert opens an integrated view to upload the renewed document.
  3.  **Auto-Contract Review:** When creating a high-value invoice, a toggle switch says "Include Standard Service Contract". Tapping it shows a preview of the generated document.

  ### Performance & Zero Trust
  *   **Data Residency:** Compliance documents (licenses, signed contracts) contain PII and must be encrypted at rest and stored in isolated S3/MinIO buckets per tenant.
  *   **Audit Logging:** Every generated contract and compliance state change must be immutably logged in the tenant's ledger.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the core data model and API layer for the Proactive AI Tax, Licensing & Compliance Mesh. The system must allow the Legal & Compliance AI Agent to create, read, and update `ComplianceRequirement` records linked to a specific tenant. It must support generating a `DocumentTemplate` (e.g., a simple text contract) that can be attached to an Invoice or Booking flow.

  **Acceptance Criteria:**
  *   Database schema for Compliance Requirements and Generated Contracts with strict Row Level Security by `tenant_id`.
  *   An internal API endpoint for the Legal AI Agent to query a tenant's compliance health and inject new requirements.
  *   A generic Document Generation service that takes a tenant's profile and a template name to produce a basic PDF or HTML contract.
  *   All data access must respect the SPIFFE/SPIRE identity boundaries.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
