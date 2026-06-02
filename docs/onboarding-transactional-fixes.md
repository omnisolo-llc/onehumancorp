# Onboarding Transactional Fixes

## Overview
The `start_onboarding` method in the OHC platform previously executed multiple database modifications (tenant creation, user creation, agent seeding, and product creation) outside of a unified database transaction. Concurrent operations or partial failures during this sequence could lead to inconsistent data states or orphaned records (e.g., a tenant is created but the admin user fails to register).

## Findings
1. **Lack of Transactional Boundary:**
   Initially, database calls were made either directly on the global `&self.db.pool` or inside disjoint `tokio::task::spawn` blocks, executing futures concurrently.

2. **Missing Field Validations:**
   The `StartOnboardingRequest` could contain invalid or empty strings for essential fields (like `admin_email`, `company_name`, `business_type`), leading to silently creating malformed business records in the system.

3. **Premature Event Publishing:**
   `TeammateMeshEvent` instances (e.g., `ProductCreated`, `GenerateStorefront`, `GeneratePolicies`) were being pushed to the Orchestrator before ensuring the dependent entities were successfully persisted to the database. If a database insert failed late in the cycle, the Orchestrator would still spawn asynchronous side-effects for phantom users.

## Implemented Solutions
1. **Unified `sqlx::Transaction`:**
   Modified `start_onboarding` in `src/server/services/onboarding/onboarding_agent.rs` to initialize a single `tx = self.db.pool.begin().await` context. All mutations (tenants, users, products, sub-agent queues, agent subscriptions, onboarding states) now execute sequentially and are bound to `&mut *tx`.

2. **Error and State Recovery (Rollbacks):**
   If any mutation fails, the function propagates the error and `tx` goes out of scope, causing PostgreSQL/SQLite to automatically rollback the transaction, ensuring zero partial-state pollution.

3. **Post-Commit Event Deferral:**
   All `TeammateMeshEvent` structs are now accumulated in a local `Vec` array and *only* published to `self.hub` *after* `tx.commit().await` has succeeded.

4. **Input Validations:**
   Added explicit `trim().is_empty()` checks and default fallbacks for core variables (e.g., fallback to `"admin@ohc.app"`, `"My Business"`, etc.) and hard-rejected completely invalid payloads such as email addresses without an `@` character.
