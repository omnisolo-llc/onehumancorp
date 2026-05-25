issue_title: "[architecture] Zero-Friction Client Portal and Document Vault"
issue_description: |
  # [architecture] Zero-Friction Client Portal and Document Vault

  ## Problem Statement
  Service-based small business owners like Carlos (handyman) and Leo (tutor) struggle with sharing sensitive or important documents (quotes, invoices, warranties, progress reports, sheet music) with their clients. Currently, they rely on chaotic email threads, unsecure WhatsApp attachments, or complex third-party tools that require clients to create accounts, remember passwords, and install new apps. This friction leads to lost documents, delayed payments, and a poor, unprofessional customer experience. Clients want instant, secure access to their history with the business, and owners need a zero-maintenance way to provide it.

  ## Research Report
  *   **Shopify:** Highly optimized for physical product eCommerce. Customer accounts exist but are heavily geared toward order history and address management, not bespoke document sharing or interactive service portals.
  *   **Wix / Squarespace:** Offer basic members areas, but they require the client to go through a traditional username/password registration flow. The UX is often clunky and not natively integrated with quoting or booking flows.
  *   **Stripe Customer Portal:** Excellent for managing subscriptions and payment methods, but lacks general document vaulting (e.g., sharing a PDF warranty or a custom design mockup).
  *   **OneHumanCorp (OHC) Differentiation - "Zero-Friction Access":** OHC implements a magic-link, passwordless portal. When an owner sends a quote, invoice, or file, the client receives a secure, time-bound or device-bound link. The portal acts as a unified hub for all interactions (payments, documents, past bookings) without ever asking the client to "Sign Up". The AI departments handle organizing and tagging documents automatically.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT_OWNER ||--o{ DOCUMENT_VAULT : "Uploads/Generates"
      AI_OPERATIONS_DEPT ||--o{ DOCUMENT_VAULT : "Auto-tags & Organizes"

      DOCUMENT_VAULT {
          string vault_id
          string tenant_id
          string customer_profile_id
          json metadata
      }

      DOCUMENT_VAULT ||--o{ SECURE_MAGIC_LINK : "Exposed via"

      SECURE_MAGIC_LINK {
          string token
          datetime expires_at
          boolean single_use
      }

      SECURE_MAGIC_LINK }|--|| CLIENT_DEVICE : "Authenticates"
      CLIENT_DEVICE ||--o{ CLIENT_PORTAL_UI : "Accesses"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). Optimized for the end-client viewing on their phone.
  *   **App Bar:** Blurred glass top nav with the Business Logo (e.g., "Carlos Handyman Services") and a subtle "Powered by OHC" watermark.
  *   **Dashboard View (Client Facing):**
      *   **Action Required Card:** A highlighted frosted glass card at the top for immediate needs (e.g., "Review Quote #1042" or "Pay Deposit").
      *   **History & Documents:** A clean, vertical list of categorized items (Invoices, Warranties, Files).
      *   **Direct Message Button:** A floating action button (FAB) that opens a direct channel to the business owner's Unified Inbox.
  *   **Owner View (OHC App):**
      *   Within the customer's profile, a "Vault" tab shows all shared documents.
      *   1-Tap sharing: "Generate Magic Link for Client" creates an SMS or WhatsApp draft instantly.

  ### Mobile UX Flow
  1. **Trigger:** Carlos finishes a repair and uploads a PDF warranty via the OHC app. The AI tags it as "Warranty" and links it to the client's profile.
  2. **Delivery:** The client receives an SMS: "Carlos added a Warranty to your file. View it here: [Magic Link]".
  3. **Access:** The client taps the link. It opens the mobile web portal instantly—no passwords.
  4. **Interaction:** The client views the warranty, sees their past invoices, and can even tap "Request New Quote" right from the portal.

  ### AI Agent Integration Points
  *   **Operations Department:** Automatically categorizes uploaded files (e.g., identifying a PDF as an "Invoice" or an image as "Before/After Photo").
  *   **Customer Success Department:** If the client replies to the magic link SMS, the CS agent routes the message into the Unified Inbox, maintaining context of what document the client was just viewing.

  ### Key Design Decisions (Why, not How)
  *   **Passwordless by Default:** Eliminating client registration is crucial. Magic links via SMS/Email remove the #1 barrier to portal adoption.
  *   **Unified Client Profile:** The vault is not a standalone storage drive; it is deeply bound to the CRM's Customer Profile, ensuring every invoice, booking, and file is contextualized.
  *   **Multi-tenant Edge Caching:** The client portal must load instantly worldwide. The architecture should leverage edge computing to serve the UI quickly while retrieving sensitive documents via secure, signed URLs.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the underlying architecture and API surface for the "Zero-Friction Client Portal and Document Vault".

  **Customer User Journey (CUJ):**
  1. A small business owner uploads a file to a customer's profile.
  2. The system generates a secure, passwordless magic link.
  3. The client opens the link and accesses a mobile-optimized web view showing their full history with the business (documents, invoices, past appointments).

  **Acceptance Criteria:**
  *   **Mobile Parity:** The client-facing portal must be perfectly usable on a 375px viewport, adhering to the Translucent Glass design tokens.
  *   **Zero-Trust Security:** Access to the vault must be strictly authenticated via the magic link token, with proper tenant isolation (`tenant_id` verification) to ensure clients only see their own files.
  *   **No Registration Flow:** The client must never see a "Create Account" or "Forgot Password" screen.
  *   **API Surface:** Provide the necessary endpoints to generate magic links, invalidate tokens, and list vault contents.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
