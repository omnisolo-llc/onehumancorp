# Test Plan: Modular Capability Plugin Mesh & Next-Gen Aesthetics

**Author(s):** Antigravity, Principal Product Architect & Visionary (L7)
**Status:** Approved
**Last Updated:** 2026-03-28

## 1. Scope
The scope of this test plan encompasses verifying the structural integrity of the Capability Plugin Mesh architecture and the successful implementation of the Next-Generation OHC Design System across the UI.

## 2. Methodology
All backend verifications will rely exclusively on Bazel hermetic testing via `bazelisk test //...`. Frontend tests mandate the use of the `browser` tool (Playwright) to verify DOM elements and visual regressions for the "Premium Feel" aesthetic tokens.

## 3. Backend (Capability Plugin Mesh)
### 3.1 Unit Testing (`srcs/orchestration:orchestration_test`)
*   **Test Goal:** Verify `RegisterCapability()` dynamically inserts a `CapabilityManifest` into the `capability_plugins` and `swarm_memory_embeddings` mock tables.
*   **Action:** Verify that duplicate endpoints return the appropriate gRPC status code.
*   **Metric:** Maintain >95% statement coverage for the dynamic registration flow.

### 3.2 Integration Testing (MCP Gateway)
*   **Test Goal:** Verify an agent querying the Gateway for "data analysis tools" returns the dynamically registered capability schema without restart.
*   **Action:** Deploy a mock K8s service, register the capability, and execute a tool discovery query.
*   **Metric:** Zero dropped queries or stale cache hits during discovery.

## 4. Frontend (Next-Gen Aesthetics)
### 4.1 Playwright Visual Regression (`browser` tool)
*   **Test Goal:** Verify the Next.js/Flutter dashboard correctly applies the OHC design tokens.
*   **Action:** Navigate to the core dashboard and inspect the DOM for specific CSS tokens:
    *   `backdrop-filter: blur(15px) saturate(180%)`
    *   `background: rgba(255, 255, 255, 0.05)`
    *   `border: 1px solid rgba(255, 255, 255, 0.1)`
    *   `font-family: 'Outfit', 'Inter', sans-serif`
*   **Metric:** Zero pixel differences in visual regression checks against approved mockups. No legacy hardcoded background colors. All dashboard panels must utilize the glass-like tokens.

### 4.2 Playwright Interaction Tests
*   **Test Goal:** Verify capability binding transitions are smooth and handle asynchronous data population.
*   **Action:** Simulate a capability registration event via WebSocket and verify the UI updates the component without a full page refresh.

## 5. Security & Identity
*   **Test Goal:** Verify SPIFFE SVIDs are respected during dynamic discovery.
*   **Action:** Attempt to query the capability database using an unauthenticated or improperly scoped token.
*   **Metric:** The request must be rejected with HTTP 401/403 (Zero Trust Network Policies).
