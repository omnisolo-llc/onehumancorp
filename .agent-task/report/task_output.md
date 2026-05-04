<div style="backdrop-filter: blur(15px) saturate(200%); background: rgba(255, 255, 255, 0.1); border: 1px solid rgba(255, 255, 255, 0.2); border-radius: 12px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="color: #333;">Triage Report & Debt Cleanup</h2>

  <p><strong>Category:</strong> <code>cleanup</code></p>

  <p><strong>Signal Hygiene:</strong> Replaced all unstructured <code>eprintln!</code> error logging with structured <code>tracing::error!</code> and <code>tracing::warn!</code> macros across the `src/server` module to properly filter and route system signals.</p>

  <p><strong>Health Guardianship:</strong> Implemented hybrid-mode and local-to-cloud mission sync probes in the `check_health` API endpoint (`src/server/hub.rs`) to track <code>hybrid_mode_switching_healthy</code> and <code>local_to_cloud_mission_sync_healthy</code>.</p>

  <p><strong>Backlog Management:</strong> Resolved the infinite loop bug in `prune_stale_missions` (`src/server/sip.rs`) where STUCK missions were improperly requeued to PENDING continuously. The logic now correctly prioritizes and sanitizes STUCK missions to BURSTING.</p>
</div>
