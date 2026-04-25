1. **Understand Goal**: Create a frictionless "Day One Onboarding" business setup wizard flow that allows users to quickly enter their business info, select deployment type, provide admin details, and seamlessly resume progress if interrupted. The backend must properly support and persist these details, and if it's the cloud mode, we need an admin to be created if not exists, but for the wizard itself, the requirement is to collect `business type, name, what they sell, and payment preferences. State must persist to the backend so resuming from another device works`.

2. **Backend Changes (Go)**:
   - In `src/server/dashboard/handlers_wizard.go`, modify `wizardStatusResponse` to include an `Extras map[string]string` field, which will hold the wizard state.
   - Update `handleWizardStatus` to populate `resp.Extras` with `cfg.Extras`.
   - Ensure `handleWizardConfigure` correctly saves `req.Extras` and returns it in the response if needed.

3. **Frontend Changes (Dart)**:
   - In `src/app/lib/screens/business_setup_wizard_screen.dart`, redesign the UI to fit the 4-step wizard explicitly requesting:
     - Step 1: Business Name and Business Type (Industry).
     - Step 2: What they sell.
     - Step 3: Payment Preferences.
     - Step 4: Admin/Deployment info (keep existing fields to pass tests or just add the new fields alongside the existing ones).
   - In `BusinessSetupNotifier`, add a `loadState` function that hits `/api/wizard/status` on init to fetch saved state (enabling cross-device resume).
   - In `BusinessSetupNotifier.nextStep()`, auto-save the state to `/api/wizard/configure` so progress isn't lost.
   - Wait, `nextStep()` is synchronous, we need an async `saveAndNextStep()`.
   - Update `businessSetupProvider` to handle asynchronous loading via `FutureProvider` or `AsyncNotifier`, or call `loadState` from the UI layer (e.g. `initState`). Actually, Riverpod's `Notifier` allows async methods. We can just add a `Future<void> init()` method and call it from a `useEffect` or `initState`.
   - Ensure "State must persist to the backend so resuming from another device works" is fully implemented.

4. **Testing Requirements**:
   - Update existing `src/app/test/screens/business_setup_wizard_screen_test.dart` to check the new fields (What they sell, Payment Preferences) and the new state saving logic.
   - Write/update E2E tests for the new UI to ensure full coverage (as mandated by constraints).
   - Run `bazelisk test //...` to ensure everything passes.

5. **Pre Commit Steps**: Execute instructions returned by `pre_commit_instructions` tool.
