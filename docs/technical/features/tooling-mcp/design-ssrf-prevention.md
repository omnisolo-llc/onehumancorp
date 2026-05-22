<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Doc: Integrations Registry SSRF Prevention

**Author(s):** AI Agent
**Status:** Approved
**Last Updated:** 2026-03-19

## 1. Overview
The `src/server/integrations/registry.rs` package handles connections to external services via user-provided URLs. This design document outlines the strategy for preventing Server-Side Request Forgery (SSRF) vulnerabilities in the Integrations Registry.

## 2. Goals & Non-Goals
### 2.1 Goals
- Prevent AI agents or users from forcing the server to make requests to internal or restricted network resources.
- Enforce strict URL validation during the integration connection process (`Connect()`).
- Block access to loopback, private, unspecified, and link-local IP addresses.
- Implement a "fail-closed" mechanism for DNS resolution errors to mitigate DNS rebinding attacks.

### 2.2 Non-Goals
- Implementing a full egress proxy or altering network-level firewall rules.

## 3. Detailed Design

### 3.1 URL Validation Function
A new `validate_url(input: &str) -> Result<(), Error>` function will be added to `registry.rs`. This function will perform the following checks:
1. Parse the URL using the Rust `url` crate.
2. Extract the hostname.
3. Perform DNS resolution using the async runtime resolver or a dedicated HTTP egress guard.
4. If DNS resolution fails, the function will return an error (fail-closed).
5. Iterate through the resolved IP addresses and reject the URL if any IP matches:
   - Loopback (`IsLoopback()`)
   - Private (`IsPrivate()`)
   - Unspecified (`IsUnspecified()`)
   - Link-Local Unicast (`IsLinkLocalUnicast()`)
   - Link-Local Multicast (`IsLinkLocalMulticast()`)

### 3.2 Integration with Connect()
The `Connect()` method in `Registry` will be updated to call `validateURL()` on the provided `baseURL` (if present) and the `WebhookURL` within `IntegrationCredentials` (if provided). If validation fails, `Connect()` will return the error and abort the connection process.

### 3.3 Test-Connection coverage
`TestConnection()` should similarly validate URLs before attempting any HTTP connection.

## 4. Edge Cases
- **DNS Rebinding:** Addressed by the fail-closed DNS check, though a full defense requires resolving and connecting via IP simultaneously or using a dedicated proxy. This implementation provides an application-level defense.
- **Malformed URLs:** Handled by standard `url.ParseRequestURI` checks.
- **Missing Scheme:** Ensure the URL parser enforces a valid scheme (http/https).

## 5. Implementation Details
- **Location:** `src/server/integrations/registry.rs` and adjacent Rust tests.
- **Language:** Rust.
- **Dependencies:** Rust URL parsing and DNS resolution helpers already used by the server stack.

</div>
