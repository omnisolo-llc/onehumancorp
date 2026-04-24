1. **Create Tooltip Registry**:
   - Create a new file `src/app/lib/widgets/help/tooltip_registry.dart`. This will contain a `Map<String, String>` mapping tooltip IDs to plain-language text (max 2 sentences).
   - Create a new wrapper widget `ContextualTooltip` in `src/app/lib/widgets/help/contextual_tooltip.dart` that looks up the text from the registry and wraps the given child using Flutter's native `Tooltip` (or a custom floating overlay using Glassmorphism tokens).

2. **Create Help Center UI**:
   - Add `src/app/lib/screens/help/help_center_screen.dart` with a searchable, beautifully designed help portal using Glassmorphism tokens.
   - Add `src/app/lib/screens/help/help_article_screen.dart` to view individual help articles.
   - Articles should cover: "Getting Started", "My Store", "Payments", "AI Agents", "Marketing", "Account & Billing".

3. **Update Router**:
   - Modify `src/app/lib/router.dart` to add the `/help` and `/help/article/:id` routes.
   - Add a "?" navigation item to the AppShell sidebar or AppBars to link to the `/help` route.

4. **Integrate AI Help Chat & Tooltips**:
   - Add a floating action button (FAB) or a persistent bottom chat button on the `AppShell` (or every major screen) for the "Ask anything" AI Help Chat. It should route to a specialized help chat overlay or screen.
   - Integrate `ContextualTooltip` on key non-obvious UI elements across major screens (e.g. Dashboard, Agents).

5. **Update BUILD.bazel**:
   - Modify `src/app/lib/screens/BUILD.bazel` to include `"help/*.dart"` in `SCREEN_SRCS`.
   - Modify `src/app/lib/widgets/BUILD.bazel` to include `"help/*.dart"` in its `srcs`.

6. **Create Tests**:
   - Add widget tests for the new screens and tooltips to ensure 100% code coverage.
   - Add an E2E test to simulate a user opening the Help Center, searching for an article, opening it, and interacting with the Help Chat.

7. **Pre-commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit**:
   - Commit the change with title "✍️ Scribe: In-App Help Center and Tooltips" and PR description matching the tasks.
