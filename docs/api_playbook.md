# OHC Interactive API Playbook

<style>
  .glass-container {
    background: rgba(255, 255, 255, 0.03);
    backdrop-filter: blur(20px) saturate(200%);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 24px;
    margin: 16px 0;
    font-family: 'Outfit', 'Inter', sans-serif;
  }
</style>

<div class="glass-container">
  <h2>Authentication & AuthZ</h2>
  <p>The OHC Hybrid Agentic OS uses SPIFFE/SPIRE for zero-trust native identity. Short-lived JWTs/SVIDs authenticate instances and requests.</p>
</div>

<div class="glass-container">
  <h2>Core Endpoints</h2>
  <ul>
    <li><strong>Orchestration:</strong> Manage swarm lifecycles.</li>
    <li><strong>Teammate Mesh:</strong> Enable sub-agent peer communication.</li>
    <li><strong>Agents:</strong> Deploy and invoke specific agent capabilities.</li>
  </ul>
</div>

<div class="glass-container">
  <h2>Standalone vs. Cloud Routing</h2>
  <p>Universal MCP Mesh extends local-to-cloud proxying, allowing a cloud agent to securely utilize an MCP tool running on the user's Standalone Desktop via reverse-tunnels.</p>
</div>

<div class="glass-container">
  <h2>Code Snippets & Testing</h2>
  <pre><code>
# Testing the API
curl -H "Authorization: Bearer $SVID" https://api.ohc.network/v1/orchestrate
  </code></pre>
</div>
