<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: [integrations] Hybrid Analytical Database (OLAP) MCP

## Problem Statement
OHC agents often need to perform complex analytical queries (aggregations, joins over millions of rows, trend analysis) to provide "Market Reality" insights. While SQLite is excellent for transactional state in Standalone mode, it is not optimized for OLAP (Online Analytical Processing) workloads, leading to slow performance and timeouts for complex agent tasks. In the Cloud, PostgreSQL is the primary store, but even then, large-scale analytics can put undue pressure on the production database.

## Research Report
The emerging standard for high-performance, embedded analytics is **DuckDB**. It offers vectorized execution and is highly compatible with the `database/sql` interface in Go. **MotherDuck** provides a cloud-native extension of DuckDB, enabling a hybrid model where local data can be seamlessly joined with cloud-scale datasets.

### Competitive Analysis
| Feature | SQLite (Current Standalone) | PostgreSQL (Current Cloud) | OHC Hybrid OLAP (DuckDB + MotherDuck) |
| :--- | :--- | :--- | :--- |
| **Analytical Speed** | Low | Medium | ✅ High (Vectorized) |
| **Cloud-Local Sync** | Manual/Custom | Complex Replication | ✅ Native (MotherDuck) |
| **Resource Usage** | Low | High | ✅ Variable (Low for local) |

### Key Technologies
- **DuckDB**: For local vectorized execution.
- **MotherDuck**: For cloud-scale analytics and seamless state sync.
- **`github.com/duckdb/duckdb-go/v2`**: Go client for DuckDB.

## Design Doc
**Architecture:**
- **Hybrid OLAP Manager**: Implements the MCP Tool interface.
- **Standalone Mode**: Initializes an embedded DuckDB instance. It can ingest data from the local SQLite SIPDB into DuckDB memory for fast analysis.
- **Cloud Mode**: Connects to MotherDuck using the `md:` prefix and `MOTHERDUCK_TOKEN`.
- **Hybrid Queries**: Allows agents to run SQL queries that "bridge" local data and cloud data.

**API Contracts:**
- `ExecuteQuery(sql string) (ResultSet, error)`
- `IngestData(source_url string, table_name string) error`

**Security:**
- Enforce strict `organization_id` scoping in Cloud/MotherDuck mode.
- Use `BwrapRunner` to ensure DuckDB execution is isolated from the host filesystem unless explicit paths are granted.

## Implementation Prompt
"Implement the Hybrid Analytical Database (OLAP) MCP tool in `srcs/server/lib/integrations/olap/`.
1. Create `olap.go` defining the `OLAPManager` MCP tool.
2. Integrate `github.com/duckdb/duckdb-go/v2` as the primary driver.
3. In Standalone mode, use a local `.duckdb` file for persistence.
4. In Cloud mode, use the `MOTHERDUCK_TOKEN` from environment variables to connect to a remote MotherDuck instance.
5. Provide MCP tools: `run_analytics_query` (takes SQL, returns JSON results) and `import_csv_to_analytics` (takes a path or URL).
6. Ensure 100% test coverage using the standard `database/sql` mocking or a temporary local DuckDB file.
7. Add an E2E test verifying that an agent can calculate a complex aggregation over a mocked dataset in DuckDB."

## Priority
P2

## Estimated Scope
Medium

</div>
