<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Sub-Agent Orchestration Queue (KAIROS Phase 4)

**Author:** Principal Product Architect & KAIROS Orchestrator (L7)
**Status:** Final

## 1. Executive Summary
KAIROS orchestrator decomposes tasks into sub-tasks. We need a scalable background queuing logic to spawn and manage isolated sub-agents in a production environment without blocking the main orchestrator loop. The current system lacks distributed tracking for these sub-agent tasks.

## 2. Research Report
- Evaluated BullMQ vs Celery vs native DB queues.
- Given our existing DB infrastructure and requirement for Standalone Mode support without external dependencies like Redis, a Database-backed queue (using the `sub_agent_queue` from Phase 4) with worker polling or LISTEN/NOTIFY is optimal.
- The queue will support worker IDs, task status updates, and parent task linkages.

## 3. Design & Architecture
- **Database Schema**: Implement `sub_agent_queue` table with fields `id UUID`, `organization_id VARCHAR`, `parent_task_id UUID`, `payload JSONB`, `status VARCHAR`, and `worker_id VARCHAR`, `created_at TIMESTAMP`, `updated_at TIMESTAMP`.
- **Queue Interface**: `TaskQueue` with methods `Enqueue(task)`, `Dequeue()`, `Acknowledge(task_id)`.
- **Worker Pool**: A dynamic pool of worker goroutines that pull from the `TaskQueue` and spawn `AgentHarness` instances to execute the work.

## 4. Visual Excellence Mandate
Every interface and artifact associated with KAIROS must adhere to the OHC Premium Feel:
- **Glassmorphism:** `backdrop-filter: blur(20px) saturate(200%)`
- **Background:** `background: rgba(255, 255, 255, 0.03)`
- **Typography:** `'Outfit', 'Inter', sans-serif`

</div>
