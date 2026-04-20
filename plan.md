1. **Address Feedback:**
   - The code review flagged 3 major issues:
     1. **SSRF Vulnerability in `validateGuardrails`:** Need to block loopback addresses (`127.0.0.1`, `::1`), private networks (RFC 1918), and link-local addresses (e.g. `169.254.169.254`) in `srcs/server/lib/integrations/hybrid_discovery/discovery.go`.
     2. **Zero Secrets Violation in `ScoutProvider`:** Remove the `APIKey` requirement in `srcs/server/agents/provider.go`. `Authenticate` should just return `nil` without requiring secrets, adhering to SPIFFE/SPIRE identity.
     3. **E2E Test Standard Violation:** Instead of just a backend unit test, we need a full Playwright E2E test starting from the home page. Wait, the user prompt states "Each PR MUST contain one FULL feature covering one FULL Critical User Journey (CUJ), testable as a complete user-facing flow...". The draft issue is just for adding the backend integration and sub-agent definition. Is there a UI for this? Let me check `srcs/tests/e2e/`. I already fixed the E2E collision issue by adding `exclusive` tag.
     - However, the code review insists on a Playwright test. I'll need to create a basic E2E test in `srcs/tests/e2e/` (e.g. `cuj_scout_test.go`) that logs in, navigates to some integrations/tools page (if it exists) or triggers the scout agent.
     - Let's check `srcs/tests/e2e/e2e_agents_test.go` or `srcs/tests/e2e/agent_test.go` to see how agent tests are written.

2. **Fix `ScoutProvider` Zero Secrets:**
   - In `srcs/server/agents/provider.go`, update `Authenticate` for `ScoutProvider` to simply return `nil`.
   - Update `GetCredentials` to return empty `Credentials{}`.
   - Update `IsAuthenticated` to return `true`.

3. **Fix SSRF in `discovery.go`:**
   - Update `validateGuardrails` to parse the IP address. If it's a hostname, we might need to resolve it, or use a custom HTTP transport that blocks private IPs. For simplicity, we can use `net.ParseIP` and check `IsPrivate()`, `IsLoopback()`, `IsLinkLocalUnicast()`. However, Go's `http.Client` follows redirects, so a robust SSRF protection requires a custom `DialContext`. I'll implement a basic check on the URL Host and maybe a custom `http.Transport`.

4. **Implement E2E Playwright Test:**
   - Look at `srcs/tests/e2e/agent_test.go` to see how to start a test, login, and interact.
   - Create a test that logs in and maybe goes to the agent dashboard, hires the Scout agent, and asks it to integrate a tool. Since the UI might not be fully built for this (it's a backend feature mostly), I'll write a test that interacts with the UI to the extent possible.

Let's execute these fixes.
