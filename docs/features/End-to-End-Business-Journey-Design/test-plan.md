# End-to-End Business Journey Test Plan

## Objective
Verify that the mobile-first, zero-configuration journey handles AI mock generation, dashboard rendering, and user flow transitions correctly for our core personas.

## Test Scenarios
1. **Persona: Maya (Baker) via Instant Setup**
   - **Action:** User navigates to Instant Setup, enters "I bake custom vegan cakes in Brooklyn", and clicks Generate.
   - **Verification:** System resolves the AI extraction and transitions to the Launch screen.
   - **Verification:** User clicks Launch and confetti success message is shown.
   - **Verification:** Share Link CTA is visible.

2. **Persona: Carlos (Handyman) via Instant Setup**
   - **Action:** User enters "I fix plumbing and do home repairs in Austin" and generates.
   - **Verification:** System resolves the extraction and launches successfully.

3. **Persona: Priya (Boutique) via Instant Setup**
   - **Action:** User enters "I run a clothing boutique selling dresses" and generates.
   - **Verification:** System resolves the extraction and launches successfully.

4. **Persona: Leo (Tutor) via Instant Setup**
   - **Action:** User enters "I teach guitar lessons online and in person" and generates.
   - **Verification:** System resolves the extraction and launches successfully.

5. **Persona: Fatima (Food Cart) via Instant Setup**
   - **Action:** User enters "Halal food cart menu for pre-orders" and generates.
   - **Verification:** System resolves the extraction and launches successfully.

6. **Dashboard CTA Presence**
   - **Action:** User navigates to the dashboard after a successful onboarding.
   - **Verification:** The "🎉 Your business is live!" card and the "Share Link" button are present on the dashboard.

## Test Implementation
These scenarios will be implemented as Playwright E2E tests within `src/e2e/business_setup.spec.ts` and `src/e2e/dashboard.spec.ts`, exercising the full UI interaction flow from the frontend and integrating with the backend.
