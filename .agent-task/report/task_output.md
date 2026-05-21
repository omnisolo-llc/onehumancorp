# Title: Integrate Zapier for No-Code SMB Workflow Automation

## Problem Statement
Small business owners—like Priya (Boutique Owner) and Carlos (Contractor)—rely on a patchwork of disconnected software tools (e.g., Google Sheets for leads, Mailchimp for newsletters, Trello for job tracking). Manually moving data between OHC and these external tools is tedious, error-prone, and consumes valuable hours each week that should be spent on growing their business. They need an automated, "set it and forget it" way to connect their OHC store and operations with the 5,000+ apps they already use, without writing a single line of code.

## Research Report

### Track 1: Dynamic Integration & Market Need Discovery
*   **Ecosystem Scraping:** An audit of competitor platforms (Shopify App Store, Squarespace Extensions) reveals that Zapier and Make.com are consistently among the top 10 most installed and highest-rated integrations. SMBs use them to bridge the gap between their core platform and niche operational tools.
*   **Community Mining:** On r/smallbusiness and ecommerce forums, workflow automation is heavily discussed. A common pain point is, "How do I get my new orders into my custom Google Sheet automatically?" or "How do I add new booking clients to my specific CRM?"
*   **Identify Integration Targets:** Zapier is the clear leader in the SMB space due to its brand recognition, ease of use, and massive app ecosystem (5,000+ apps). While Make.com is powerful, Zapier's UX is significantly more tailored to non-technical users like Maya or Carlos.

### Track 2: Selected Tool Deep-Dive Evaluation (Zapier)
*   **User-First Value Mapping:** For Carlos, integrating Zapier means when a client books an estimate in OHC, a Trello card is automatically created on his "New Jobs" board. For Priya, it means a new customer in OHC is instantly added to her Klaviyo VIP list. The value is pure time savings and elimination of manual data entry.
*   **Capabilities & Limits:** Zapier's Developer Platform allows OHC to build a private or public app integration. OHC would expose RESThooks (webhooks) for triggers (e.g., `Order Created`, `Customer Added`, `Booking Confirmed`) and REST API endpoints for actions (e.g., `Create Product`, `Update Inventory`). Zapier handles the polling or webhook reception reliably.
*   **SaaS Viability:**
    *   **Pricing:** Zapier offers a free tier (100 tasks/mo, single-step Zaps) which is sufficient for basic SMB needs. Paid tiers start at $19.99/mo for multi-step Zaps.
    *   **Cloud Mode:** In OHC's multi-tenant cloud, users authenticate via OAuth 2.0.
    *   **Standalone Mode:** For local deployments, users can use Zapier's API key authentication or Webhooks by Zapier to receive payloads from their private OHC instance, assuming their instance can make outbound HTTP requests.

### Track 3: Strategic Integration Dispatch

## Design Doc
*   **Triggers (OHC -> Zapier):** The OHC core event bus will emit secure, verified webhook payloads to Zapier when key domain events occur: `Order.Created`, `Customer.Registered`, `Booking.Confirmed`, `Inventory.Low`.
*   **Actions (Zapier -> OHC):** The OHC API will expose endpoints that Zapier can call to perform actions: `Create Customer`, `Update Inventory Level`, `Create Order`.
*   **User Interface:** A new "Automations" tab in the OHC admin dashboard. Users will see a "Connect Zapier" button which initiates the OAuth 2.0 flow. Once connected, they can click "Create a Zap" which deep-links them directly to the Zapier editor with OHC pre-selected. We will also display embedded Zapier templates (e.g., "Add OHC customers to Google Contacts").

## Implementation Prompt
*   **User-Facing Outcome:** A non-technical business owner can navigate to "Settings > Automations", click "Connect Zapier", and seamlessly link their OHC store. They can then use Zapier's drag-and-drop interface to automatically send their daily sales data to a Google Sheet or add new clients to their external CRM, saving hours of manual work.
*   **Acceptance Criteria:**
    *   Implement an OAuth 2.0 provider flow in OHC to allow Zapier to authenticate on behalf of a user.
    *   Develop a Zapier App (via Zapier Developer Platform) exposing at least 3 triggers (`Order Created`, `Customer Created`, `Booking Created`).
    *   Implement secure webhook dispatching in OHC that reliably delivers these events to Zapier's webhook URLs upon user subscription.
    *   Provide a basic UI in the OHC dashboard showing the Zapier connection status and a link to Zapier.

## Priority
P1

## Estimated Scope
Large
