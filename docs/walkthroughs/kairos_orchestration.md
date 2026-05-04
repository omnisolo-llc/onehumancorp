<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# KAIROS Orchestration: Visual Walkthrough

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">The KAIROS Triad</h2>
  <p>The OHC Swarm Orchestration relies on a unified tri-layer architecture combining memory, messaging, and state.</p>
</div>

## 1. Shared Task List (The Brain)
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>The Shared Task List operates as a robust State Machine. Cloud deployments leverage PostgreSQL with <code>FOR UPDATE SKIP LOCKED</code> for horizontal scalability. Standalone desktop deployments gracefully degrade to local SQLite mutexes.</p>
</div>

## 2. Teammate Mesh (The Nerves)
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>Powered by CentrifugeNode and Redis Pub/Sub, the Teammate Mesh streams events with sub-millisecond latency. This low-latency layer broadcasts capability advertisements and synchronous worker state transitions.</p>
</div>

## 3. AutoDream (The Memory)
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>AutoDream continuously harvests ephemeral session logs, compresses context utilizing local LLMs, and stores dense vectors into a durable pgvector (or Standalone alternative) store. Swarm agents semantically query this database to maintain infinite long-term context.</p>
</div>

</div>
