---
status: BLOCKED
agent: Palette
blockers:
  - Domain violation: Requires file modifications outside my exclusive domain (apps/web/, apps/mobile/, apps/desktop/).
agent: Implementer
title: "Implement KAIROS Interactive API Playbook"
priority: P0
estimated_scope: Large
---
<div style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

<h1>Title: Implement KAIROS Interactive API Playbook</h1>

<h2>Problem Statement</h2>
<p>The OHC swarm requires a distributed Shared Task List, a realtime Teammate Mesh, and an AutoDream vector memory pipeline for true autonomous orchestration in both cloud-native and standalone deployments. We need to formalize these interfaces for sub-agents.</p>

<h2>Research Report</h2>
<p>We analyzed the orchestration needs across deployment modes.</p>

<table border="1">
  <tr>
    <th>Feature</th>
    <th>Cloud-Native Mode</th>
    <th>Standalone Mode</th>
  </tr>
  <tr>
    <td>Shared Task List</td>
    <td>PostgreSQL (FOR UPDATE SKIP LOCKED)</td>
    <td>SQLite (Transactions & Mutex)</td>
  </tr>
  <tr>
    <td>Teammate Mesh</td>
    <td>Redis Pub/Sub</td>
    <td>In-Memory Go Channels</td>
  </tr>
  <tr>
    <td>AutoDream Vector</td>
    <td>pgvector</td>
    <td>Local Blob Embeddings</td>
  </tr>
</table>

<pre><code class="language-mermaid">
graph TD
    A[Agent] --&gt;|Write| B(.agent-task/memory)
    B --&gt;|Watched By| C(AutoDream Pipeline)
    C --&gt; D[pgvector ohc_memory_embeddings]
</code></pre>

<h2>Design Doc</h2>
<ol>
  <li><b>Shared Task List</b>: <code>shared_tasks_v4</code> schema using <code>VARCHAR PRIMARY KEY</code> for compatibility.</li>
  <li><b>Teammate Mesh</b>: Implement event structs with <code>agent_id</code>, <code>action</code>, <code>status</code> across <code>mesh:tasks</code> and <code>mesh:coordination</code>.</li>
  <li><b>AutoDream</b>: Polls memory directory, embeds content, stores in <code>ohc_memory_embeddings</code>.</li>
</ol>

<h2>Implementation Prompt</h2>
<p>Hello Implementer!</p>
<ol>
  <li>Utilize the existing <code>shared_tasks_v4</code> migration in <code>srcs/server/db/migrations/047_shared_tasks_v4.sql</code>.</li>
  <li>Implement the <code>SharedTaskOrchestrator</code> to interface with this database.</li>
  <li>Implement the Teammate Mesh APIs for Redis/Memory transports.</li>
  <li>Build the AutoDream pipeline to sync <code>.agent-task/memory/</code> into <code>ohc_memory_embeddings</code>.</li>
</ol>
</div>
