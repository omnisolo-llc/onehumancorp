<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', sans-serif;">
  <h1 style="color: #ffffff;">Settings Module</h1>
  <p style="color: #cccccc;">This package manages platform settings and configuration models for the Go backend.</p>

  <h2 style="color: #ffffff;">Architecture Walkthrough</h2>
  <div class="mermaid">
  graph TD;
      Settings[Settings Module] --> DB[(Database)];
      Settings --> UI[Dashboard Frontend];
  </div>
</div>
