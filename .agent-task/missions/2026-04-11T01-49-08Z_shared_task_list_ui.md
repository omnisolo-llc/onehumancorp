---
status: "DONE"
agent: Palette
Title: "KAIROS Phase 4: Shared Task List Premium UI"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
The One Human Corp (OHC) platform has implemented the KAIROS Orchestration backend (Shared Task List, Teammate Mesh, AutoDream). However, the end-user lacks a visual interface to monitor and orchestrate the swarm's tasks. We need a Flutter-based UI for the Shared Task List that adheres strictly to the OHC Premium Feel.

# Research Report
- The orchestration backend relies on `shared_tasks`, Redis Pub/Sub, and PostgreSQL/SQLite.
- The UI must connect to these APIs and visually display tasks transitioning through states (PENDING, ASSIGNED, IN_PROGRESS, REVIEW, COMPLETED, FAILED).
- Visual Excellence is a strict mandate: Glassmorphism, 20px blur, and Outfit/Inter fonts.

# Design Doc
**UI Architecture:**
- **Route:** `/orchestration/tasks` in the Flutter web/desktop app.
- **Components:**
  - `TaskListScreen`: Main dashboard showing a Kanban-style or list view of tasks.
  - `TaskGlassCard`: A reusable widget displaying individual task details (Title, Assigned Agent, Status, Dependencies).

**Aesthetics:**
- The background of the screen and cards must use the `GlassCard` widget or equivalent styling:
  `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03);`
- Typography must be explicitly set to `'Outfit'` or `'Inter'`.

# Implementation Prompt
You are an Implementer agent. Your task is to build the Flutter UI for the Shared Task List.
1. Create a new screen in `srcs/app/lib/screens/orchestration/task_list_screen.dart`.
2. Ensure the screen fetches and displays tasks from the backend API.
3. Build the task cards using the required Glassmorphism styling. Do NOT use standard Material `Card` without applying the blur and transparency. If using `MaterialType.transparency`, explicitly omit the `color` property.
4. Verify your changes using the Playwright frontend verification workflow: `cd srcs/app && flutter build web` and `python3 -m http.server 3000` from `srcs/app/build/web`.

# Priority
P0

# Estimated Scope
Medium
