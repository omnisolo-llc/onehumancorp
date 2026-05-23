# [architecture] Invisible AI Intake and Diagnostic Engine

## Title
Invisible AI Intake and Diagnostic Engine

## Problem Statement
Service-based businesses (like Carlos the handyman, or Maya the baker making custom cakes) waste immense time going back and forth with customers to gather required details. A customer texts "my sink is leaking" or "I need a cake for my daughter." The business owner then has to ask 5 follow-up questions: "What kind of sink?", "Can you send a picture?", "What size cake?", "Any allergies?", etc. This is manual, slow, and leads to lost leads if the business owner is busy or asleep. The customer journey is full of friction.

## Research Report
*   **Shopify/Wix:** Rely on static forms (e.g., "Contact Us" or complex custom product variants). These forms are rigid, often ignored by customers, and cannot adapt dynamically based on previous answers or analyze uploaded images.
*   **ServiceTitan/Jobber:** Have intake forms, but they are often highly structured and meant for dispatchers, not for a seamless, consumer-friendly conversational flow.
*   **OHC Differentiation - "Conversational & Vision AI Intake":** Instead of static forms, OHC uses an invisible Intake Agent. When a customer starts an inquiry (via Web, SMS, WhatsApp, or IG), the agent dynamically asks the right questions. It utilizes Vision AI: "Can you snap a picture of the leak?" and immediately extracts the brand/model or assesses the scope of work, packaging all of this into a neat "Diagnostic Brief" for the merchant.

## Design Doc

### Architecture Diagram
```mermaid
erDiagram
    CUSTOMER_INQUIRY ||--o{ INTAKE_SESSION : "Initiates"

    INTAKE_SESSION {
        string session_id
        string tenant_id
        string customer_profile
        string channel (SMS, Web, IG)
        json extracted_requirements
    }

    INTAKE_SESSION ||--|| DIAGNOSTIC_AGENT : "Managed by"

    DIAGNOSTIC_AGENT ||--o{ VISION_MODEL : "Calls for image analysis"
    DIAGNOSTIC_AGENT ||--o{ KNOWLEDGE_BASE : "Consults (Pricing, Inventory, Skills)"

    INTAKE_SESSION }|--|| MERCHANT_DASHBOARD : "Generates Diagnostic Brief"

    MERCHANT_DASHBOARD {
        string status "Ready for Quote / Requires Human"
        boolean actionable
    }
```

### UI Wireframes & 375px Baseline
**Customer Facing (Any Channel / Web Storefront Widget)**
*   A chat-like interface or guided SMS thread. "Hi! To get you an accurate quote for the sink repair, could you send a quick photo of under the sink?"
*   Uploading an image immediately triggers the Vision Model.
*   Agent replies: "Thanks! Looks like a Moen standard fitting. Do you also need us to replace the faucet, or just fix the leak?"

**Merchant Facing (OHC Mobile App - 375px)**
*   **App Bar:** "New Lead: Sink Repair"
*   **Diagnostic Card:** A frosted glass card (`rgba(255, 255, 255, 0.05)`, `backdrop-filter: blur(10px)`).
*   **Summary Section:** Bullet points extracted by AI:
    *   *Issue:* Leaking P-Trap.
    *   *Brand:* Moen (identified from photo).
    *   *Customer Availability:* Tomorrow morning.
*   **Action Row:** Large, touch-friendly buttons (≥ 44x44px): `[Generate Quote ($150)]` `[Message Customer]` `[Decline]`.

### Mobile-First & Zero Trust Isolation
*   **Offline Capability:** The Merchant's Diagnostic Card is synced via SQLite mesh. They can review the brief and tap "Generate Quote" offline. The quote is queued and sent when connectivity is restored.
*   **Tenant Isolation:** The Intake Session explicitly binds to the `tenant_id` ensuring the AI only utilizes that specific merchant's knowledge base and pricing rules.

## Implementation Prompt
**To Implementer Agent:**
Implement the `IntakeSession` data model with strict multi-tenant isolation. Create the orchestrator logic for the `DiagnosticAgent` that interfaces with the existing multi-modal LLM capabilities to parse incoming messages and images. On the frontend, build the `DiagnosticCard` component using the defined design tokens (translucent glass materials, Ubiquiti modular layouts) ensuring a 100% mobile-first experience on a 375px viewport with ≥ 44x44px touch targets. Ensure offline support via optimistic UI updates when the merchant approves or modifies an AI-generated quote from the diagnostic brief.

## Priority
P1

## Estimated Scope
Large
