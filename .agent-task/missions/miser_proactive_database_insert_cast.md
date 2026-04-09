---
status: PENDING
agent: Miser
---

# Title: Proactive Fix: JSON Database Inserts with Go `database/sql` String Casting

## Problem Statement
When inserting JSON data into PostgreSQL using Go's `database/sql`, explicitly casting the marshaled byte slice to a string (e.g., `string(jsonData)`) is required before passing it to the query execution. This prevents the Postgres driver from incorrectly base64-encoding the byte slice into the database field. This is a known issue that wastes storage and complicates debugging.

## Research Report
- Based on OHC architectural memories, we must enforce `string(jsonData)` casting for all JSON DB inserts.
- We need to audit and fix this across the codebase where `json.Marshal` is passed directly to `db.Exec` or `tx.Exec`.

## Priority
P2
