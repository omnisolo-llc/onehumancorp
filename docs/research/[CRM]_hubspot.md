# [CRM] OHC Tool Integration Research Brief: HubSpot Integration

## Title
Syncing OHC Customer Data with HubSpot CRM

## Problem Statement
Many growing small businesses already use a dedicated CRM like HubSpot to manage their sales pipeline, deals, and marketing efforts. They want to use OHC for specific operations (like bookings or invoicing) but need their customer data to remain synchronized with HubSpot to maintain a single source of truth for their sales team.

## Research Report
HubSpot is a dominant player in the inbound marketing and sales CRM space for SMBs and mid-market companies.

**Evaluated Tool:**

1. **HubSpot (hubspot.com)**
    *   **Focus:** Inbound marketing, sales, and customer service CRM.
    *   **Pros:** Extremely popular, comprehensive feature set. Acts as a central hub for many businesses.
    *   **Cons:** Can become very expensive as contact lists and feature requirements grow.

**Recommendation:**
Integrating with HubSpot is a high-value feature for OHC, as it allows us to coexist with established sales workflows. The integration should focus on a bi-directional (or at least robust one-way) sync of contact information to prevent data silos.

## Design Doc
**Integration Approach: Bi-directional Contact Sync with HubSpot**

1.  **Authentication:**
    *   Implement standard authorization flow to securely connect to the external system.

2.  **Contact Sync:**
    *   When a new customer is created in OHC, create a corresponding Contact in the external CRM.
    *   When a customer is updated in OHC, update the external Contact record.

3.  **Activity Logging (Optional):**
    *   Log key OHC activities (e.g., "Invoice Paid") as Engagements/Notes on the external Contact timeline to give sales reps full visibility.

## Implementation Prompt
**Objective:** Implement contact syncing from OHC to HubSpot CRM.

**Acceptance Criteria:**
1.  Implement the authentication flow and securely store tenant connection details.
2.  Implement a background worker that listens for Customer creation/update events in OHC.
3.  Map OHC Customer fields (Name, Email, Phone, Company) to standard external Contact properties.
4.  On OHC Customer creation/update, push the mapped data to the external system, storing the resulting external ID for future updates.

## Priority
P2

## Estimated Scope
Medium
