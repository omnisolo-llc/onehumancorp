<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Agentic OS Features Test Plan

1. **Shared Task List Tests:** Verify state transitions (`PENDING` -> `IN_PROGRESS` -> `REVIEW` -> `COMPLETED`) are strictly enforced and distributed locks prevent concurrent state changes.
2. **Teammate Mesh Tests:** Ensure messages are delivered correctly and consistently over both Redis (Cloud) and In-Memory (Standalone) transports.
3. **Orchestration Queue Tests:** Validate enqueue/dequeue mechanics, retry policies, and timeout handling for both Redis and SQLite queue implementations.
4. **AutoDream Pipeline Tests:** Verify episodic memory is correctly batched, passed through LLM embeddings, and stored in pgvector.

</div>
