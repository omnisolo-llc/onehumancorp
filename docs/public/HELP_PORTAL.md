<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 24px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #ffffff; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# OHC Help Portal & Visual Walkthroughs

**Version:** 1.0.0
**Target Audience:** All Users (Cloud-Native, Standalone, & Headless)

## 1. Welcome to the Swarm
Welcome to the One Human Corp (OHC) Help Portal. The Agentic OS is designed for zero-friction orchestration, whether you're commanding the swarm from a local desktop or scaling across Kubernetes clusters. This portal provides visual walkthroughs and technical guidance for mastering OHC.

## 2. Navigating the Hybrid Interface
The OHC interface elegantly adapts to your current mode:

*   **Cloud Mode:** Access via your organization's custom subdomain. The UI focuses on multi-tenant observability and high-concurrency swarm management.
*   **Standalone Mode:** Launched via the desktop shell. The interface streamlines local resources, directly connecting to your local SQLite-backed SIPDB.

### 2.1 The Glassmorphism Dashboard
Every element of the OHC UI strictly adheres to our Premium Aesthetic:
*   *Blur:* 20px
*   *Typography:* Outfit / Inter
*   *Atmosphere:* High-saturation, low-opacity glass layers over dynamic backgrounds.

## 3. Visual Walkthroughs

### 3.1 Delegating Your First Task
1.  **Open the Orchestration Hub:** Navigate to the "Hub" tab in the left sidebar.
2.  **Define the Mission:** Enter your objective in the "New Mission" input field. Be specific!
3.  **Assign Roles:** Select the appropriate agent roles (e.g., `swe`, `scribe`, `sales_rep`). OHC will automatically provision the agents based on available quota.
4.  **Execute:** Click "Launch Swarm".

### 3.2 Monitoring Swarm Vitality
The Vitality Dashboard provides real-time telemetry on your active agents.
*   **Active Agents:** Count of currently processing sub-agents.
*   **Mesh Traffic:** Real-time visualization of Teammate Mesh communication (via WebSockets).
*   **Resource Utilization:** Monitor VRAM and DB read/writes to ensure optimal performance.

## 4. Troubleshooting common issues

### 4.1 "Agent Not Responding"
*   **Cloud Mode:** Verify your SPIFFE/SPIRE identity token hasn't expired. Check the `ohc_swarm_tasks_completed` metric in Grafana.
*   **Standalone Mode:** Ensure the local backend process is running (`bazelisk run //:desktop`). Check the local console for `database is locked` errors, which may occur during intense SQLite concurrency.

### 4.2 UI Not Rendering Correctly
Ensure your browser or local client supports backdrop filters. The Glassmorphism UI requires `backdrop-filter: blur(20px)` support. Update your client if issues persist.

## 5. Getting Further Support
If you need assistance beyond these walkthroughs, our support agents are always available via the in-app Chatwoot integration (when deployed in Cloud Mode).

</div>
