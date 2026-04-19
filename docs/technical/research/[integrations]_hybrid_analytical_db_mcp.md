# Title: [integrations] Hybrid Analytical Database (OLAP) MCP

## Problem Statement
One Human Corp (OHC) agents are increasingly tasked with processing massive datasets, including swarm logs, market telemetry, and diverse file formats like Parquet, CSV, and JSON. While OHC has robust transactional support via SQLite and PostgreSQL, these row-oriented databases are inefficient for complex analytical (OLAP) workloads. Agents currently lack a high-performance, vectorized engine for sub-second analytical reasoning, leading to latency bottlenecks and excessive compute consumption when performing aggregations or joins over millions of rows in the Standalone Desktop environment.

## Research Report
Market analysis and internal benchmarking (e.g., TPC-H datasets) reveal that DuckDB, an in-process columnar database, provides a 10x-1000x performance advantage over SQLite for analytical queries.
- **DuckDB (Local-First):** DuckDB's vectorized execution engine is specifically optimized for OLAP. It can query Parquet files directly without an import step and integrates natively with Go via `database/sql`.
- **MotherDuck (Cloud-Native):** MotherDuck extends DuckDB to the cloud, providing a serverless data warehouse that is fully compatible with the DuckDB dialect.
- **OHC "Unfair Advantage":** By integrating DuckDB/MotherDuck as a Hybrid MCP Tool, OHC agents can perform intensive analytics locally in Standalone mode and seamlessly "burst" to MotherDuck for multi-tenant, cloud-scale data warehousing in Cloud-native mode. This aligns with OHC's "Aesthetic Excellence" by powering data-rich, real-time dashboards with zero friction.

## Design Doc
**Architecture:**
- Add a new package `srcs/server/lib/integrations/hybrid_olap/`.
- Introduce an `OLAPManager` implementing the MCP Tool interface.
- Dynamically route requests based on `os.Getenv("OHC_MULTITENANT") == "true"`.
  - **Standalone Mode:** Utilize the local `duckdb` driver. Support direct attachment of local Parquet/CSV files.
  - **Cloud-Native Mode:** Connect to `MotherDuck` using the `md:` connection string. Enforce tenant isolation by scoping queries to organization-specific databases or schemas.

**API Contracts:**
- `ExecuteQuery(ctx context.Context, sql string, params map[string]interface{}) ([]map[string]interface{}, error)`
- `IngestFile(ctx context.Context, path string, tableName string) error` (Supports Parquet, CSV, JSON)

**Security:**
- Cloud mode MUST validate `organization_id` to ensure strict tenant isolation.
- Prevent arbitrary shell execution via DuckDB extensions by restricting `INSTALL` and `LOAD` commands to a pre-approved allowlist.

## Implementation Prompt
"Implement the Hybrid Analytical Database (OLAP) MCP tool in `srcs/server/lib/integrations/hybrid_olap/`.
1. Create `olap.go` defining the `OLAPManager` and its MCP capabilities (`ExecuteQuery` and `IngestFile`).
2. Implement environment-aware driver selection. If `OHC_MULTITENANT` is \"true\", use the MotherDuck connection string (`md:mydb?token=...`). Otherwise, use a local DuckDB instance.
3. For `IngestFile`, utilize DuckDB's native `read_parquet`, `read_csv`, and `read_json` functions to enable zero-copy ingestion.
4. Ensure 100% test coverage in `olap_test.go` using a mockable database interface.
5. Create an E2E test fulfilling the OHC E2E Test Standard: login -> navigate to the \"Swarm Analytics\" dashboard -> trigger a complex aggregation over a mock 10k-row Parquet file -> verify the result is displayed with OHC's Premium glassmorphism aesthetics.
6. Update the `BUILD.bazel` file to include `github.com/duckdb/duckdb-go/v2` and other necessary dependencies."

## Priority
P1

## Estimated Scope
Medium
