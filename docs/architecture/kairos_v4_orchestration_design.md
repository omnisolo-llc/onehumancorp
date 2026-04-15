<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">
<h1>KAIROS Orchestrator v4: Shared Task List & Sub-Agent Queue</h1>
<h2>1. Shared Task List (Phase 1)</h2>
<p>PostgreSQL uses <code>SELECT FOR UPDATE SKIP LOCKED</code> for Cloud-Native concurrency. SQLite uses explicit transactions/mutexes (<code>sync.Mutex</code> / <code>datetime('now')</code>) for Standalone execution.</p>
<h2>2. Sub-Agent Orchestration Queue (Phase 2)</h2>
<p>Cloud-Native uses Redis (via <code>rueidis</code>) Lists/Sorted Sets. Standalone uses an internal SQLite table (<code>sub_agent_jobs</code>) with locking.</p>
<h2>3. Teammate Mesh APIs (Phase 2)</h2>
<p>Exposes <code>POST /api/mesh/broadcast</code> and <code>GET /api/mesh/subscribe</code>. Payloads are OHC-SIP compliant.</p>
<h2>4. AutoDream Pipelines (Phase 3)</h2>
<p>Sync local SQLite vector embeddings to Cloud pgvector instances in <code>autodream_memories</code>.</p>
</div>
