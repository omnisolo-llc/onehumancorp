issue_title: "Integrate Stripe Terminal (Tap to Pay) for In-Person Payments"
issue_description: |
  ### Title
  Integrate Stripe Terminal (Tap to Pay) for In-Person Payments

  ### Problem Statement
  Owners and operators who conduct business in person (like Carlos, the Field Service Owner, or Fatima, the Food Cart Operator) currently face friction when collecting payments. Relying purely on digital invoices (payment links) means they have to wait for the customer to open an email or SMS, click a link, and manually type in credit card details. This can lead to delayed payments and missed sales. They need a way to seamlessly accept payments right on their mobile device without requiring additional hardware card readers.

  ### Research Report
  - **Ecosystem Scraping & Competitors**: Point-of-sale systems like Square and Shopify POS dominate the in-person retail experience. However, a major trend in modern mobile operation platforms is the adoption of "Tap to Pay on iPhone" and "Tap to Pay on Android," which allow the merchant's standard mobile phone to act as a contactless payment terminal.
  - **Tool Evaluation (Stripe Terminal)**: Stripe offers the Tap to Pay SDK (Stripe Terminal) for both iOS and Android. It integrates deeply into the Stripe ecosystem, which OHC is already using for Checkout Sessions.
  - **User-First Value Mapping**: For a non-technical operator like Fatima, this means she can just tap a "Collect Payment" button on the OHC mobile app, hold out her phone, and let the customer tap their credit card or Apple Pay to complete the transaction instantly. No dongles, no extra apps.
  - **SaaS Viability**: Stripe Terminal charges a per-transaction fee (typically around 2.7% + 5c) but has no upfront hardware costs or monthly SaaS fees for the Tap to Pay software, making it highly viable for small businesses. It operates beautifully in multi-tenant cloud environments.
  - **Capabilities & Limits**: The integration requires native mobile SDKs (Stripe Terminal React Native or Flutter SDK, or native Swift/Kotlin) since it relies on secure NFC hardware access.

  ### Design Doc
  - **Trigger**: When an owner views an active quote, invoice, or order on the mobile app, a new primary action "Tap to Pay" will be visible.
  - **Action**: Tapping the button initializes the Stripe Terminal SDK on the device, communicates with the OHC backend to fetch a PaymentIntent secret, and launches the native OS Tap to Pay sheet.
  - **User Experience**: The customer taps their card. Upon success, the OHC app immediately reflects the invoice as "Paid" and records the transaction in the finance summary.
  - **Backend Integration**: The OHC backend will need new API endpoints to generate Stripe Terminal connection tokens and to create PaymentIntents specifically flagged for Terminal capture.

  ### Implementation Prompt
  Implement the backend API endpoints to support Stripe Terminal and integrate the Stripe Terminal SDK into the OHC Flutter app. The outcome must allow a mobile user (owner) to open a pending invoice and successfully process an in-person NFC payment using their phone's Tap to Pay capability. Ensure that the invoice status updates to paid in real time and the transaction is recorded properly.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
