1. **Create the Website Builder Wizard Screen (`srcs/app/lib/screens/website_builder_wizard_screen.dart`)**:
   - Create a state notifier using Riverpod to track: `step`, `template`, `brandColor`, `productName`, `productPrice`, `productDescription`, `domainOption`, `domainName`, `isAdvancedMode`, etc.
   - Implement the 5 steps matching the design doc:
     - Step 0: Template Gallery (Grid of templates, selectable, CTA turns green).
     - Step 1: Brand Colors & Logo (Color palettes, generate logo).
     - Step 2: Add Product (Name, price, description with AI generation).
     - Step 3: Connect Domain (Free subdomain vs custom domain).
     - Step 4: Go Live (Preview and Publish button, clipboard copy).
   - Add the progressive disclosure toggle for "Advanced Mode".
   - Use `GlassCard`, `Outfit`, and `Inter` fonts for premium aesthetic.
   - Make it mobile-responsive (375px) via single-column layout.

2. **Add Backend Endpoint (`srcs/server/dashboard/handlers_wizard.go` and `server.go`)**:
   - Add a `/api/wizard/website` endpoint to `server.go`.
   - Implement `handleWizardWebsite` in `handlers_wizard.go` to parse the `wizardWebsiteRequest` and return a successful JSON response (`{"status": "published"}`).

3. **Update Navigation (`srcs/app/lib/router.dart`, `business_setup_wizard_screen.dart`, `dashboard_screen.dart`)**:
   - Add `GoRoute(path: '/website_builder', builder: ...)` in `router.dart`.
   - Change `GoRouter.of(context).go('/dashboard')` to `GoRouter.of(context).go('/website_builder')` in `business_setup_wizard_screen.dart`.
   - Add a prominent "Build My Website" button on `DashboardScreen`.

4. **Write E2E Test (`srcs/app/e2e/website_builder.spec.ts`)**:
   - Write a complete Playwright test that starts at the home page, logs in, clicks "Build My Website" (or completes the business setup), and goes through all 5 steps of the Website Builder Wizard, clicking buttons, filling fields, and asserting the final result matches the design ("published" state).

5. **Pre-commit**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
