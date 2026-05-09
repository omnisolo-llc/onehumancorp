# OHC Architecture Research Report: End-to-End Business Journey

## Executive Summary
This research investigation focused on defining the architectural foundations for the end-to-end user journey within OneHumanCorp (OHC). Our key finding is that current onboarding and operational flows must be entirely redesigned to meet the "Grandmother Test" and the core product vision: allowing a non-technical user to launch a live business from their mobile device in under 10 minutes.

The resulting architectural brief defines the overarching strategy for Acquisition, Onboarding, Activation, Retention, Revenue, and Referral across all five core user personas (Maya, Carlos, Priya, Leo, Fatima).

## Key Findings & Friction Points
1.  **Cognitive Overload during Onboarding:** Requesting too much setup information upfront (e.g., complex shipping rules, DNS records) causes immediate user drop-off.
2.  **Payment Gateway Integration:** The technical jargon and multi-step process required to connect gateways like Stripe stalls the "Activation" milestone.
3.  **Inventory & Calendar Syncing:** Manually mapping real-world physical inventory or service availability to digital systems without intuitive AI assistance is a major barrier for non-technical users.
4.  **Mobile Limitations:** Traditional SaaS platforms assume desktop usage for initial configuration, violating OHC's mobile-first mandate.

## Architectural Design Overview
The design doc addresses these friction points with three key strategies:

1.  **AI-First Progressive Profiling:** The onboarding wizard requests minimal input (e.g., "What do you sell?"). "The Promoter" AI Agent extrapolate this into a functional storefront draft. Advanced configurations (custom domains, complex shipping) are deferred until suggested by "The Advisor" AI agent post-activation.
2.  **Mobile-First Exclusively (375px baseline):** The entire onboarding and operation flow is constrained to a 375px viewport with native mobile UI patterns.
3.  **Autonomous Activity Feed:** Post-activation, the home dashboard shifts from a configuration screen to an "Agent Activity Feed," where background AI agents present 1-tap approval tasks (e.g., "Approve Quote", "Send Fulfillment Email").

### Persona Sequence Diagrams
*(Detailed Mermaid.js sequence diagrams are available in the formal Issue Brief)*
-   **Maya (Physical Products):** Flow focuses on instant storefront generation from minimal text prompt, followed by AI-assisted Instagram DM replies.
-   **Carlos (Services/Bookings):** Flow focuses on generating a booking calendar and AI-drafted quote approvals.
-   **Priya (Omnichannel):** Flow focuses on image-to-inventory extraction and multi-store tier upgrade paths.
-   **Leo (Subscriptions):** Flow focuses on automated meeting link generation and recurring revenue retention.
-   **Fatima (Offline-First):** Flow focuses on QR code physical integration and ultra-simple audio notifications for pre-orders.

## Next Steps
An architectural issue brief has been submitted to `docs/research/[architecture]_business_journey_architecture.md`.

The primary action item for the implementer swarm is to construct the mobile-first UI wizard that guides a user through the initial setup (deferring advanced configs) and integrates the AI agent handoffs for instant storefront generation. Playwright E2E tests will be required to verify a successful run-through from login to the generated storefront.
