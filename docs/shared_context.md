# Developer Insights & Technical Context

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="margin-top: 0; color: #fff;">Overview</h2>
  <p style="color: #ccc;">
    This document serves as the central intelligence repository for known technical debt, temporary workarounds, and implicit architectural contexts discovered within the OHC codebase. It bridges the gap between active development state and manual specifications.
  </p>
</div>

<br>

<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255, 255, 255, 0.08); padding: 20px; border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">
  <h2 style="margin-top: 0; color: #fff;">Extracted Technical Debt</h2>

  <ul style="color: #ccc;">
    <li>
      <strong>Ironclaw Security Scans:</strong> The current `ironclaw` security scanner has unresolved or hardcoded mock violations representing insecure development practices (`TODO: fix security`). These highlight the need for a comprehensive audit of mock injection practices within testing suites. Refer to `srcs/cmd/ironclaw/main_test.go` and `main.go`.
    </li>
    <li>
      <strong>Dashboard Model Data Parsing:</strong> The Flutter dashboard model (`srcs/app/lib/models/dashboard.dart`) uses redundant fallback keys (`totalCostUSD` vs `total_cost_usd` and `costUSD` vs `cost_usd`) for backward compatibility during JSON deserialization. The data layer should eventually be standardized to exclusively utilize `snake_case` from the backend API.
    </li>
    <li>
      <strong>Missing Handler Security Mocks:</strong> Some integration tests (`srcs/dashboard/server_missing_test.go`, `server_test.go`) use hardcoded malicious or dummy Spiffe IDs/role assertions (e.g., `spiffe://evil-hacker.com/agent/1`, `Bad Agent`). Test suites should migrate to dynamically provisioning localized SPIFFE identities.
    </li>
    <li>
      <strong>Settings UI Refresh:</strong> The settings screen (`srcs/app/lib/screens/settings_screen.dart`) employs a "simple refresh hack" to force-reload state. Long term, this should transition to a reactive Riverpod or Checkpointer stream provider.
    </li>
  </ul>
</div>
