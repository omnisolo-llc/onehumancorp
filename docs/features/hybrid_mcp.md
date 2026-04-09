<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid MCP Capabilities

The One Human Corp (OHC) platform is designed to operate seamlessly across both Cloud-Native and Standalone execution environments. To empower our agents with contextual awareness and file system operations regardless of the mode, OHC leverages the **Hybrid Model Context Protocol (MCP)** architecture.

## 1. Hybrid MCP RAG Protocol

The Hybrid MCP RAG Protocol acts as the bridge between local offline execution and cloud scalability, ensuring agents maintain perfect memory persistence without compromising data sovereignty.

### The Privacy vs. Scalability Problem
Most Agentic OS solutions force users to choose between local privacy with limited compute, or cloud scalability with full data exfiltration. The Hybrid MCP RAG Protocol solves this by implementing a **Local Default** strategy.

### Synchronization Architecture
- **Standalone Mode (Local):** Agents extract insights and store semantic summaries in the local SQLite database.
- **Background Sync:** A lightweight Sync Daemon observes the local SQLite instance. When an internet connection is established and the agent escalates a task, the daemon batches pending RAG context.
- **Cloud Convergence:** Encrypted payloads are sent to the Cloud Gateway via SPIFFE/SPIRE mutually authenticated TLS. The cloud Postgres database merges these insights using Last-Write-Wins (LWW) mechanisms, maintaining a globally synchronized semantic search context.

```mermaid
graph TD
    A[Standalone Worker] -->|Private Local State| B[(SQLite DB)]
    B -.->|Sync Daemon| C{API Gateway}
    C -->|Aggregated Context| D[(PostgreSQL / pgvector)]
    D -->|Semantic Memory| E[Cloud Swarm]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

## 2. Hybrid File System MCP Proxy

To handle file manipulations (like reading configurations or writing generated code) across disparate environments, OHC abstracts filesystem operations into a unified Hybrid File System MCP Proxy.

### Unified Interface
The `mcp.FileSystemProvider` interface ensures that standard MCP tools (`read_file`, `write_file`, `list_directory`) function identically regardless of the underlying infrastructure.

### Execution Modes
- **Cloud Implementation:** Access is chrooted and scoped using `auth.Claims` to tenant-specific Kubernetes Persistent Volumes (PVs) or virtualized S3-backed endpoints. This prevents cross-tenant data leaks.
- **Standalone Implementation:** Maps directly to local file systems with strict safety bounds, ensuring the agent only modifies authorized workspace directories.

By leveraging these Hybrid MCP capabilities, the OHC Swarm retains its absolute autonomy, gracefully transitioning between complete offline privacy and immense cloud computation.

</div>
