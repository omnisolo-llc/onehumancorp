The goal is to update the Business Setup Wizard to align with the "Day One" requirements in the prompt.
1. The `business_setup_wizard_screen.dart` currently asks for "Company Name", "Industry", "Size" (S/M/L dropdown), "Goals" (Support, Build software, etc), "Deployment Preference".
2. The requirements explicitly say:
    *   **Welcome screen**: Beautiful hero animation, one-line value proposition ("Your business, live in minutes").
    *   **Business type**: Single-tap selection from friendly categories with large icons: Online Store, Service Business, Restaurant / Food, Creative / Portfolio, Local Business, Other. No dropdowns.
    *   **Business name & description**: Large-input text fields. AI auto-suggests a tagline and short description based on the business name. User can accept or edit. *(I will leave out the actual AI integration for now, or just have static text fields).*
    *   **What do you sell?**: Multi-select tiles: "Physical products", "Digital downloads", "Services / appointments", "Food & beverages", "Subscriptions". Friendly labels only.
    *   **How do you want to receive payments?**: Card tiles — Online only, In-person (POS), Both, Skip for now. Show estimated time to first payment next to each.
    *   **Administrator account**: Name, email, password (strength meter), optional SSO (Google / Apple). No username, no security questions.
    *   **Review & Launch**: Summary card with a pulsing "Launch My Business →" CTA.
3. I'll rewrite `business_setup_wizard_screen.dart` and its state to track these new steps.
4. Since the `business_setup_wizard_screen.dart` uses `notifier.launch`, I'll update it to match the new state object and make a dummy request or keep the existing launch logic.
5. Create E2E test `cuj_business_setup_e2e_test.dart` to cover the new wizard.
