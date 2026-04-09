<div style="font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem;">

# OHC Interactive Setup Walkthrough

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37);">
  <h2 style="margin-top: 0;">Welcome to One Human Corp (OHC)</h2>
  <p>This guide will walk you through setting up the OHC Hybrid Agentic OS, catering to both Standalone Desktop developers and Multi-Tenant Cloud orchestrators.</p>
</div>

## 1. Prerequisites

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>Ensure you have the following installed before proceeding:</p>
  <ul>
    <li><code>bazelisk</code> for managing the build process.</li>
    <li><code>go 1.22+</code> for backend services.</li>
    <li><code>docker</code> and <code>docker-compose</code> (Cloud mode only).</li>
  </ul>
</div>

## 2. Choosing Your Deployment Mode

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <h3>A. Standalone Mode (Local Desktop)</h3>
  <p>Ideal for low resource consumption and privacy. This mode uses SQLite instead of PostgreSQL.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code># Start the standalone backend
bazelisk run //srcs/server:standalone

# Or run the tests specifically
bazelisk test //...
</code></pre>
</div>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <h3>B. Cloud-Native Mode (Multi-Tenant)</h3>
  <p>Optimized for horizontal scaling using Kubernetes, PostgreSQL, and Redis.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code># Spin up cloud dependencies
docker-compose up -d postgres redis

# Run the cloud server
bazelisk run //srcs/server:cloud
</code></pre>
</div>

## 3. Verifying the Setup

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem;">
  <p>After starting the services, verify the API health endpoint to ensure everything is operating smoothly.</p>
  <pre style="background: rgba(0,0,0,0.5); padding: 1rem; border-radius: 8px;"><code>curl http://localhost:8080/api/health</code></pre>
  <p>You should see a 200 OK response indicating successful deployment.</p>
</div>

</div>
