issue_title: "[research] Invisible OHC Mobile Wallet Pass Engine"
issue_description: |
  # Architecture Brief: Invisible OHC Mobile Wallet Pass Engine

  ## Problem Statement
  Small business owners like Priya (Boutique) and Maya (Baker) struggle to maintain customer loyalty and facilitate quick in-person identification. Physical punch cards get lost, and forcing customers to download a bespoke app for a single boutique introduces too much friction. They need a zero-friction way to stay in their customers' pockets.

  ## Research Report
  - **Market Context**: Apple Wallet and Google Wallet passes have a 95% retention rate compared to standard mobile apps.
  - **The Gap**: Existing solutions (like PassKit or standard Shopify plugins) require the merchant to manually design passes, manage API keys, and explicitly prompt customers. This violates the "Grandmother Test."
  - **OHC Differentiation**: We need an *invisible* engine. When a customer makes their first purchase, OHC automatically generates a personalized, branded Wallet Pass (acting as a digital receipt, loyalty card, and booking identifier) and includes it via an Apple/Google Wallet link in the standard confirmation SMS/Email.
  - **Location Awareness**: Apple Wallet passes support geo-fencing. If Priya sets her store location in OHC, the customer's lock screen will automatically surface the pass when they walk into the boutique, enabling instant tap-to-pay recognition.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Checkout Event] --> B{Wallet Pass Engine}
      B --> C[Fetch Merchant Branding]
      B --> D[Generate PKPass/JWT]
      D --> E[Store Pass Metadata in Postgres]
      E --> F[Inject Pass Link to Confirmation SMS]
      F --> G[Customer Adds to Apple/Google Wallet]
      G --> H{Location/Event Trigger}
      H --> I[Pass surfaces on Lock Screen]
      I --> J[Merchant scans QR/NFC at POS]
  ```

  ### Implementation Strategy
  1.  **Pass Generation Service**: A Rust microservice within the API that uses the `pkpass` (Apple) and `google-auth` (Google Pay) specifications to dynamically generate passes.
  2.  **Branding Injection**: The service automatically pulls the merchant's logo, brand colors, and name from the `tenants` table to style the pass without user configuration.
  3.  **Dynamic Content**: The pass contains a dynamic QR code linked to the customer's unique ID (`customer_profile.id`). For service businesses, it can display the next appointment time.
  4.  **Distribution**: The generated download link is automatically appended to the post-purchase transactional SMS or email handled by the Customer Success Agent.
  5.  **POS Integration**: The OHC Merchant App (running on the merchant's phone) must include a scanner to read the Wallet Pass QR code, instantly pulling up the customer's profile, loyalty points, and open orders.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the "Invisible OHC Mobile Wallet Pass Engine."

  **Critical User Journey (CUJ):**
  1. A customer completes a purchase on Maya's OHC-powered storefront from their mobile browser.
  2. The confirmation screen and subsequent email include a prominent "Add to Apple Wallet" / "Add to Google Wallet" button.
  3. Tapping the button downloads a branded pass.
  4. When the customer visits Maya's physical location for pickup, the pass appears on their lock screen.
  5. Maya scans the pass using the OHC Merchant App, instantly verifying the pickup order.

  **Acceptance Criteria:**
  - Rust implementation of the `.pkpass` generation conforming to Apple's specifications.
  - Integration with the checkout flow to surface the pass link.
  - Zero-configuration required from the merchant (branding is auto-inferred).
  - Include tests verifying pass generation payload and signature creation.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
