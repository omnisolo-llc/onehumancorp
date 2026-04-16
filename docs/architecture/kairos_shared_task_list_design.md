<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">
<h1 style="color: #FFFFFF; font-weight: 600; letter-spacing: -0.02em;">KAIROS: Shared Task List & Teammate Mesh Architecture</h1>

<h2 style="color: #B0B0B0; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 0.5rem;">1. Executive Summary</h2>
<p>The OHC Hybrid Agentic OS requires a deeply integrated Teammate Mesh and Shared Task List to orchestrate autonomous agents. This document details the PostgreSQL schema for task decomposition, the Redis Pub/Sub integration for real-time coordination, and the pgvector pipelines for long-term "AutoDream" memory consolidation.</p>

<h2 style="color: #B0B0B0; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 0.5rem;">2. Database Architecture (PostgreSQL)</h2>
<p>The <code>ohc_tasks</code> schema provides the durable state for task decomposition and tracking.</p>
<pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>
CREATE TABLE ohc_tasks.mission_queue (
    mission_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'QUEUED', -- QUEUED, IN_PROGRESS, BLOCKED, DONE
    assigned_agent VARCHAR(100),
    priority VARCHAR(10) NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ohc_tasks.mesh_locks (
    resource_id VARCHAR(255) PRIMARY KEY,
    locked_by VARCHAR(100) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);
</code></pre>

<h2 style="color: #B0B0B0; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 0.5rem;">3. Teammate Mesh (Redis Pub/Sub)</h2>
<p>Agents coordinate via the following Redis channels to ensure real-time synchronization:</p>
<ul>
    <li><code>mesh:events:task_created</code> - Emitted when a new task is added.</li>
    <li><code>mesh:events:status_update</code> - Emitted on transition (e.g., IN_PROGRESS to DONE).</li>
    <li><code>mesh:locks:acquire</code> - Distributed locking for file/resource access.</li>
</ul>

<h2 style="color: #B0B0B0; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 0.5rem;">4. AutoDream Consolidation (pgvector)</h2>
<p>Completed missions and memory synopses are vectorized for Swarm Intelligence:</p>
<pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>
CREATE TABLE ohc_memory.autodream_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    embedding vector(1536),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
</code></pre>
</div>
