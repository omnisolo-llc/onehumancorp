# OneHumanCorp Onboarding Developer Guide

This guide provides technical details on the onboarding system for OneHumanCorp (OHC), focusing on the "Day One" experience.

## Architecture Overview

The onboarding process is a collaboration between the frontend (React-based or the embedded UI in `lib.rs`), the `OnboardingAgent` service, and the REST API.

### Component Diagram

1.  **Frontend Wizard**: A multi-step UI that collects user input and provides real-time feedback (e.g., website previews).
2.  **Onboarding API**: `/api/onboarding/*` routes that handle state persistence and AI suggestions.
3.  **Onboarding Agent**: The backend orchestrator that manages database interactions, AI generation, and initial tenant provisioning.
4.  **Database**: The `onboarding_state` table stores the progress of each user.

## Core Features

### 1. State Persistence & Resumption

We use a "Save-as-you-go" strategy. Every time a user clicks "Next", the current state is pushed to the backend.

-   **Endpoint**: `POST /api/onboarding/state`
-   **Payload**:
    ```json
    {
      "tenant_id": "...",
      "org_id": "...",
      "user_id": "...",
      "step": 3,
      "state": { "company_name": "Maya's Cakes", ... }
    }
    ```
-   **Resumption**: On page load, the frontend calls `GET /api/onboarding/state`. If a previous state exists, the wizard automatically navigates to the last completed step and populates the fields.

### 2. AI-Powered Enhancements

To minimize friction, we use AI to perform heavy lifting:

-   **Product Descriptions**: The `/api/onboarding/suggest` endpoint takes a product name and business type and returns a catchy 1-sentence description.
-   **Tenant Provisioning**: Upon completion, the agent automatically creates standard products and seeds a full team of AI agents for the user.

### 3. Visual Excellence

All onboarding screens must adhere to the OHC Design System:
- **Font**: 'Outfit' for headings, 'Inter' for body text.
- **Colors**: Primary Gold (`#D4AF37`), Deep Background (`#0A0A0A`).
- **Style**: Glassmorphism (backdrop-filter blur, low opacity white backgrounds).

## Database Schema

The `onboarding_state` table is defined as follows:

```sql
CREATE TABLE IF NOT EXISTS onboarding_state (
    tenant_id TEXT NOT NULL,
    organization_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    current_step INTEGER NOT NULL DEFAULT 0,
    state_json JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (tenant_id, organization_id)
);
```

## Adding a New Step

To add a new step to the wizard:
1.  **UI**: Add a new `div` with an ID like `step-N` in `src/server/lib.rs` (or the React component).
2.  **Indicator**: Update the `step-indicator` dots to reflect the new total steps.
3.  **Logic**: Ensure `nextStep(N)` is called on button click.
4.  **Backend**: If the new step requires specific logic (e.g., verifying a domain), add the corresponding method to `OnboardingAgent`.

## Testing Strategy

Onboarding is mission-critical. We employ three layers of testing:

1.  **Unit Tests**: In `src/server/services/onboarding/onboarding_agent.rs`, testing state saving and AI suggestion logic (with mocks).
2.  **API Tests**: Verifying the Axum handlers in `src/server/api/onboarding/mod.rs`.
3.  **E2E Tests**: Playwright scripts in `src/e2e/onboarding.spec.ts` that simulate a user going through the entire flow across different personas (Maya, Carlos, etc.).

## Best Practices

-   **30-Second Rule**: If a step takes more than 30 seconds for a user to understand, it must be simplified.
-   **Mobile-First**: Always verify changes on a 375px wide screen.
-   **No Dead Ends**: Every step must have a clear "Next" action and a "Back" button for correction.
-   **Optimistic UI**: Update the UI immediately while saving state in the background.

---

*OneHumanCorp - Empowering Every Human to be a Business Owner.*
