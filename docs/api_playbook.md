# OHC Interactive API Playbook

<style>
/* OHC Premium Glassmorphism Aesthetic Mandate */
body {
    font-family: 'Inter', sans-serif;
    background-color: #0d0d0d;
    color: #f1f1f1;
}
h1, h2, h3, h4 {
    font-family: 'Outfit', sans-serif;
    color: #ffffff;
}
.glass-panel {
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(20px) saturate(200%);
    -webkit-backdrop-filter: blur(20px) saturate(200%);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 24px;
    margin-bottom: 24px;
    box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);
}
code {
    background: rgba(255, 255, 255, 0.1);
    padding: 2px 4px;
    border-radius: 4px;
    font-family: monospace;
}
pre {
    background: rgba(0, 0, 0, 0.4);
    padding: 16px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.05);
    overflow-x: auto;
}
</style>

<div class="glass-panel">
    <h2>1. Authentication & AuthZ</h2>
    <p>All API endpoints require secure authentication via Bearer Tokens. The OHC Gateway handles tenant scoping and Plan Enforcement natively before hitting microservices.</p>
    <pre><code>Authorization: Bearer ohc_sec_...</code></pre>
</div>

<div class="glass-panel">
    <h2>2. Core Endpoints</h2>
    <ul>
        <li><strong>Orchestration:</strong> <code>POST /api/v1/orchestration/jobs</code> - Dispatch an event-driven agent task.</li>
        <li><strong>Teammate Mesh:</strong> <code>GET /api/v1/mesh/nodes</code> - Retrieve active cluster nodes.</li>
        <li><strong>Agents:</strong> <code>GET /api/v1/agents/departments</code> - List AI departments and their configurations.</li>
    </ul>
</div>

<div class="glass-panel">
    <h2>3. Standalone vs. Cloud Routing</h2>
    <p>OHC supports Hybrid Routing. Depending on your configuration, endpoints automatically route locally (Standalone) via the Sync Daemon, or hit the global distributed layer (Cloud Mode). The API contract remains identical.</p>
</div>

<div class="glass-panel">
    <h2>4. Testing Instructions & Snippets</h2>
    <p>Use the following cURL snippet to test your local Standalone connection:</p>
    <pre><code>curl -X GET "http://localhost:8080/api/v1/mesh/nodes" \
     -H "Authorization: Bearer YOUR_TOKEN"</code></pre>
</div>
