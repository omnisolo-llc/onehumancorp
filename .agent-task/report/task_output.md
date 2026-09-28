issue_title: 🛡️ Scribe: [new documentation feature]
issue_description: |
  **Role:** Principal Technical Writer & Scribe (L7)

  **Issue:** Interactive Walkthroughs Refactoring

  **Description:**
  A previous refactoring introduced `safe-help-content.mjs` to safely render walkthrough components, but HTML files (`src/ui/tauri/src/ui/*.html`) were left referencing inline string interpolation to render bubbles, which created inconsistent styles, missing import reference errors (e.g., `closeWalkthrough` failure), and failed to adhere to the Translucent Glass materials design.

  **Resolution:**
  - Used python regex script to successfully identify and update `setup.html`, `dashboard.html`, and other walkthrough implementations to utilize `renderWalkthroughStep`.
  - Injected ES Module imports for `renderWalkthroughStep` into `HEAD` tags.
  - Validated syntax structure differences across variants.
  - A no-work finding is necessary for full suite validation because `next` executable is missing (`Next build failed: 127`), which prevents `make test` and `make lint` completion in the provided sandbox environment.

  **Evidence:**
  - `make test && make lint` failed at Next.js build due to missing Next executable (`Error: Next build failed: 127`).

issue_priority: P1
issue_category: Documentation
issue_type: Feature
issue_label: [ohc:lane:documentation, agent-ready]
assignees: [scribe]
