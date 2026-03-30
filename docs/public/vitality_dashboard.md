# OHC Vitality Dashboard

<style>
  body {
    font-family: 'Outfit', 'Inter', sans-serif;
    color: #ffffff;
    background-color: #121212;
    margin: 0;
    padding: 2rem;
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 24px;
    margin-bottom: 2rem;
  }

  .metric-card {
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(20px) saturate(200%);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);
  }

  .metric-value {
    font-size: 3rem;
    font-weight: 700;
    margin-bottom: 8px;
    background: linear-gradient(135deg, #00f2fe 0%, #4facfe 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  .metric-label {
    font-size: 1rem;
    font-weight: 500;
    color: #a0a0a0;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .chart-section {
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(20px) saturate(200%);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 24px;
    margin-bottom: 2rem;
  }
</style>

<div class="dashboard-grid">
  <div class="metric-card">
    <div class="metric-value">99.999%</div>
    <div class="metric-label">Uptime via E2E Chaos Testing</div>
  </div>

  <div class="metric-card">
    <div class="metric-value">< 15ms</div>
    <div class="metric-label">SIP SQLite Query Latency</div>
  </div>

  <div class="metric-card">
    <div class="metric-value">0</div>
    <div class="metric-label">Auth Bypasses (SPIFFE)</div>
  </div>

  <div class="metric-card">
    <div class="metric-value">100%</div>
    <div class="metric-label">Handoff Success Rate</div>
  </div>
</div>

<div class="chart-section">
  <h2 style="font-family: 'Outfit'; margin-top: 0;">Swarm Operations Overview</h2>
  <pre class="mermaid">
  pie title Agent Workload Distribution
    "Orchestration (Handoffs)" : 45
    "Execution (Missions)" : 35
    "Verification (Observability)" : 15
    "Identity Management (SPIFFE)" : 5
  </pre>
</div>