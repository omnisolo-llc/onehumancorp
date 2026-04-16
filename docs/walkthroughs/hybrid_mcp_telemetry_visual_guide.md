# Hybrid MCP Telemetry Visual Guide

This guide walks through configuring, observing, and extending the Hybrid MCP Integration components (e.g., Telemetry-MCP Bridge, Standalone Sync Queue) via the OHC UI.

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); margin-bottom: 20px;">

## 1. Architectural Flow

1. **Local Collection:** The Standalone Sync Queue caches OTel metrics in local SQLite.
2. **Bridge Activation:** The `telemetry-mcp-bridge` polls the queue and formats data to MCP tools.
3. **KAIROS Ingestion:** The Cloud KAIROS Orchestrator invokes the MCP tool, syncing data to OHC-SIP (PostgreSQL/VectorDB).

## 2. Visual Wireframe: Telemetry Operations Dashboard

The following wireframe represents the OHC Premium UI for the Telemetry-MCP Bridge in Grafana/Dashboards.

<div style="background: linear-gradient(135deg, rgba(20,20,30,0.8) 0%, rgba(10,10,15,0.9) 100%); padding: 30px; border-radius: 16px; border: 1px solid rgba(255,255,255,0.05); color: #E0E0E0; font-family: 'Inter', sans-serif;">

  <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 15px; margin-bottom: 20px;">
    <h3 style="margin: 0; font-family: 'Outfit', sans-serif; font-weight: 500; font-size: 1.5em;">🌐 Hybrid MCP Telemetry Bridge</h3>
    <span style="background: rgba(46, 204, 113, 0.1); color: #2ecc71; padding: 5px 12px; border-radius: 20px; font-size: 0.8em; font-weight: 600;">ACTIVE (SYNCING)</span>
  </div>

  <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-bottom: 20px;">
    <!-- Stat Box 1 -->
    <div style="background: rgba(255,255,255,0.02); padding: 20px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.03); backdrop-filter: blur(10px);">
      <p style="margin: 0 0 10px 0; font-size: 0.85em; color: #888; text-transform: uppercase; letter-spacing: 1px;">Queue Size</p>
      <div style="font-size: 2.2em; font-weight: 300; font-family: 'Outfit', sans-serif; color: #fff;">142<span style="font-size: 0.4em; color: #888; margin-left: 5px;">events</span></div>
    </div>

    <!-- Stat Box 2 -->
    <div style="background: rgba(255,255,255,0.02); padding: 20px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.03); backdrop-filter: blur(10px);">
      <p style="margin: 0 0 10px 0; font-size: 0.85em; color: #888; text-transform: uppercase; letter-spacing: 1px;">Sync Latency</p>
      <div style="font-size: 2.2em; font-weight: 300; font-family: 'Outfit', sans-serif; color: #fff;">45<span style="font-size: 0.4em; color: #888; margin-left: 5px;">ms</span></div>
    </div>
  </div>

  <!-- Activity Log -->
  <div style="background: rgba(0,0,0,0.3); padding: 15px; border-radius: 8px; font-family: monospace; font-size: 0.9em; color: #aaa;">
    <div style="margin-bottom: 5px;">[12:04:11] <span style="color: #4db8ff;">INFO</span> Bridge connected to standalone-node-1234</div>
    <div style="margin-bottom: 5px;">[12:04:15] <span style="color: #4db8ff;">INFO</span> Synced batch (count=50) to Central OHC DB</div>
    <div style="margin-bottom: 5px;">[12:04:18] <span style="color: #4db8ff;">INFO</span> Handshake complete with KAIROS Orchestrator</div>
  </div>
</div>

</div>
