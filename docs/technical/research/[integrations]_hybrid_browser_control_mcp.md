<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 2rem; font-family: 'Outfit', 'Inter', sans-serif;">

# Title: [integrations] Hybrid Browser Control MCP

## Problem Statement
Agentic UI automation in the current market is plagued by high latency and state fragmentation. Browsers are typically launched and destroyed per task, leading to "cold starts" and the loss of session context (cookies, local storage). OHC requires a "Universal Agent Harness" capable of sub-second UI orchestration. This necessitates a Hybrid Browser Control MCP that can interact with persistent browser daemons in both local (Standalone) and remote (Cloud-Native) environments.

## Research Report
Research into high-performance agent harnesses (e.g., `gstack`) reveals that maintaining a long-lived browser daemon significantly reduces execution latency.
- **Competitors**: Most frameworks use ephemeral Playwright/Puppeteer sessions.
- **OHC Advantage**: By implementing a Hybrid Browser Control MCP, we allow agents to connect to a "Navigator" layer—a persistent Playwright/Chromium daemon. This ensures that tabs, sessions, and authenticated states persist across multiple agent turns, enabling complex, multi-step UI reasoning with sub-second response times.
- **Reference**: `docs/research/[harness]_universal_agent_harness_architecture.md`.

## Design Doc
**Architecture:**
- Create a new package `srcs/server/lib/integrations/browser_control/`.
- Introduce a `BrowserControlManager` implementing the MCP Tool interface.
- Dynamically route to the appropriate browser endpoint:
  - **Standalone Mode**: Connect to a local persistent Playwright daemon (running as a sidecar or background service).
  - **Cloud Mode**: Route to a multi-tenant browser farm (e.g., `browserless.io` or internal K8s Chromium pods) using SPIFFE-secured headers.

**API Contracts:**
- `ExecuteAction(ctx context.Context, action string, params map[string]interface{}) (ActionResult, error)`
- `GetBrowserState(ctx context.Context) (BrowserState, error)`
- `TakeScreenshot(ctx context.Context) ([]byte, error)`

**Security:**
- Enforce strict organization-based session isolation in Cloud mode.
- Use `RedactInterfacePII` on HTML/DOM content before returning it to the agent to prevent leakage of sensitive user data.

## Implementation Prompt
"Implement the Hybrid Browser Control MCP tool in `srcs/server/lib/integrations/browser_control/`.
1. Create `browser_control.go` defining the `BrowserControlManager` and its MCP capabilities.
2. Implement the client logic to communicate with a persistent Playwright/Chromium daemon via CDO (Chrome DevTools Protocol) or a custom HTTP wrapper.
3. For Standalone mode, assume the daemon is available at a configurable local port.
4. For Cloud mode, ensure the request includes `X-Organization-ID` and `Authorization` headers (SVID-based).
5. Implement a `RedactDOMPII` function to strip sensitive fields (emails, passwords, credit cards) from DOM snapshots before returning them.
6. Create tests in `browser_control_test.go` with a mocked browser daemon.
7. Update `BUILD.bazel` to include Playwright/Go dependencies."

## Priority
P0

## Estimated Scope
Large
</div>
