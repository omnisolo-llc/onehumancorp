# Research Report: OHC Data Model Architecture

## 1. Overview
This research report outlines the unified data model architecture for the OneHumanCorp (OHC) platform. A robust data foundation is critical for enabling small business owners to run varied businesses (e.g., physical goods, services, digital downloads) while leveraging seamless AI background operations. The architecture strictly enforces multi-tenancy and is designed to securely decouple operations using Row-Level Security (RLS).

## 2. Findings
- **Data Model Flexibility**: The data model requires handling a diverse set of products—from digital downloads and services to physical goods and food/beverage pre-orders. A unified `PRODUCT` and `PRODUCT_VARIANT` structure with an extensible JSONB `attributes` column provides this flexibility without schema bloat.
- **Multi-Tenancy Enforcement**: A critical invariant is strong multi-tenancy. Rather than relying entirely on application-level filtering, PostgreSQL's Row-Level Security (RLS) policies on every table ensure cross-tenant data leaks are impossible at the database level.
- **AI Agent Memory Integration**: The `AGENT_MEMORY` table utilizes the `pgvector` extension. This allows the various "Departments" (e.g., The Ambassador, The Manager) to perform efficient similarity searches, recalling past context and interactions specific to the tenant without hallucinating external business data.
- **Transactional Integrity**: Financial and operational flows must rely on strict state machines. The `ORDER`, `PAYMENT`, and `BOOKING` entities are designed to progress through definitive statuses (`pending`, `paid`, `fulfilled`), ensuring deterministic behavior for both human oversight and automated AI tasks.
- **Schema Evolution Strategy**: Implementing zero-downtime additive migrations minimizes disruption to the platform while preserving performance and reliability. Using JSONB allows configuration flexibility, enabling dynamic tenant settings without constant schema changes.

## 3. Data Model Design Artifact
The fully fleshed-out data model, complete with an Entity-Relationship (ER) diagram, key invariants, and access patterns, has been documented in `docs/research/[architecture]_data_model.md`.

## 4. Next Steps & Proposed Architecture
- Develop the initial Go database layer, implementing an ORM or query builder wrapper that securely injects the `tenant_id` context into every query.
- Create initial database migration scripts implementing the defined schema, incorporating foreign key relationships tied to `tenant_id` and initial RLS policies.
- Configure and test the `pgvector` extension for `AGENT_MEMORY` and validate isolated semantic searches across different simulated tenants.
- Integrate these database patterns with the existing Rust API architecture as documented in `README.md`.