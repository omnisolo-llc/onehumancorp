<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

<h1>OHC Interactive API Playbook</h1>

<p><strong>Version:</strong> 1.0.0</p>
<p><strong>Target Audience:</strong> Orchestration Engineers & Human CEOs</p>

<h2>1. Introduction</h2>
<p>The One Human Corp (OHC) API is the central nervous system of the Agentic OS. It bridges the gap between Cloud-Native Kubernetes clusters and Standalone Desktop deployments via the <strong>Swarm Intelligence Protocol (OHC-SIP)</strong>.</p>

<h2>2. Authentication (Zero Secrets)</h2>
<p>All endpoints are secured via SPIFFE/SPIRE zero-trust principles or an OIDC JWT. We do not use static API keys. Ensure your client provides a valid JWT.</p>

<pre><code class="language-bash">
curl -X GET https://api.ohc.local/v1/agents/status \
  -H "Authorization: Bearer &lt;JWT_OR_SVID&gt;"
</code></pre>

<h2>3. Core Orchestration Endpoints</h2>

<h3>3.1 Hire Agent</h3>
<p><strong>Endpoint:</strong> <code>POST /api/agents/hire</code></p>
<p>Provisions and onboards a new agent into the Swarm.</p>

<h3>3.2 Teammate Mesh Broadcast</h3>
<p><strong>Endpoint:</strong> <code>POST /api/mesh/broadcast</code></p>
<p>Broadcasts an OHC-SIP compliant message to the Teammate Mesh. Requires <code>agent_id</code>, <code>action</code>, and <code>status</code> in the payload root.</p>

</div>
