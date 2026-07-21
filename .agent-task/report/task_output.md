issue_title: "Agentic Field Service Dispatch & Autonomous Route Optimization Engine"
issue_description: |
  # Research Report: Agentic Field Service Dispatch & Autonomous Route Optimization Engine

  ## Problem Statement
  Field service owners like Carlos (a handyman operating out of his truck) spend a massive portion of their day coordinating logistics instead of performing billable work. When an inquiry comes in, Carlos must manually check his calendar, assess the travel time from his previous job to the new location, draft a quote, and sequence the day's visits. Existing tools are either standalone booking calendars (Calendly) that don't understand travel constraints, or complex fleet management software designed for large enterprises. Carlos needs an invisible engine that acts as a dispatch manager: automatically optimizing routes, booking service appointments based on geographic proximity to existing jobs, and proactively updating customers if he's running behind.

  ## Research Report

  **Competitor Systems Audit:**
  - **Jobber / ServiceTitan:** Powerful, industry-standard tools for field services, but they are heavy, expensive, and require significant manual setup. They are geared toward businesses with multiple technicians and dispatchers, rather than a solo operator.
  - **Calendly / Acuity:** Great for scheduling, but they treat all appointments as geographically agnostic. They do not factor in the drive time from Job A in the North side of town to Job B in the South side.
  - **Shopify / Wix:** Lack native capabilities for field service routing and time-slot management based on physical locations.

  **Gaps Identified:**
  OneHumanCorp (OHC) currently lacks a spatial-aware scheduling capability. If a customer tries to book Carlos, the system checks calendar availability but ignores location. This leads to inefficient routing, missed appointments, and lower daily revenue. OHC needs a dispatch engine where the "Operations Agent" natively understands geography and automatically optimizes Carlos's day.

  ## Design Doc

  ### Architecture Overview

  The Autonomous Route Optimization Engine extends the existing booking system with spatial awareness and real-time dispatch capabilities.

  - **Spatial Ledger (PostGIS):** A PostgreSQL extension to store service locations and calculate distances.
  - **Routing Service:** Integrates with mapping APIs (e.g., Google Maps API or Mapbox) to estimate drive times between appointments.
  - **Operations Agent (The Dispatcher):** Dynamically adjusts available time slots for new bookings based on the location of already confirmed jobs that day.

  ### Architecture Diagram

  ```mermaid
  graph TD;
      subgraph Mobile Device
          CustomerApp[Customer Booking UI]
          OwnerApp[Carlos's OHC App]
      end

      CustomerApp -->|Request Slot at Address X| Gateway[OHC API Gateway];
      OwnerApp -->|View Daily Route| Gateway;

      Gateway --> DispatchEngine[Dispatch & Route Engine];

      DispatchEngine --> SpatialDB[(PostGIS Spatial DB)];
      DispatchEngine --> MapsAPI[External Maps API (Drive Times)];

      DispatchEngine --> OpsAgent[Operations AI Agent];

      subgraph AI Agent Workflows
          OpsAgent -->|Calculate dynamic availability| CustomerApp;
          OpsAgent -->|Detect delay -> SMS Customer| CSAgent[Customer Success Agent];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Customer Booking:** A customer enters their address. The system (via the Operations Agent) queries the day's existing jobs and only offers time slots that allow for sufficient drive time.
  2. **Owner Route View:** Carlos opens the OHC app. His dashboard shows today's jobs as a sequenced, geographically optimized list.
  3. **One-Tap Navigation & Status:** Each job card has a "Start Travel" button. Tapping it opens Google Maps/Waze.
  4. **Autonomous Updates:** If Carlos taps "Running Late" (or if the Agent detects a delay based on the completion time of the previous job), the Customer Success Agent automatically drafts and sends an SMS to the next customer: "Hi, Carlos is finishing up a repair nearby and will arrive about 15 minutes later than planned."

  ### AI Agent Integration Points
  - **Operations Agent:** Acts as the dispatcher. Evaluates spatial constraints to propose feasible booking slots.
  - **Customer Success Agent:** Handles proactive communication. If the schedule shifts, it manages customer expectations automatically via SMS or WhatsApp.

  ## Implementation Prompt
  Implement the Agentic Field Service Dispatch & Autonomous Route Optimization Engine.
  - **User-Facing Outcome:** Solo field service operators can offer self-serve booking that intelligently groups jobs by location, ensuring they don't have to crisscross the city. The day's schedule is presented as an optimized route.
  - **CUJ:** Customer A books a morning job in Zip Code 10001. Customer B tries to book an afternoon job in Zip Code 10002. The system calculates drive time and only offers slots to Customer B that accommodate travel from 10001. Carlos sees his day laid out seamlessly and can tap to navigate.
  - **Acceptance Criteria:**
    - Introduce PostGIS capabilities to the PostgreSQL database for storing and querying spatial data securely within tenant boundaries.
    - Implement the `RouteOptimizationService` that interfaces with a mock mapping API (for testing) to calculate drive times.
    - Update the booking availability endpoints to factor in geographical constraints.
    - Build a mobile-first (375px) "Daily Route" view in the frontend.
    - Add E2E tests verifying that overlapping/impossible travel bookings are rejected and that the route view renders correctly.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
