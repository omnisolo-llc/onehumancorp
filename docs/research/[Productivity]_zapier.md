# [Productivity] OHC Tool Integration Research Brief: Zapier Integration

## Title
Connecting OHC to Thousands of Apps via Zapier

## Problem Statement
No matter how many native integrations OHC builds, there will always be niche tools or specific workflows that small business owners need to connect to their OHC data. Building custom point-to-point integrations for every possible request is unsustainable and distracts from core platform development.

## Research Report
Zapier is the undisputed leader in iPaaS (Integration Platform as a Service) for end-users, acting as the connective tissue between thousands of web applications.

**Evaluated Tool:**

1. **Zapier (zapier.com)**
    *   **Focus:** No-code workflow automation.
    *   **Pros:** Massive ecosystem. Excellent visual builder for users to create "Zaps" (if this, then that).
    *   **Cons:** The integration burden is on OHC to build and maintain the connector app.

**Recommendation:**
Building a public connector application for OHC is the ultimate "escape hatch" integration. It instantly gives our users the ability to connect OHC to almost any other tool they use (e.g., adding a row to a spreadsheet when a new customer is created, or sending a chat message when a high-value invoice is paid).

## Design Doc
**Integration Approach: Building an OHC Connector App**

1.  **Authentication:**
    *   Implement secure authentication for the connector app to securely access OHC tenant accounts.

2.  **Triggers (OHC -> External):**
    *   Expose webhooks or polling endpoints in OHC for key events:
        *   `New Customer`
        *   `New Order`
        *   `Order Status Updated`
        *   `New Appointment`

3.  **Actions (External -> OHC):**
    *   Expose data endpoints for external systems to perform actions in OHC:
        *   `Create Customer`
        *   `Create Order`
        *   `Update Order Status`
        *   `Create Appointment`

## Implementation Prompt
**Objective:** Develop the backend infrastructure required to support a public connector application for OHC.

**Acceptance Criteria:**
1.  Implement a dedicated API authentication mechanism for users to connect their OHC account to external workflow tools.
2.  Develop a webhook registration and dispatch system to send real-time event payloads (Triggers) to external webhook URLs.
3.  Ensure comprehensive, well-documented data endpoints are available for all core OHC entities (Customers, Orders, Appointments) to support external Actions.
4.  Create the initial connector project scaffolding defining the authentication, at least two Triggers, and at least two Actions.

## Priority
P1

## Estimated Scope
Large
