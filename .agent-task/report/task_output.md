issue_title: "Implement Universal AI Voice Receptionist Engine"
issue_description: |
  # Universal AI Voice Receptionist Engine

  ## Problem Statement
  Small business owners like Carlos (handyman) and Fatima (food cart) are highly operational and hands-on. They are frequently driving, cooking, or on a job site, meaning they miss phone calls. Every missed call is a missed booking, a lost order, or a dissatisfied customer. Existing solutions (voicemail, expensive answering services) either create friction for the customer or are completely disconnected from the business's actual inventory and calendar. Our personas need an intelligent voice receptionist that answers calls in real-time, speaks naturally (and in multiple languages, like Arabic for Fatima), references live inventory/calendar availability, and can book appointments or take pre-orders autonomously.

  ## Research Report
  - **Market Reality:** 62% of small business customer calls go unanswered. Missed calls cost the SMB market billions annually.
  - **Competitor Analysis:**
    - **Shopify / Wix / Squarespace:** No native voice capabilities. They rely entirely on web/mobile storefronts, which doesn't solve the problem for service-heavy or food businesses where customers prefer to call.
    - **Standalone AI Voice Products (Bland AI, Vapi, Retell):** Highly capable but require deep API integrations. A non-technical user cannot hook up Vapi to their square inventory.
  - **The OHC Opportunity:** By tightly integrating a conversational AI voice agent directly with OHC's Universal Capacity and Inventory Ledger, we give every SMB a 24/7 receptionist out of the box. The agent "knows" Carlos's pricing and schedule. It "knows" when Fatima is sold out of lamb over rice.
  - **Key Requirements:**
    - **Omnilingual:** Support for multiple languages (crucial for diverse SMB owners and their customers).
    - **Zero-Latency Feel:** Sub-500ms conversational latency.
    - **Action-Oriented:** The agent doesn't just talk; it creates OHC state changes (books a slot, triggers a quote, records a pre-order).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Caller[Customer Mobile Phone] --> PSTN[PSTN / Twilio Ingress];
      PSTN --> Vapi[Voice AI Gateway]
      Vapi -->|WebSocket Stream| AgentCore[OHC Voice Orchestration Agent];

      subgraph OHC Zero Trust Boundary
          AgentCore -->|Read/Write| Ledger[Universal Capacity & Inventory Ledger];
          AgentCore -->|Read| CRM[CRM & Customer Context];
          AgentCore -->|Trigger| Notify[Mobile Push Notification Engine];
      end

      Ledger -.-> Carlos[Carlos's Mobile App];
      Notify --> Fatima[Fatima's OHC App];
  ```

  ### Data Model & Invariants
  ```mermaid
  erDiagram
      Tenant ||--o{ VoiceSession : owns
      Tenant ||--o{ PhoneNumber : provisions
      PhoneNumber ||--|| VoiceGatewayConfig : maps_to
      VoiceSession ||--|| LedgerTransaction : initiates

      Tenant {
          string id
          string business_name
          string default_language
      }
      PhoneNumber {
          string number
          string status
      }
      VoiceGatewayConfig {
          string provider
          string webhook_url
          string llm_prompt_hash
      }
      VoiceSession {
          string session_id
          string caller_number
          string status
          datetime start_time
          string transcript_ref
      }
  ```
  **Invariants:**
  - `VoiceSession` is strictly scoped to a single `Tenant` via the incoming `PhoneNumber`.
  - AgentCore operates under SPIFFE/SPIRE Zero Trust, meaning it must acquire a short-lived token scoped only to the `Tenant` before accessing the `Ledger`.

  ### AI Department Coordination
  - **Operations Department:** Responsible for querying the `Ledger` during the call to confirm availability (e.g., "Do we have time for a 2 PM appointment?").
  - **Customer Service (CS) Department:** Maintains the long-term memory of the caller via the `CRM` (e.g., "Ah, Mr. Smith, calling about the cake again?").
  - **Finance Department:** Triggered if a quote or deposit is requested during the call, synthesizing an instant localized invoice link sent via SMS post-call.

  ### UX & Mobile Flow (375px First)
  1. **Settings / Receptionist Tab (Owner View):**
     - Clean, macOS-style Translucent Glass card: "AI Voice Receptionist: [ON/OFF] toggle".
     - **Language Dropdown:** Default English. Add secondary language (e.g., Arabic, Spanish).
     - **Business Rules Card:** "What can the receptionist do?"
       - [x] Take bookings
       - [x] Give quotes
       - [x] Answer FAQs
     - **Voice Persona:** Tap to listen to 3 options (Friendly, Professional, Casual).
  2. **Post-Call Mobile Notification (Actionable):**
     - Push notification arrives: "New Booking via Voice: John Smith, Friday at 2 PM".
     - Tapping it opens a summary card showing the call transcript and the auto-generated calendar event.

  ### Performance & Security Targets
  - **Latency:** Voice gateway to OHC backend response must be < 400ms to maintain conversational fluidity.
  - **Security:** Voice Webhooks must strictly validate incoming signatures from the Voice AI Gateway. The backend service processing the WebSocket stream must run in a minimal-privilege container isolated from the main Postgres database, communicating only via gRPC with the specific multi-tenant data layer.

  ### Key Design Decisions
  - **Bring-Your-Own-Number (BYON) or Provisioned:** Provide an OHC-provisioned number by default but allow porting.
  - **Strict Multi-Tenancy:** The Voice AI Gateway context is strictly scoped by the ingress phone number mapping to the OHC tenant ID.
  - **No Complex Configuration:** Maya/Carlos do not write prompts. They select their business goals, and the system synthesizes the prompt dynamically using the business's existing catalog, FAQs, and settings.

  ## Implementation Prompt
  **Objective:** Implement the backend and mobile UI for the Universal AI Voice Receptionist Engine.
  **Context:** When a user toggles the receptionist "ON", the system must provision a Twilio number (or use an existing one), hook it up to our preferred Voice LLM gateway (e.g., Vapi/Retell), and route webhooks into the `AgentCore`.
  **User Journey (CUJ):**
  1. Carlos navigates to the "Receptionist" tab on his Android app.
  2. He toggles it ON, selects a "Professional" voice, and enables "Bookings".
  3. A customer calls Carlos's new OHC number. The AI answers immediately, checks Carlos's availability in the Ledger, and books a 2 PM slot.
  4. Carlos receives a push notification with the new booking.
  **Acceptance Criteria:**
  - The mobile UI uses standard OHC design tokens (glass materials, modular cards) and fits perfectly on a 375px screen.
  - The voice agent can successfully read availability from the Universal Capacity Ledger.
  - The voice agent can successfully write a new appointment to the Ledger.
  - Tenant isolation is guaranteed; the agent only accesses data for the specific business owner.
  - Provide integration tests mocking the PSTN/Voice Gateway webhook flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
