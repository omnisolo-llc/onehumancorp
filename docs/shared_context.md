<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', sans-serif;">
  <h1 style="color: #ffffff;">Developer Insights</h1>
  <p style="color: #cccccc;">This document aggregates technical debt and architectural notes synthesized from the codebase.</p>

  <h2 style="color: #ffffff;">Security and Maintenance</h2>
  <ul style="color: #cccccc;">
    <li><strong>Ironclaw Security:</strong> The <code>ironclaw</code> tool contains hardcoded insecure comments and passwords (<code>TODO: fix security</code>, <code>password = "secret"</code>) within <code>main_test.go</code> and <code>main.go</code>. These should be addressed to maintain a secure codebase.</li>
    <li><strong>Dashboard Models:</strong> The dashboard model in <code>app/lib/models/dashboard.dart</code> contains duplicate fallback parsing logic (<code>totalCostUSD</code> vs <code>total_cost_usd</code>). Ensure JSON API responses are standardized to snake_case or camelCase to avoid mapping issues.</li>
  </ul>
</div>
