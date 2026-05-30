issue_title: "Native Indian Payment Integration with Razorpay"
issue_description: |
  **Problem Statement:**
  Rohan (Handmade Crafts) in India cannot easily use Stripe for local customers who prefer UPI, RuPay, or local net banking. He needs a trusted local payment gateway that feels native to Indian customers, avoiding the high failure rates and friction associated with international payment processors in the Indian market.

  **Research Report:**
  - **Strategy**: Direct API integration with Razorpay.
  - **Target Persona**: Rohan (Indian SMB owner).
  - **Advantages**: Deep support for UPI (India's primary payment method), local cards, and net banking. Includes features like Razorpay Magic Checkout for higher conversion. Trusted by millions of Indian merchants.
  - **Risks**: Stringent regulatory KYC requirements in India for the merchant.
  - **Pricing**: Competitive local pricing (~2% per transaction for domestic).
  - **Ease of Use**: Indian customers are highly familiar with the Razorpay checkout interface.
  - **Compatibility**: Cloud & Standalone.

  **Design Doc:**
  - **Architecture Diagram:**
    ```mermaid
    sequenceDiagram
        actor User as Customer (India)
        participant UI as OHC Frontend (Mobile/Desktop)
        participant API as OHC Backend API
        participant Razorpay as Razorpay API

        User->>UI: Selects Razorpay as Payment Method
        UI->>API: Calls /checkout with payment method Razorpay
        API->>Razorpay: Creates Order via POST /v1/orders
        Razorpay-->>API: Returns Order ID
        API-->>UI: Returns Razorpay checkout url with pref_id
        UI->>User: Redirects to Razorpay Checkout
        User->>Razorpay: Completes Payment (UPI/Netbanking)
        Razorpay-->>API: Webhook event payment.captured
        API->>API: Updates OHC Database (Order status -> Paid)
    ```
  - **UI Wireframes (375px first):**
    - **Settings -> Finance:** Add a "Payment Providers" section with a toggle switch for "Razorpay (India)". When toggled, a modal requests API Key and API Secret.
    - **Checkout Flow:** If enabled and region is India, the checkout page displays a native Razorpay button or embedded widget allowing selection of UPI, Net Banking, or Cards.
  - **AI Agent Integration Points:**
    - The *Accountant* agent monitors `payment.captured` webhooks to automatically reconcile INR transactions and generate tax-ready GST reports.
  - **Key Design Decisions:**
    - Use the Razorpay `/v1/orders` endpoint to create a preference/order.
    - Implement `razorpay_webhook_handler` to securely verify signatures (`X-Razorpay-Signature`) before accepting updates.
    - Use `server_telemetry::record_api_call_cost` (using `tenant_id`) to track usage.

  **Implementation Prompt:**
  Implement the Razorpay payment integration for OHC.
  1. Add `RazorpayClient` in `src/server/integrations/razorpay/client.rs` that makes an authenticated API call to `https://api.razorpay.com/v1/orders` to create a checkout preference, and `https://api.razorpay.com/v1/payments` for creating a direct payment.
  2. The client must record telemetry using `server_telemetry::record_api_call_cost`.
  3. Update `RazorpayProvider` to expose `create_payment` and `handle_webhook` methods.
  4. Register the new methods in `IntegrationsRegistry` (`src/server/integrations/registry.rs`).
  5. Ensure the webhook handler properly transitions order status.

  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
