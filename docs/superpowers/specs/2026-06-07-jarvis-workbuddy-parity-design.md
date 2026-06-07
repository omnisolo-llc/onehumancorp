# Jarvis Assistant WorkBuddy-Parity Design

Date: 2026-06-07
Status: Direction approved; pending user review of written spec

## Objective

Build a new primary app surface for a general-purpose Jarvis-style AI workstation assistant. The assistant should match the functional shape of Tencent WorkBuddy: a desktop/workspace operator that accepts natural-language tasks, plans and executes them, operates on authorized files, generates deliverable artifacts, supports remote control, remembers user preferences, and exposes specialists through an Expert Center.

This is not a business-center redesign. The primary product should feel like a personal AI operator for any work: documents, research, code, files, data, presentations, scheduling, and recurring tasks.

## Product Position

The new primary route is `/assistant`.

`/agents` remains intact as Expert Center. It becomes a secondary capability area reachable from `/assistant` for specialized experts, expert teams, skills, and advanced agent configuration. We should not delete or replace Expert Center because WorkBuddy itself has an expert-centered surface and the repo already has useful implementation around expert teams, skills, connectors, approvals, and agent catalog workflows.

The assistant becomes the default user mental model:

- "Tell it what to do."
- "It plans, asks when needed, executes safely, and returns verifiable results."
- "It can keep working while I switch tasks or message it remotely."

## WorkBuddy-Parity Capability Matrix

| Capability | Required Behavior | Current Repo Signal | Gap To Fill |
| --- | --- | --- | --- |
| Natural-language task creation | User creates a task from one prompt with mode, model, workspace, context, output format, and constraints. | `/agents` composer, built-in agent runtime config, task APIs. | Move to `/assistant` as the primary flow and wire to durable task records. |
| Task management | Tasks are listed by workspace/status, can be resumed, stopped, archived, unarchived, and filtered. | Shared tasks, workflows, KAIROS, UI workflow lists. | Productize task list, statuses, resume/stop actions, and task history. |
| Conversation | Each task has a follow-up conversation, tool progress, top actions, file upload, and stop/retry. | Agent streaming event proto and some chat routes. | Build task-scoped conversation timeline and streamed progress UI. |
| Parallel tasks | Multiple tasks can run concurrently and user can switch between them. | Subagents, KAIROS shared tasks, `ParallelFork`, expert team parallel execution. | Expose concurrent task controls and per-task progress in UI. |
| Local file operations | Assistant reads/writes only authorized folders, processes batches, renames, converts, and generates files. | Built-in read/write/edit/glob/grep/bash/local sync tools and sandbox code. | Add explicit folder authorization, file browser, audit trail, and artifact registration. |
| Office artifacts | Generate documents, spreadsheets, slide decks, charts, PDFs, reports, and structured tables. | Generic file tools and screenshot tool. | Add first-class artifact pipeline for DOCX, XLSX/CSV, PPTX, PDF, chart images, and markdown/HTML previews. |
| Results review | Results panel has Artifacts, All Files, Changes, and Preview views. | `/agents` has static result tabs. | Persist task artifacts, file manifests, diffs, previews, and downloadable outputs. |
| Changes review | File/code modifications show diffs before acceptance when risk requires review. | Edit tools, approvals, quality gates. | Attach changed files to task results and route risky changes through approval policy. |
| Expert Center | User can summon specialists or teams for specialized work. | `/agents` Expert Center, expert teams, skills. | Link from `/assistant`; allow escalation from a task to an expert/team. |
| Skills | Browse, install, disable, update, uninstall, upload, and create skills. | `SkillConfig`, skill tools, catalog UI. | Implement durable skill registry and lifecycle operations. |
| Connectors | Connect external services and MCP endpoints; use them at task time. | MCP config, dynamic MCP tools, connector UI. | Add connector auth/setup/status, capability discovery, and task-time connector selection. |
| Remote control | Messaging apps can submit tasks, receive progress, and continue conversations while desktop/server is running. | UI catalog only; chat integrations exist in server integrations. | Add inbound adapters for Slack/Telegram/Discord first, then additional platforms. |
| Automations | Create recurring tasks with schedule, workspace, prompt, connectors, notifications, history, and approvals. | UI templates, job queue/migrations. | Add scheduler model, execution history, notification channel, and approval integration. |
| Memory | Remember preferences, summaries, durable project context; edit, import, forget. | Memory consolidation docs/code, memory UI surface. | Productize memory CRUD/import and retrieval in task context. |
| Permission modes | Guarded default mode and elevated/full-access mode with clear risk prompts. | Tools gating, sandbox, approvals. | Define user-facing permission profiles and enforce per-task/tool policy. |
| Model/provider config | Choose model/provider per task, including local/provider endpoint options. | Runtime config, provider fields. | Validate providers, persist preferences, and surface model capability hints. |
| Data management | Shared files, archived tasks, unshare queue, storage/account cleanup. | Some UI labels and backend storage pieces. | Build durable artifact/file storage lifecycle and cleanup actions. |

## Route And Navigation

`/assistant` is the primary app surface. It should be accessible from the dashboard and should be the default destination after login once this feature is stable.

Primary sections:

- Assistant workspace: task list, conversation, composer, results.
- Remote Control: messaging app setup and connection status.
- Automations: recurring task setup and run history.
- Memory: remembered facts, summaries, imports, and forget controls.
- Skills: skill marketplace and installed skill management.
- Connectors: service/MCP setup and status.
- Expert Center link: route to existing `/agents`.

`/agents` keeps its current role as Expert Center. Visual alignment with `/assistant` is outside Phase 1 unless required for navigation or shared components.

## `/assistant` Layout

The first release should use a dense workstation layout:

- Left rail: workspace selector, task status filters, task list, archived tasks entry.
- Center: active task conversation with task title, status, streamed progress, assistant messages, tool calls, follow-ups, stop/retry controls.
- Bottom composer: prompt input, `@` references, file upload, screenshot action, work directory, output format, mode, model, constraints, permission mode.
- Right panel: results with tabs for Artifacts, All Files, Changes, and Preview.
- Top actions: New Task, Automations, Remote Control, Memory, Skills, Connectors, Expert Center.

This should be the actual usable workstation, not a landing page.

## Core Data Model

Add or adapt durable records for:

- Workspace: id, owner/user/org scope, name, default work directory, default model, created/updated timestamps.
- Assistant task: id, workspace id, title, prompt, status, mode, model/provider config, permission profile, current step, created/updated timestamps, archived flag.
- Task message: id, task id, role, content, attachments, tool call metadata, created timestamp.
- Task artifact: id, task id, type, filename, path/blob reference, mime type, size, preview reference, created timestamp.
- Task file change: id, task id, path, change type, diff/summary, approval status.
- Connector config: id, service, auth status, exposed capabilities, secret reference, enabled workspaces.
- Automation: id, workspace id, schedule, prompt, context, model, permission profile, notification channel, status.
- Memory item: id, scope, content, source, reliability, editable flag, last referenced timestamp.

Where existing tables already cover these concepts, reuse them rather than adding parallel models. The implementation plan should map each concept to current migrations before creating new schema.

## Execution Flow

1. User creates a task from `/assistant`.
2. Server creates a durable task record and initial conversation message.
3. Planner decomposes the request into steps and chooses tools, connectors, skills, and optional experts.
4. Permission policy evaluates the task and tool plan:
   - Read-only and low-risk actions can proceed under guarded mode.
   - File writes, external sends, destructive operations, payment/legal/high-risk actions require approval.
   - Elevated mode can reduce prompts but must remain auditable.
5. Agent runtime streams progress events to the conversation.
6. File outputs are registered as artifacts.
7. File changes are registered with diffs.
8. Results panel updates in real time.
9. User can follow up, stop, retry, resume, archive, or share/download outputs.

## Artifact Pipeline

Artifacts should be first-class. The initial supported types:

- Markdown and plain text reports.
- HTML preview documents.
- CSV and XLSX spreadsheets.
- Chart images generated from tabular data.
- PDF exports from reports/previews.
- PPTX slide decks.
- ZIP bundles for multi-file outputs.

The agent should never leave generated files as untracked side effects. Every output must be attached to the task artifact manifest.

## File And Permission Model

Guarded mode is the default:

- User grants explicit work directories.
- Reads are limited to granted folders and uploaded attachments.
- Writes are limited to task output directories unless user approves writing in place.
- External sends, posting, deletion, overwriting, and broad batch operations require confirmation.
- Every tool action is logged with task id, tool name, target path/service, result, and risk level.

Full-access mode belongs after guarded permissions are enforced. When introduced, it still shows a summary of planned risky actions before execution.

## Remote Control

Remote Control should support Slack, Telegram, and Discord first. Additional platforms can follow after the architecture is stable.

Remote behavior:

- User connects a messaging platform.
- Incoming DM creates or resumes an assistant task.
- Progress updates are sent back to the message thread.
- Sensitive operations ask for confirmation in the same platform.
- The local desktop/server must be online for local file operations.

The adapter contract should normalize platform events into:

- user id
- workspace id
- message text
- attachments
- thread/conversation id
- reply target

## Automations

Automations are scheduled assistant tasks:

- One-time, hourly, daily, weekly, monthly schedules.
- Prompt, workspace, files/context, model, connector selection, permission profile.
- Run history with status, outputs, logs, and approvals.
- Notification channel for start, blocked, completed, failed.

Automations should reuse the same task execution pipeline as manual tasks.

## Memory

Memory is user-visible and editable:

- Remembered preferences.
- Project/workspace summaries.
- Imported summaries from external chat history.
- Task-derived nightly summaries.
- Forget selected memory.
- Edit memory.

Memory retrieval must be scoped to the selected workspace unless the user explicitly enables global memory.

## Expert Center Integration

Expert Center remains `/agents`.

Assistant integration points:

- "Use Expert" command in the composer.
- Suggested expert/team when the planner detects a specialized task.
- Expert task runs still produce artifacts, messages, and changes in the same assistant task result model.
- Expert Center can remain more advanced and catalog-driven.

## Error Handling

The user should see actionable blocked states:

- Needs permission.
- Needs connector setup.
- Needs missing file/folder.
- Waiting for remote desktop/server.
- Model/provider unavailable.
- Tool failed and can retry.
- Automation skipped or failed.

Every failed task should keep its partial messages, logs, and artifacts.

## Testing Strategy

Start with focused tests around behavior and route integration:

- `/assistant` renders primary workstation layout and links to Expert Center.
- Creating a task posts the full composer payload.
- Task list shows running, completed, failed, blocked, and archived tasks.
- Results panel renders artifacts, files, changes, and preview from API data.
- Guarded permission blocks risky file/external operations until approval.
- Remote webhook creates a task and returns progress response for Slack-style events.
- Automation creation schedules a reusable assistant task.
- Memory edit/forget/import updates visible memory state.

Backend tests should cover task lifecycle, artifact registration, permission decisions, connector lookup, automation scheduling, and remote event normalization.

## Phased Implementation

Phase 1: Primary `/assistant` shell and task lifecycle

- Create route and workstation layout.
- Add task list, conversation, composer, and result tabs.
- Wire to task create/list/detail APIs.
- Preserve `/agents` and link to it as Expert Center.

Phase 2: Artifact and file operations

- Register generated artifacts.
- Implement All Files, Changes, and Preview tabs with real API data.
- Add guarded folder/work-directory authorization.

Phase 3: Skills, connectors, and expert escalation

- Make skills/connectors selectable at task time.
- Add connector status and MCP capability discovery.
- Add assistant-to-expert escalation.

Phase 4: Remote control and automations

- Add Slack/Telegram/Discord adapter contract.
- Add scheduler-backed automations and run history.
- Add notification delivery.

Phase 5: Memory and polish

- Add memory edit/import/forget.
- Add nightly summaries.
- Add model/provider validation and preference persistence.

## Non-Goals For First Implementation

- Do not delete `/agents`.
- Do not rebuild Tauri packaging first.
- Do not implement every messaging platform at once.
- Do not promise unrestricted local computer control before guarded permissions work.
- Do not force business departments into the primary assistant experience.

## Source References

- Tencent WorkBuddy Overview: https://www.workbuddy.ai/docs/workbuddy/
- Tencent WorkBuddy Claw Remote Control: https://www.workbuddy.ai/docs/workbuddy/Claw
- Tencent WorkBuddy Tips & Tricks: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Efficient-Tips
- Tencent WorkBuddy Slack remote workflow: https://www.workbuddy.ai/docs/workbuddy/From-Beginner-to-Expert-Guide/Practice-Cases/Claw-Slack
