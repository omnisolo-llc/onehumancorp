# PowerSync Hybrid Synchronization Tool

## Problem Statement
The OHC Hybrid Architecture requires seamless offline-to-cloud synchronization between local SQLite databases and remote PostgreSQL endpoints. Currently, we need to evaluate and create blueprints for emerging tools like PowerSync to close any hybrid capability gaps for offline-to-cloud syncing.

## Research Report
PowerSync provides robust offline-first synchronization.
- **Standalone Desktop Mode:** Local single-user SQLite support via `powersync.PowerSyncDatabase` allows seamless local persistence.
- **Cloud-Native Mode:** Centralized PostgreSQL database support acts as the central source of truth. Multi-tenant sync rules can be securely mapped over standard JWT authentication workflows.
- Our initial codebase research confirms partial implementations exist (e.g., `PowerSyncIntegration` metadata and auth handling), but we need to ensure comprehensive testing and establish a firm blueprint for full integration and fallback behavior.

## Design Doc
We need to design a complete data synchronization lifecycle between `ohc_app` and `srcs/server` using PowerSync.
1. **Frontend**: Implement `PowerSyncService` to reliably manage local `PowerSyncDatabase` schemas and fallback scenarios when remote connection is unavailable.
2. **Backend**: Enhance the PowerSync Token handler and `Server` to inject proper tenant-based dynamic sync rules mapping PostgreSQL schemas.

## Implementation Prompt
Dear Implementer,
Please implement full offline-to-cloud sync leveraging the `PowerSyncService`.
- **Target File:** `srcs/app/lib/services/powersync_service.dart` and `srcs/server/dashboard/server.go`.
- **Backend:** Update the PowerSync rules generation endpoint `/api/powersync/rules` to explicitly apply `TenantID` isolation from the JWT claim.
- **Frontend:** Implement robust error-handling and auto-reconnect fallback within `BackendConnector.uploadData()`.
Ensure 100% test coverage including E2E simulation.

## Priority
P1

## Estimated Scope
Medium
