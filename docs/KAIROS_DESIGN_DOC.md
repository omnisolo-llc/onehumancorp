# KAIROS AI OS Orchestration: Unified Architecture

This document outlines the KAIROS Hybrid AI OS Orchestration layer, aligning with OHC's Vision and Market Reality.

## 1. Shared Task List (The Brain)
A durable state machine decomposing complex tasks.
- **Cloud-Native:** Uses PostgreSQL `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency.
- **Standalone:** Degrades to SQLite transactions with application-level locking.

## 2. Teammate Mesh (The Nerves)
A realtime coordination mesh.
- Relies on Redis Pub/Sub (`rueidis`) and `CentrifugeNode` for high throughput.
- Enforces Zero Secrets by authenticating purely via SPIFFE/SPIRE.

## 3. AutoDream (The Memory)
A pipeline for semantic long-term memory.
- Ephemeral session data is embedded using LLMs and stored in `pgvector`.
- Grants the Swarm intelligent recall, reducing token usage across operations.

## Aesthetic Mandate
All frontend artifacts representing this architecture must strictly enforce:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
