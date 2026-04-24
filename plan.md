# Problem Description

The problem requests replacing the current `Business Setup Wizard` logic with a simplified version.
The new wizard should have:
1. **Welcome screen**: Beautiful hero animation, one-line value proposition ("Your business, live in minutes").
2. **Business type**: Single-tap selection from friendly categories with large icons: Online Store, Service Business, Restaurant / Food, Creative / Portfolio, Local Business, Other. No dropdowns.
3. **Business name & description**: Large-input text fields. AI auto-suggests a tagline and short description based on the business name. User can accept or edit.
4. **What do you sell?**: Multi-select tiles: "Physical products", "Digital downloads", "Services / appointments", "Food & beverages", "Subscriptions". Friendly labels only.
5. **How do you want to receive payments?**: Card tiles — Online only, In-person (POS), Both, Skip for now. Show estimated time to first payment next to each.
6. **Administrator account**: Name, email, password (strength meter), optional SSO (Google / Apple). No username, no security questions.
7. **Review & Launch**: Summary card with a pulsing "Launch My Business →" CTA. Clicking it provisions the tenant, selects a starter website template, pre-seeds AI agents, and lands the user in the dashboard with a "Your business is setting up…" animated progress overlay.

# Proposed Plan

1. **Update Frontend UI & State**
   - Refactor `src/app/lib/screens/business_setup_wizard_screen.dart` to match the 7 new steps (Welcome, Business type, Business name & description, What do you sell, Payment preference, Administrator account, Review & Launch).
   - Use `AnimatedSwitcher` to animate between the steps.
   - Use `GlassCard` and OHC Premium Tokens (Glassmorphism, `Outfit`/`Inter` fonts) for styling.
   - Update `BusinessSetupState` to store fields like `businessType`, `businessDescription`, `whatYouSell`, `paymentMethod`, etc.
   - Make all screens responsive (down to 375px) without horizontal scrolling.

2. **Backend API updates**
   - To resume state, the system currently expects a `wizardConfigureRequest` to handle configuring. But the wizard state needs an endpoint `/api/wizard/state/save`? Wait, the problem says "All wizard state must be persisted to the OHC backend so resuming from another device works seamlessly. Implement backend API endpoints if they do not already exist."
   - The backend `dashboard` server handles wizard config currently. I will add `handleWizardStateSave`, `handleWizardStateLoad` to `src/server/dashboard/handlers_wizard.go` and wire them up in `src/server/dashboard/server.go`.
   - Add a `handleWizardStateSave` handler which accepts a JSON payload of the wizard state and stores it in memory (or Redis, though `dashboard` server is simpler, I'll store it in a `wizardState` map in `Server`). Or wait, `Server` has `settings.Extras`. We can just store wizard progress in `settings.Extras`. Let's just create a quick endpoint for `state/save` and `state` (load).

3. **Cleanup unused code**
   - Delete `src/server/lib/features/onboarding/business_setup_wizard.dart` and `business_setup_wizard_test.dart` as they are Flutter UI mockups misplaced in the `server` tree and unused. Or I will just replace `src/app/lib/screens/business_setup_wizard_screen.dart` and delete the server ones.

4. **Testing**
   - Write unit tests for `src/app/lib/screens/business_setup_wizard_screen.dart` (if needed, replace existing tests like `wizard_screen_test.dart` or add `business_setup_wizard_test.dart` in `src/app/lib/screens/`).
   - Write Go E2E tests for the flow in `src/tests/e2e/e2e_business_setup_test.go`. The test must start from home page after login, navigate the wizard, select options, and assert the final API call or UI state.

5. **Pre-commit checks**
   - Call `pre_commit_instructions` and follow the required validation, verification, formatting, and tests (`bazelisk test //...`).
