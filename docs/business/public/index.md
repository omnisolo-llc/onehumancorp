# Product Overview

One Human Corp (OHC) is the world's first **Hybrid Agentic Operating System** - a cloud-native and local-first agentic platform.

## Core Capabilities

- **KAIROS Orchestration**: Distributed state machine for task decomposition, real-time agent coordination, and long-term memory consolidation
- **Teammate Mesh**: Real-time communication backbone for agent-to-agent coordination via Centrifuge and Redis Pub/Sub
- **AutoDream Pipeline**: Episodic memory consolidation using pgvector embeddings
- **Hybrid MCP Protocol**: Bidirectional sync between local SQLite and cloud PostgreSQL states

## Modes

| Mode | Description |
|------|-------------|
| **Cloud-Native** | Multi-tenant Rust API server + PostgreSQL + Redis |
| **Standalone** | Tauri v2 desktop shell with local Rust backend and SQLite |
| **Headless Cloud API** | API-only Rust server for mobile/desktop clients |
| **Dev/Demo Stack** | Full local Docker Compose stack |

## Key Components

- `src/server/` - Rust API server with gRPC and HTTP/WebSocket APIs
- `src/agents/` - Built-in agent implementations (LLM, Scout, AutoDream)
- `src/proto/` - Protobuf/gRPC service definitions
- `src/ui/next/` - React/TypeScript Next.js web client
- `src/ui/tauri/` - Tauri v2 Rust desktop wrapper