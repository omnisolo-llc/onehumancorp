issue_title: "Automated Agentic Setup Protocol for Real Estate Operators"
issue_description: |
  # Research Report: Automated Agentic Setup Protocol for Real Estate Operators

  ## 1. Executive Summary
  This report investigates an architectural gap in OneHumanCorp (OHC): the lack of an automated, vertical-specific setup flow for Real Estate Operators (property managers, independent agents). Currently, setting up complex workflows (e.g., property listings, showing schedules, applicant screening, rent collection) requires manual configuration, which alienates non-technical operators and delays time-to-value. By implementing a "Zero-Click" or "Conversational" Agentic Setup Protocol for Real Estate, OHC can instantly provision a tailored workspace, differentiating itself from generic CRM or website builders.

  ## 2. Market Mapping & Competitor Discovery
  - **Traditional Generic CRMs (HubSpot, Salesforce):** Powerful but highly complex; require extensive customization and often expensive consultants to map real estate workflows.
  - **Specialized Real Estate Software (Buildium, AppFolio, kvCORE):** Vertical-specific but often monolithic, expensive, and inflexible for hybrid operators (e.g., someone who manages short-term rentals and long-term leases).
  - **Website Builders (Wix, Squarespace):** Good for simple portfolios but lack native, robust property management, showing scheduling, and tenant application workflows without cobbling together third-party plugins.
  - **OHC Opportunity:** Leverage AI to dynamically generate the data schema, UI views, and automated workflows tailored precisely to the operator's description of their business in minutes, combining the simplicity of a website builder with the power of a customized CRM.

  ## 3. OHC Gap & Pain Point Identification
  - **Persona Focus:** "Elena" (Independent Property Manager managing 15 units).
  - **The Gap:** Elena wants to transition from spreadsheets to a software platform. In traditional platforms, she has to manually create custom fields for "Rent Amount," "Lease Expiry," set up calendar integrations for showings, and create web forms for applications.
  - **Pain Point:** The setup paralysis prevents adoption. The cognitive load of translating her real-world business into software primitives (databases, forms, webhooks) is too high.

  ## 4. Deep Dive Architecture Design

  ### Data Model & Dynamic Provisioning
  The system must support dynamic schema generation based on natural language input.
  - **Setup Agent ("The Architect"):** An LLM-powered agent that converses with the user during onboarding. It translates user intent ("I manage long-term residential rentals in Chicago") into a structured tenant configuration.
  - **Dynamic Schema Execution:** The Architect generates a JSON representation of the required data entities (e.g., `Property`, `Unit`, `Tenant`, `Lease`, `MaintenanceRequest`).
  - **Provisioning Engine:** A backend service that takes the JSON schema and provisions the necessary PostgreSQL tables/columns (using EAV or dynamic JSONB columns for flexibility within a multi-tenant structure) and default UI views.

  ### AI Agent Coordination
  - **The Architect (Setup):** Handles the initial provisioning.
  - **Operations Agent ("The Manager"):** Automatically configures showing schedules based on unit availability and syncs with the operator's calendar.
  - **Customer Success Agent ("The Ambassador"):** Configured to automatically reply to showing inquiries, qualify leads based on income/credit score requirements, and schedule the showing.

  ### Mobile-First Implementation
  - **Conversational Onboarding (375px):** The setup process is a chat interface, not a massive form. "Tell me about your properties..."
  - **Dashboard Generation:** The output is a pre-configured mobile dashboard with immediately useful metrics (Occupancy Rate, Pending Maintenance, Upcoming Showings).

  ## 5. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** Conversational Agentic Setup for Real Estate Operators

  **Target Persona:** Elena the Property Manager

  **Outcome:** Elena signs up, tells the AI she manages 15 apartments, and within 3 minutes, she has a fully configured OHC workspace with a property listing site, an applicant intake form, and an automated showing scheduler, all accessible on her phone.

  **Critical User Journey (CUJ):**
  1. Elena signs up on her mobile device.
  2. The onboarding screen presents a chat interface: "Welcome to OHC. What kind of business do you run?"
  3. Elena replies: "I manage 15 long-term apartment rentals."
  4. The Setup Agent (The Architect) infers the necessary data structures and workflows. It asks a clarifying question: "Do you want to handle maintenance requests through the app?"
  5. Elena says "Yes."
  6. The backend provisions the data schema (Properties, Units, Leases, Maintenance) and configures the Operations and Ambassador agents.
  7. Elena is dropped into a fully customized mobile dashboard showing her empty property list, ready to add her first unit.

  **Next Actions for Engineering:**
  - **Step 1:** Develop the 'Setup Agent' (The Architect) using Gemini Pro to parse business descriptions into a standardized JSON configuration schema (entities, fields, agent workflows).
  - **Step 2:** Implement the backend Provisioning Engine that consumes the JSON schema and configures the tenant's workspace (e.g., updating JSONB schema definitions, setting up default agent prompts).
  - **Step 3:** Build the conversational UI for the mobile onboarding flow (375px), replacing the legacy static wizard.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
