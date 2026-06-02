# Missing `sub_agent_jobs` Database Schema Migration

## Problem Statement
The KAIROS Orchestrator utilizes a `sub_agent_jobs` table to manage and queue jobs that sub-agents execute in the background. While the logic to handle these jobs was implemented in `src/server/orchestration/queue/pg_queue.rs` and `sqlite_queue.rs`, the actual database schema migration to create the `sub_agent_jobs` table was never included in the initialization process. As a result, starting up a new cloud-native instance leads to catastrophic query failures when the Orchestrator attempts to poll or insert into the missing queue table.

Furthermore, it is an absolute requirement that all tenant data on the platform be strictly isolated using PostgreSQL Row-Level Security (RLS). A missing schema means that we are also missing the RLS enforcement policy for this table.

## Research Report
- Evaluated `docs/technical/architecture/kairos/sub-agent-queue-design.md`, which defines the `sub_agent_queue` schema. The code references `sub_agent_jobs` now.
- I searched the migration history `src/server/migrations/*.sql` and found no instances of `CREATE TABLE sub_agent_jobs`.
- Other similar tables (e.g. `sub_agent_queue` which is deprecated but present in older code) had RLS applied, but `sub_agent_jobs` was completely missing.

## Design Doc
We will implement the schema natively in the backend with full RLS support:

### Database Schema
We need a standard SQL table `sub_agent_jobs`:
- `id`: VARCHAR PRIMARY KEY
- `tenant_id`: VARCHAR NOT NULL (Tenant Identifier)
- `parent_task_id`: VARCHAR
- `agent_role`: VARCHAR
- `payload`: JSONB
- `status`: VARCHAR DEFAULT 'QUEUED'
- `attempts`: INTEGER DEFAULT 0
- `max_attempts`: INTEGER DEFAULT 3
- `run_after`: TIMESTAMP WITH TIME ZONE
- `locked_until`: TIMESTAMP WITH TIME ZONE
- `created_at`: TIMESTAMP WITH TIME ZONE
- `updated_at`: TIMESTAMP WITH TIME ZONE

### RLS Policies
The table must enable row-level security:
- `ALTER TABLE sub_agent_jobs ENABLE ROW LEVEL SECURITY;`
- `CREATE POLICY tenant_isolation_sub_agent_jobs ON sub_agent_jobs FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));`

### Implementation Prompt
"Create a new database migration file `src/server/migrations/058_sub_agent_jobs.sql` that defines the `sub_agent_jobs` table schema exactly as requested in the design document, making sure to apply PostgreSQL Row Level Security (RLS) on the `tenant_id`."

## Priority
P0

## Estimated Scope
Small
