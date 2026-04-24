1. **Frontend Update (`src/app/lib/screens/business_setup_wizard_screen.dart`)**
   - The wizard needs to strictly follow the problem description steps:
     1. Welcome screen
     2. Business type (single tap grid tiles: Online Store, Service Business, Restaurant / Food, Creative / Portfolio, Local Business, Other)
     3. Business name & description
     4. What do you sell? (multi-select grid tiles)
     5. How do you want to receive payments?
     6. Administrator account (name, email, password with strength)
     7. Review & Launch
   - When launching, it already uses `/api/wizard/configure` but we also need to implement `/api/wizard/state/save` and load endpoints so that if a user quits and comes back from another device, they resume where they left off.

2. **Backend API endpoints (`src/server/dashboard/handlers_wizard.go`)**
   - Implement `handleWizardStateSave(w, r)`: Takes a JSON payload of the entire state and saves it. For now, since `settings.Extras` is our DB store, we can marshal the state into a JSON string and store it in `settings.Extras["wizard_state"]`.
   - Implement `handleWizardStateLoad(w, r)`: Reads `settings.Extras["wizard_state"]` and returns it, or a 404/empty.
   - Wire these up in `src/server/dashboard/server.go`.

3. **Modify `BusinessSetupWizardScreen` to load and save state**
   - Add a `saveState()` method to `BusinessSetupNotifier` that calls `/api/wizard/state/save`. Call this on every `nextStep()` and `prevStep()`.
   - Add a `loadState()` method that runs on init and populates the `BusinessSetupState` from the backend API `/api/wizard/state/load`.

4. **Add go/flutter tests**
   - Add/update flutter tests if needed (`src/app/test/screens/business_setup_wizard_screen_test.dart` if present).
   - Write E2E test in Go.

5. **Pre-commit steps**
   - Run `pre_commit_instructions` and follow them to complete proper testing, verifications, reviews and reflections.
