# [Email Marketing] OHC Tool Integration Research Brief: ActiveCampaign

## Title
Enable Advanced Customer Journeys and Email Automation

## Problem Statement
While simple newsletters are a good start, many small business owners quickly outgrow basic email sends. They want to set up automated sequences (e.g., "Welcome series," "Abandoned cart recovery," or "Re-engagement campaigns") based on specific customer actions. Setting up complex logic manually in OHC would take immense engineering effort, and businesses already using specialized tools don't want to recreate their intricate workflows.

## Research Report
ActiveCampaign is a market leader in advanced marketing automation for SMBs.

**Evaluated Tool:**

1. **ActiveCampaign (activecampaign.com)**
    *   **Focus:** Advanced marketing automation, CRM, and email marketing.
    *   **Pros:** Its visual automation builder is arguably the best in the industry. Deep segmentation capabilities based on almost any data point or event.
    *   **Cons:** The learning curve is steep for novice users. The interface can feel overwhelming.
    *   **Pricing:** Starts around $15/month for basic, but quickly scales up based on features and contacts.

**Recommendation:**
While Resend is excellent for simple, programmatically triggered emails (as recommended in our primary Email Marketing brief), **ActiveCampaign** is the tool we should integrate with for users who demand complex, visual automation flows. Instead of building a complex drag-and-drop automation builder in OHC, we should focus on streaming rich customer events from OHC to the external platform, allowing the business owner to use their native UI for the actual campaign logic.

## Design Doc
**Integration Approach: Event Streaming to External Platform**

1.  **Authentication:**
    *   Business owner provides their integration credentials in OHC settings.

2.  **Contact Sync & Event Tracking (Trigger):**
    *   OHC acts as a data source.
    *   When a customer is created or updated in OHC, their profile is synced.
    *   More importantly, when key actions occur in OHC (e.g., `Order Placed`, `Appointment Booked`, `Invoice Paid`), OHC sends a "Custom Event" to the external platform.

3.  **Automation (User Experience):**
    *   The business owner logs into their external account.
    *   They create an automation triggered by the custom event sent from OHC (e.g., "When 'OHC_Appointment_Booked' occurs").
    *   The external platform handles the delivery and timing of the resulting communications.

## Implementation Prompt
**Objective:** Implement contact syncing and event streaming for advanced marketing automation.

**Acceptance Criteria:**
1.  Create a configuration model storing the integration credentials.
2.  Add an event listener system in OHC that hooks into core domain events (Customer Created, Order Completed, Appointment Booked).
3.  When an event fires, map the OHC data to the corresponding external contact and dispatch the event payload if the integration is active.

## Priority
P2

## Estimated Scope
Medium
