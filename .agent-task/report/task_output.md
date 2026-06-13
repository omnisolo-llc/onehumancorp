issue_title: "Integration: Stripe Tap to Pay on Mobile for In-Person Payments"
issue_description: |
  **Title**: Integration: Stripe Tap to Pay on Mobile for In-Person Payments

  **Problem Statement**:
  Priya (Boutique Operator) and Carlos (Field Service Owner) frequently interact with customers in person and need a frictionless way to collect payments on the spot. Currently, requiring physical card terminals or sending payment links creates friction and delays in capturing revenue. They need to turn their existing mobile devices into payment terminals without purchasing additional hardware, allowing them to accept contactless payments instantly while syncing seamlessly with their OHC workspace.

  **Research Report**:
  - **Ecosystem Scraping & Community Mining**: Research indicates that "Tap to Pay on iPhone" and "Tap to Pay on Android" are among the most requested features for small business point-of-sale systems. Competitors like Square, Shopify POS, and Wix all offer Tap to Pay natively on mobile devices.
  - **Selected Tool**: Stripe Tap to Pay on Mobile (Stripe Terminal).
  - **Capabilities & Limits**:
    - Enables accepting in-person contactless payments directly on an NFC-equipped Android or iPhone device without extra hardware.
    - Uses Stripe Terminal SDK which seamlessly integrates with Flutter natively.
    - Cloud viability: Works perfectly in a multi-tenant cloud setup via Stripe Connect / standard Stripe accounts.
  - **SaaS Viability**: No additional hardware costs. Standard Stripe processing fees apply (typically 2.9% + 30¢ for card not present, but Tap to Pay offers competitive card-present rates like 2.7% + 5¢).
  - **Ease of Use for Owners**: Zero setup friction for hardware. Owners just enable the feature on their phone and hold it out to the customer.

  **Design Doc**:
  - **Trigger**: In the OHC mobile app (Flutter), when Priya or Carlos creates a quick sale or closes an invoice, they select "Tap to Pay" as the payment method.
  - **Action**: The OHC app invokes the Stripe Terminal SDK to activate the device's NFC reader.
  - **User Feedback**: The screen displays a clear, native contactless payment prompt. Once the customer taps their card or device, a success animation plays, and the payment is immediately logged against the tenant's OHC revenue dashboard.
  - **Fallback**: If NFC fails, it seamlessly falls back to a QR code payment link.

  **Implementation Prompt**:
  Implement Stripe Terminal SDK into the OHC Flutter mobile app to support Tap to Pay.
  Acceptance Criteria:
  1. A "Tap to Pay" option appears on payment collection screens in the mobile app.
  2. Selecting the option triggers the native NFC payment reader (iOS/Android).
  3. Successful payments automatically record in the OHC backend via a webhook and update the related task/invoice to "Paid".
  4. The UI gracefully handles disabled NFC or unsupported devices by offering a QR code alternative.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
