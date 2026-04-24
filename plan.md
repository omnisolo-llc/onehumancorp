1. **Tooltip Registry**:
   - Create `src/app/lib/widgets/tooltip_registry.dart` containing a centralized `TooltipRegistry` class/provider to store tooltip texts by key, and a `RegistryTooltip` widget that reads from it.
   - Use `grep` and `sed` or `write_file` to create this.

2. **Help Portal Screen (In-App Help Center)**:
   - Create `src/app/lib/screens/help_portal_screen.dart` with categories: "Getting Started", "My Store", "Payments", "AI Agents", "Marketing", "Account & Billing".
   - Include a search bar.
   - Use `GlassCard` and OHC premium tokens.

3. **Release Notes Screen**:
   - Create `src/app/lib/screens/release_notes_screen.dart` showing recent OHC updates in plain language.

4. **API Documentation Screen**:
   - Create `src/app/lib/screens/api_docs_screen.dart` with a mockup of an interactive API reference.

5. **AppShell Updates & AI Help Chat**:
   - Modify `src/app/lib/router.dart` to include the new routes (`/help`, `/release-notes`, `/api-docs`).
   - Add sidebar links for "Release Notes", "API Docs" and "Help Portal".
   - Add a Floating Action Button (FAB) in `AppShell` for the "Ask anything" AI-powered help chat.
   - Implement the Help Chat widget overlay/modal in `AppShell`.

6. **Interactive Walkthroughs**:
   - Create `src/app/lib/widgets/walkthrough_overlay.dart` that implements the step-by-step tours ("Set up your store", etc.) using an overlay with speech bubbles.

7. **Testing**:
   - Create `src/app/test/screens/help_portal_screen_test.dart` and other tests to ensure 100% coverage.
   - Run `bazelisk test //...` to ensure everything compiles and passes.

8. **Pre-commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit PR**:
   - Title: "✍️ Scribe: In-App Help Center and Documentation Infrastructure"
