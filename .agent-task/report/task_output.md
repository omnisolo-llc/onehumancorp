issue_title: "Agentic Autonomous Booking & Scheduling System"
issue_description: |
  # Research Report: Agentic Autonomous Booking & Scheduling System

  ## Problem Statement
  Service-based SMBs (like Carlos the Handyman and Leo the Music Tutor) struggle with managing bookings across multiple platforms (Instagram, phone calls, website). They waste significant time negotiating timeslots, collecting deposits, and managing no-shows. Existing tools (Calendly, Acuity) are too generic, often requiring users to jump between a calendar app and a payment app, creating friction and lost leads.

  ## Research Report: Market Mapping & Competitor Discovery
  - **Calendly / Acuity Scheduling**: Dominant players, but function mainly as link-sharing utilities. They lack deep integration into the business's operational flow and require the user to configure complex rules manually.
  - **Shopify / Square**: Shopify focuses primarily on physical goods. Booking apps exist but are third-party and feel disconnected. Square has a solid booking system but is rigid and lacks conversational AI booking capabilities.
  - **The OHC Opportunity**: Integrate an autonomous booking agent that handles the entire flow—from inquiry parsing in a chat interface, offering available slots, collecting a deposit, and confirming the appointment—without the owner lifting a finger.

  ## Design Doc: High-Level Architecture

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Customer - Web/Mobile/Chat] --> APIGateway[API Gateway]
      APIGateway --> TriageAgent[Work Triage Agent]
      TriageAgent -- "Identifies Booking Intent" --> BookingAgent[Operations Assistant - Booking]
      BookingAgent --> CalendarService[Calendar & Availability Service]
      CalendarService --> PostgreSQL[(PostgreSQL - Central Ledger)]
      BookingAgent --> PaymentService[Payment & Deposit Service]
      PaymentService --> Stripe[Stripe API]
      BookingAgent -- "Notification/Confirmation" --> CustomerSuccessAgent[Customer Assistant]
      CustomerSuccessAgent --> Client
  ```

  ### Mobile UX Flow (375px)
  1.  **Customer View**: Customer visits the OHC-powered link-in-bio or storefront.
  2.  **Conversational Booking**: Instead of a complex calendar grid, the interface can be a chat window: "Hi, I need a leaky pipe fixed on Tuesday."
  3.  **Agent Response**: The agent responds immediately: "Hi! Carlos is available on Tuesday at 10 AM or 2 PM. Which works better?"
  4.  **Selection & Deposit**: Customer taps a pill button for "10 AM". The chat presents an inline Stripe payment module: "Great, please secure your slot with a $50 deposit."
  5.  **Owner View (The Agent Feed)**: Carlos opens his OHC app (375px). A card appears: "New Booking: Leaky Pipe at 10 AM Tuesday. $50 deposit collected." with a single "Acknowledge" button.

  ### AI Agent Integration Points
  -   **Work Triage Agent**: Parses incoming messages (from SMS, WhatsApp, Web) to identify booking intent.
  -   **Operations Assistant (Booking Module)**: Has strict boundaries to only offer timeslots based on the `CalendarService` availability. It handles the back-and-forth negotiation.
  -   **Customer Assistant**: Handles reminders (24 hours before) and post-service follow-ups (requesting reviews).

  ### Key Design Decisions
  -   **Conversational First, Grid Second**: Allow customers to book via natural language, falling back to a calendar grid only if they prefer it.
  -   **Integrated Deposits**: Deposits are a first-class feature of a booking, not an afterthought.

  ## Implementation Prompt
  **Objective:** Implement the core `CalendarService` and integrate it with the `Operations Assistant` to allow for basic text-based booking slot retrieval and reservation.

  **Critical User Journey (CUJ):**
  1.  A user (acting as a customer) sends a message via the OHC web chat: "Are you free tomorrow afternoon?"
  2.  The system identifies this as a booking inquiry.
  3.  The `Operations Assistant` queries the `CalendarService` for the tenant's availability for "tomorrow afternoon".
  4.  The system replies with formatted text and inline buttons for the available slots (e.g., 1 PM, 3 PM).
  5.  The customer clicks "1 PM".
  6.  The system provisionally locks the slot and prompts for a deposit (if configured).

  **Acceptance Criteria:**
  -   Create a `CalendarService` (Go/PostgreSQL) capable of storing availability rules and checking free/busy status.
  -   Extend the `Operations Assistant` prompt/tools to query this service via function calling.
  -   Ensure the chat UI (Flutter/PWA) correctly renders the agent's response, especially if the agent returns structured data for available slots.
  -   Implement a distributed lock (Redis Redlock) when a slot is selected to prevent double-booking before deposit confirmation.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
