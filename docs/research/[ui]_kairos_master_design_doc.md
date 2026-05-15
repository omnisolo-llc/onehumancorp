### Title
Master Design Doc: KAIROS AI OS Orchestration (Phase 4)

### Problem Statement
The OHC Swarm requires absolute autonomy to effectively empower small business owners with zero technical knowledge. This requires a durable, distributed state machine, background queuing logic, and a highly available realtime communication layer. KAIROS Orchestration is the architectural consolidation that realizes this requirement by leveraging a durable database schema and microservices to decompose high-level feature requests for the agent team, along with deep-deliberation cycles.

### Architecture
The absolute autonomy of the OHC Swarm rests on three pillars (The KAIROS Triad):
1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh (The Nerves):** A highly available, low-latency communication layer. Using `CentrifugeNode` and Redis Pub/Sub (`rueidis`), agents broadcast state changes, advertise capabilities, and stream events.
3. **AutoDream (The Memory):** The long-term persistence layer. Ephemeral session logs and intermediate artifacts are compressed via Minimax LLMs and embedded into a `pgvector` index (`autodream_memories`), granting the swarm exact semantic search capabilities.

```mermaid
graph TD
    subgraph Swarm
        A1[Worker Agent 1]
        A2[Worker Agent 2]
    end

    subgraph Teammate Mesh (Redis/Centrifugo)
        M[Mesh Hub]
    end

    subgraph KAIROS Orchestrator
        T[(Shared Task List / DB)]
        AD[AutoDream Pipeline]
        V[(pgvector Memories)]
    end

    A1 <-->|Pub/Sub| M
    A2 <-->|Pub/Sub| M

    A1 -->|Claim Task| T
    A2 -->|Claim Task| T

    T -.->|Completions| AD
    AD -->|Embed| V
    A1 -->|Semantic Search| V

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A1,A2,M,T,AD,V premium;
```

### UI Flow
This architectural consolidation fully conforms to the **Visual Excellence Mandate**. Downstream UI representing these architectural components or interpreting the mesh telemetry MUST reflect a polished, modern styling, applying the following CSS elements to create a premium glassmorphism effect:

```css
<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>
```

### Implementation Prompt
Implementers should focus on mapping the Swarm worker agents to the Shared Task List, ensuring cross-platform database compatibility (PostgreSQL and SQLite) leveraging row-locking semantics appropriately. Then, construct the Teammate Mesh via a Redis/Centrifugo pub-sub structure for inter-agent communication, and lastly, bridge the ephemeral state into pgvector using the AutoDream LLM pipeline for semantic search indexing. Follow the provided `mermaid` diagram to structure interactions and dependencies between the Triad components.

## Globalization and Localization

### 14. Right-to-Left (RTL) Support from Day One
To reach a truly global market (specifically the Middle East), the Kairos design system must support Right-to-Left (RTL) languages natively from inception. This requires using logical CSS properties (e.g., `margin-inline-start` instead of `margin-left`) across all components, ensuring the UI flips seamlessly when the language is changed.

### 15. Dynamic Content Truncation
Different languages take up drastically different amounts of physical space (e.g., German is often much longer than English). UI components, especially Action Cards, must gracefully handle dynamic text expansion. We must implement smart text truncation strategies with "Read More" expanders rather than relying on fixed-height containers that will break layout in other languages.

### 16. Culturally Appropriate Iconography
Icons are not universally understood. A "shopping cart" icon might make sense in the US, but a "basket" might be preferred elsewhere. The design system must allow for dynamic swapping of icon sets based on the user's selected locale to ensure maximum comprehension and cultural sensitivity.

### 17. Z-Index Management
### 18. CSS Variable Namespaces
### 19. Focus Trapping for Accessibility
### 20. Reduced Motion Preferences
### 21. Z-Index Management
### 22. CSS Variable Namespaces
### 23. Focus Trapping for Accessibility
### 24. Reduced Motion Preferences
