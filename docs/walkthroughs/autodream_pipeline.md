<div markdown="1" style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# AutoDream Pipeline Walkthrough

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Overview</h2>
  <p>The <strong>AutoDream Pipeline</strong> is a background worker mechanism inside the KAIROS Orchestration layer. It is responsible for asynchronously consolidating ephemeral session contexts, pruning redundancies, and injecting semantic truth into the swarm's durable long-term memory via pgvector.</p>
</div>

## System Architecture

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>To prevent context window overflow and maintain coherent long-term reasoning, OHC implements a sophisticated memory consolidation process:</p>
  <ul>
    <li><strong>Extraction:</strong> Periodically polls recent session data and completed tasks.</li>
    <li><strong>Embedding:</strong> Synthesizes raw context using LLMs and generates 1536-dimensional embeddings.</li>
    <li><strong>Loading:</strong> Upserts data into the <code>autodream_memories</code> table using pgvector for precise nearest-neighbor semantic search.</li>
  </ul>
</div>

## Hybrid Architecture Fallback

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <h3>Cloud-Native Mode</h3>
  <p>When running in multi-tenant mode with PostgreSQL (<code>OHC_MULTITENANT=true</code>), the pipeline leverages <code>pgvector</code> for high-performance and exact nearest-neighbor matching.</p>

  <h3>Standalone Desktop Mode</h3>
  <p>For standalone, single-user setups utilizing local SQLite, the pipeline degrades gracefully. Embeddings are stored efficiently, and text extraction/recency-based fallback mechanisms ensure a seamless offline experience without heavy K8s or PostgreSQL dependencies.</p>
</div>

</div>
