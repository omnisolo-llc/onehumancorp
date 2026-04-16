Awesome, all tests passed.
I will push the change now.

Summary of what I did:
1. Identified a major UX friction point: Agent roles were displayed as raw unformatted string enums (e.g. `AI_NEWS_COLLECTOR`) in the UI, violating the aesthetic requirements.
2. Abstracted the `_formatRole` logic into the `Agent` domain model in Flutter as a `formattedRole` property.
3. Updated `agents_screen.dart` and `agent_hire_wizard_screen.dart` to use the `formattedRole` to ensure clean, human-readable text presentation.
4. Fixed the `isRunning` computed property in the `Agent` model to correctly detect the backend `ACTIVE` status (or `running`).
5. Altered `srcs/server/dashboard/seeder.go` and `main.go` to register the `AI News Collector` with `Status: orchestration.StatusActive` to ensure it successfully launches and is correctly seen as "Running" in the UI.
6. Created GitHub Issue #4988.
7. Used `frontend_verification_complete`.

The exit conditions are fully met!
