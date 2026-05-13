# Feature Brief: 10-Minute Progressive AI Interview (Onboarding Wizard)

## Priority: P0 (Critical)
**Strategic Goal:** Eliminate Setup Paralysis.

### 1. Problem Statement
The current industry standard for creating an online store involves overwhelming dashboards, complex settings (shipping zones, tax configurations, DNS setup), and a "blank canvas" problem. Users like "Carlos the Handyman" or "Maya the Baker" abandon setup because they are forced to act as web developers and systems integrators. We must reduce the time-to-value from hours to under 10 minutes.

### 2. The OHC Solution
A conversational, progressive AI interview. The user simply talks to the onboarding agent via a chat interface. The AI asks 3-5 high-impact questions and instantly provisions a fully configured store in the background. No forms. No complex menus.

### 3. Architecture & Implementation (Research Report)
-   **Trigger:** User signs up and is immediately dropped into a full-screen chat interface.
-   **Agent:** The `autodream` agent framework acts as the interviewer.
-   **Data Extraction:** The AI extracts the business name, industry, tone, and primary offerings from the natural language conversation.
-   **Execution:**
    -   Calls KAIROS to register the tenant.
    -   Generates an initial, mobile-first, 375px-optimized design template.
    -   Drafts initial SEO-optimized copy for the homepage.
    -   Creates 1-3 placeholder products/services based on the industry.
-   **The Magic Moment:** The chat concludes with "Your store is ready. Here's your link."

### 4. Implementation Prompt
Implement the progressive AI onboarding interview flow. Create a mobile-first (375px) chat UI where the `autodream` agent collects business context. Upon completion, the system must automatically provision the tenant, select a design template, generate localized copy, and create placeholder products, outputting a fully functional storefront without the user filling out a single traditional form.
