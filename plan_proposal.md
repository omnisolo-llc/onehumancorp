1. **Analyze the Requirements**:
   - The user requested "Business Setup Wizard (New User — Day One)".
   - Need to follow "Required wizard steps" closely.
     - **Welcome screen**: Hero animation, one-line value prop.
     - **Business type**: Single-tap selection with large icons (Online Store, Service Business, Restaurant / Food, Creative / Portfolio, Local Business, Other). No dropdowns.
     - **Business name & description**: Large-input text fields. AI auto-suggests tagline/description (can be mocked for now or just standard textfields based on UI).
     - **What do you sell?**: Multi-select tiles (Physical products, Digital downloads, Services / appointments, Food & beverages, Subscriptions).
     - **How do you want to receive payments?**: Card tiles (Online only, In-person (POS), Both, Skip for now).
     - **Administrator account**: Name, email, password, optional SSO.
     - **Review & Launch**: Summary card with a pulsing "Launch My Business →" CTA.
   - All wizard state must be persisted.

2. **Current State**:
   - `srcs/app/lib/screens/business_setup_wizard_screen.dart` currently has:
     - `step 0`: Welcome message (text only, no hero animation).
     - `step 1`: Company Name, Industry, Size (dropdown, which is against "no dropdowns" rule).
     - `step 2`: Select Goals (checkboxes).
     - `step 3`: Deployment Preference.
     - `step 4`: Admin Name, Email, Password.

3. **Modifications to `BusinessSetupState`**:
   - Update fields to match the new required steps:
     - `step` (int)
     - `businessType` (String)
     - `businessName` (String)
     - `businessDescription` (String)
     - `whatYouSell` (List<String>)
     - `paymentPreference` (String)
     - `adminName`, `adminEmail`, `adminPassword`
   - Adjust `BusinessSetupNotifier` to match the new state fields and validation logic.

4. **UI Implementation for `BusinessSetupWizardScreen`**:
   - Update `state.step == 0`: Welcome screen with pulsing hero or simple animation and value prop.
   - Update `state.step == 1`: Business type selection (Single-tap selection from friendly categories with large icons).
   - Update `state.step == 2`: Business name & description text fields.
   - Update `state.step == 3`: What do you sell? (Multi-select tiles).
   - Update `state.step == 4`: Payment preference (Card tiles).
   - Update `state.step == 5`: Administrator account (Name, email, password).
   - Update `state.step == 6`: Review & Launch with a pulsing CTA.

5. **Testing**:
   - Update E2E tests or unit tests if any exist for this screen.
   - Check `bazelisk test //...` to ensure all tests pass.

6. **Playwright UI Verification**:
   - Create a Playwright script to navigate to `/business-setup` or whatever the route is.
   - Verify the screens look like what was built.

I will request a plan review for this outline before setting the official plan.
