<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: [integrations] Hybrid Browser Control MCP

## Problem Statement
OHC agents currently use ad-hoc Playwright scripts or simple HTTP requests for web interaction. For complex, multi-step UI tasks (e.g., navigating a dashboard, interacting with a React app, resolving captchas), agents need a persistent, stateful browser context that can be shared across a swarm. There is no unified "Hybrid Browser Control MCP" that provides a persistent Playwright/Chromium daemon bridging both Cloud (headless) and Standalone (headful/visible) environments.

## Research Report
The most robust tool for programmatic browser control is **Playwright**. By running a persistent Playwright Server (using the CDP - Chrome DevTools Protocol), OHC can allow multiple agents to attach to the same browser session. In Standalone mode, this can be a headful browser visible to the user, while in Cloud mode, it is a headless container.

### Competitive Analysis
| Feature | Basic HTTP / Scrapy | Cloud Selenium Grid | OHC Hybrid Browser MCP |
| :--- | :--- | :--- | :--- |
| **JS Execution** | ❌ No | ✅ Yes | ✅ Yes |
| **Persistent Session** | ❌ No | ❌ Hard | ✅ Yes (Swarm Shared) |
| **Hybrid (Headful/Headless)** | ❌ No | ❌ No | ✅ Yes |

### Key Technologies
- **Playwright (Go/Node.js)**: For browser automation.
- **CDP (Chrome DevTools Protocol)**: For remote connection.
- **`BwrapRunner`**: To sandbox the Chromium process.

## Design Doc
**Architecture:**
- **Browser Control MCP**: Implements the MCP Tool interface.
- **Browser Manager**: A singleton service that manages a pool of Chromium instances.
- **Cloud Mode**: Runs headless Chromium in a dedicated K8s pod or sidecar.
- **Standalone Mode**: Launches a headful Chromium process on the user's desktop, allowing the user to "watch" the agent work.
- **Session Bridge**: A WebSocket proxy that routes MCP browser commands to the underlying Playwright instance.

**API Contracts:**
- `Navigate(url string) error`
- `Click(selector string) error`
- `GetScreenshot() ([]byte, error)`
- `InteractiveSession() (SessionURL, error)` (Returns a URL for the user to view the live session).

**Security:**
- Enforce strict `organization_id` isolation for browser profiles in Cloud mode.
- Use `BwrapRunner` to restrict Chromium's filesystem and network access.
- Implement "Human-in-the-loop" approval for sensitive actions (e.g., clicking a 'Delete' button).

## Implementation Prompt
"Implement the Hybrid Browser Control MCP tool in `srcs/server/lib/integrations/browser/`.
1. Create `browser.go` defining the `BrowserManager` MCP tool.
2. Integrate with Playwright (via `playwright-go`).
3. In Standalone mode, launch Chromium with `headless: false` so the user can see the agent's actions.
4. In Cloud mode, launch Chromium with `headless: true` inside a sandboxed environment using `BwrapRunner`.
5. Provide MCP tools: `browser_navigate`, `browser_click`, `browser_type`, and `browser_screenshot`.
6. Implement a session management system that allows multiple agents in a swarm to share the same browser profile.
7. Ensure 100% test coverage using Playwright's built-in testing capabilities and mocking the browser server.
8. Add an E2E test where an agent navigates to a local test page and takes a screenshot."

## Priority
P1

## Estimated Scope
Large

</div>
