1. *Add `API Key` visibility toggle in `AgentHireWizardScreen`.*
   - Read the UX friction report carefully.
   - Identified the need for `obscureText` toggle in `AgentHireWizardScreen` for API Key.
   - Added an `_apiKeyController` and a visibility toggle to the `Advanced Configuration` section in `AgentHireWizardScreen`.
2. *Verify Glassmorphism in `AgentHireWizardScreen`.*
   - Identified that `BackdropFilter` and `ColorFilter.matrix` was missing on the nodes in the Topology step.
   - Wrapped the agent and code builder containers with `BackdropFilter`.
3. *Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.*
   - Run `pre_commit_instructions` tool.
   - Wait for feedback or confirmation.
4. *Submit changes.*
   - Ensure the tests pass.
   - Commit with the correct title `🗺️ Guide: [new onboarding feature]`.
