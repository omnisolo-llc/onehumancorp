1. **Understand Goal:** Implement the "Website Builder Onboarding" Wizard as described in the requirements. Triggered after the setup wizard, or when the user taps "Build My Website".
2. **Review Requirements:**
   - Template gallery: Full-bleed card grid, live mini-preview, "Use this template ->" CTA.
   - Brand colors & logo: Color palette picker (AI suggests 3 palettes), logo upload with AI background removal, or "Generate a logo for me" (3 options).
   - Add your first product or service: Inline add form (name, photo, price, short description). AI auto-generates description.
   - Connect a domain: "Use a free OHC subdomain", "Use my own domain", "Buy a domain".
   - Go Live: Preview of live site, "Publish" button. Auto-copied to clipboard.
   - Progressive Disclosure: Standard mode plain language, Advanced mode raw config fields.
   - OHC Premium Design Standards: Glassmorphism (`backdrop-filter: blur(20px)`), Outfit + Inter, animations.
   - Test Coverage: 100% unit and E2E test coverage.
3. **Implementation Plan:**
   - Create `src/app/lib/screens/website_builder_wizard_screen.dart`.
     - Implement the 5 steps using `Stepper` or a custom wizard flow.
     - Build the state manager using Riverpod (`WebsiteBuilderNotifier` / `WebsiteBuilderState`).
     - Incorporate the OHC Premium Design Tokens.
   - Add route in `src/app/lib/router.dart` (e.g., `/wizards/website-builder`).
   - Create `src/app/test/screens/website_builder_wizard_screen_test.dart` for 100% unit test coverage.
   - Update `src/tests/e2e/wizard_test.go` or create a new test for the website builder E2E test. Actually, the requirements say "E2E-test every wizard: start from the home page after user login, navigate through the complete wizard flow by clicking UI links/buttons... go through every step until the process finishes...". So I will add it to `cuj_website_builder_e2e_test.dart` or something. Wait, the prompt says "E2E widget tests (`cuj_*_e2e_test.dart`) should be written in Dart, placed in `srcs/app/test/`". I will add `cuj_website_builder_e2e_test.dart` in `src/app/test/`.
   - Update the UI to include a link to the website builder from the Dashboard.
   - Fix all Dart tests and ensure `bazelisk test //...` passes.
