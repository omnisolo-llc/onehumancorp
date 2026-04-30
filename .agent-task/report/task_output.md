# OHC Agentic OS Triage & Hygiene Report

## 1. Fault Triage
- **issue_category**: `cleanup`
- **Signal**: Stagnant missions looping in the `agent_missions` queue and causing head-of-line blocking.
- **Root Cause**: Missions were dequeued using `ORDER BY created_at ASC`. When a stuck mission was requeued with a refreshed `updated_at`, it retained its old `created_at` and blocked fresh missions.

## 2. Signal Hygiene
- Cleaned up obsolete definitions and dead code from `domain/organization.rs` and unused legacy clients.
- Eradicated warnings and simplified build outputs for clearer operational visibility.

## 3. Health Guardianship
- Added `mission_sync_backlog` and `hybrid_mode_ready` metrics to the core `/health` probe in `hub.rs`.
- This ensures the OS can accurately identify when local-to-cloud mission sync gets congested.

## 4. Architectural Audit & SPIRE Violations
- Identified a hardcoded `"system"` tenant fallback in `sync_missions` and `sync_context`.
- **Fix**: Extracted the tenant `organization_id` strictly from the `x-spiffe-id` gRPC metadata header to enforce Zero Trust boundaries across multi-tenant data.

## 5. Backlog Management
- Switched the priority mechanism of `agent_missions` from `created_at ASC` to `updated_at ASC`.
- Effectively sanitizes the queue by deprioritizing actively failing/stuck missions to the end of the line.
