# Issue Brief: Data Model Architecture Evolution

## Title
Data Model Architecture: Entities, Relationships, and Multi-Tenancy Guarantees

## Problem Statement
As OneHumanCorp scales to support diverse business types—from bakers and freelance handymen to boutique owners—the underlying data model must remain robust, scalable, and strictly isolated per tenant. A non-technical small business owner relies on the system to keep their customer data, orders, and AI agent memories perfectly secure and separate from others. We must define clear entity relationships, access patterns, and invariants that guarantee row-level multi-tenancy without adding complexity to the business owner's experience.

## Research Report
- **Goal**: Review and evolve the OHC data model to ensure complete tenant isolation and optimized access patterns for both the mobile-first UI and the background AI agents.
- **Findings**:
  - **Multi-Tenancy**: The current architecture mandates row-level isolation in PostgreSQL using a `tenant_id` column with `ENABLE ROW LEVEL SECURITY`. This is critical and must be strictly maintained.
  - **Entity Types**: Key entities include Business (Tenant), Product, Order, Customer, Agent, Page, Booking, and Memory.
  - **Access Patterns**:
    - AI agents need fast access to customer history and long-term memory (pgvector).
    - The mobile app requires low-latency queries for orders and analytics.
- **Competitive Analysis**: Shopify and Wix handle multi-tenancy seamlessly but often struggle with deep AI integration at the data layer. By building pgvector memories directly into the tenant schema, OHC gains a significant advantage in personalized AI operations.

## Design Doc

### Entity-Relationship Diagram
...

### Key Invariants
1. **Tenant Isolation**: A business owner can only access data where `tenant_id` matches their authenticated session. This is enforced at the database level using RLS.
2. **Agent Scoping**: AI agents operating on behalf of a tenant must have their database queries automatically scoped to that `tenant_id`.
3. **Data Residency**: All entities (Products, Orders, Customers, Memories) must explicitly reference a `tenant_id`.

### Migration Strategy
- When evolving the schema (e.g., adding new entities like `Subscription`), use zero-downtime migrations.
- Ensure every new table includes a `tenant_id` column and the corresponding RLS policies are applied immediately upon creation.

## Implementation Prompt
Implement the data model enhancements for the OHC platform. Ensure that all new tables include a `tenant_id` column and that Row Level Security (RLS) is enabled and configured correctly. Update the Go backend repository layer to pass the `tenant_id` context in all queries. Implement E2E tests verifying that a user from one tenant cannot access data from another tenant, even via API manipulation.

## Priority
P0

## Estimated Scope
Medium

## Database Evolution and Migration Strategy

### 14. Zero-Downtime Migrations
As the OHC platform rapidly evolves, the database schema will change frequently. We must implement a strict zero-downtime migration strategy using tools like `gh-ost` or similar proxy layers. Schema changes must be backward compatible, utilizing a multi-step rollout process (add column, dual write, backfill, switch read, drop old column) to ensure the platform never goes offline during an update.

### 15. Change Data Capture (CDC) Pipeline
To feed the event mesh and analytical data warehouses, we cannot rely on application-level event emission alone, as bugs might miss events. We must implement a robust Change Data Capture (CDC) pipeline (e.g., Debezium) directly on the PostgreSQL WAL (Write-Ahead Log). This guarantees that every single database mutation is captured and streamed to the NATS mesh reliably.

### 16. Tenant-Aware Data Analytics
Providing insights to the business owner requires running complex analytical queries over their data. Running these queries on the primary transactional database is dangerous. We must stream tenant data (via the CDC pipeline) to a dedicated OLAP data warehouse (like ClickHouse or Snowflake), partitioned by tenant ID, where The Business Advisor agent can run heavy aggregations without impacting the performance of the live storefronts.

### 17. B-Tree vs Hash Indexing
### 18. Write-Ahead Log Monitoring
### 19. Partition Pruning
### 20. Connection Leak Detection
### 21. B-Tree vs Hash Indexing
### 22. Write-Ahead Log Monitoring
### 23. Partition Pruning
### 24. Connection Leak Detection
