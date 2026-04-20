<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: [integrations] Hybrid Secure-Credential-Bridge (SCB) MCP

## Problem Statement
In OHC's Hybrid Architecture, agents frequently operate in Cloud-native environments while requiring access to credentials and signing capabilities that are strictly tethered to the user's local machine (e.g., SSH private keys, GPG keys, local AWS/GCP profiles). Currently, there is no secure way for a cloud-hosted agent to request a signature or authentication token from the local desktop without either:
1.  **Exposing Private Keys**: Transferring sensitive keys to the cloud, which violates security best practices and the OHC "Zero Secrets" mandate.
2.  **Manual Intervention**: Requiring the user to manually copy-paste tokens, breaking the "Absolute Autonomy" value.

## Research Report
Market analysis of existing agentic tools (Claude Code, Replit Agent) shows a heavy reliance on local environment variables or manual auth prompts. OHC can achieve an "Unfair Advantage" by implementing a Secure-Credential-Bridge (SCB) that allows cloud agents to send signing requests to a local daemon.

### Competitive Analysis
| Feature | Manual Copy-Paste | Static Cloud Secrets | OHC Secure-Credential-Bridge |
| :--- | :--- | :--- | :--- |
| **Security** | Medium (Manual) | Low (Static Keys) | ✅ High (Keys never leave local) |
| **Autonomy** | ❌ Low | ✅ High | ✅ High |
| **Hybrid Compatibility** | ❌ No | ✅ Yes | ✅ Yes |

### Key Technologies
- **MCP Tool Protocol**: For request/response between agent and bridge.
- **SPIFFE/SPIRE**: For authenticating the cloud agent to the local bridge.
- **Local Signing Daemon**: A lightweight Go process running on the Standalone Desktop.

## Design Doc
**Architecture:**
- **SCB Proxy (MCP Tool)**: Runs on the Cloud/Server side. It exposes tools like `sign_ssh_payload`, `get_aws_token`, etc.
- **SCB Local Daemon**: Runs on the user's local machine (Standalone Desktop). It listens for authenticated requests from the Cloud SCB Proxy.
- **Secure Channel**: Communication is secured via mTLS using SPIFFE IDs or a persistent WebSocket over HTTPS with JWE (JSON Web Encryption) payloads.

**API Contracts:**
- `RequestSignature(key_type string, payload []byte) (signature []byte, err error)`
- `GetTemporaryCredential(service string, params map[string]string) (credential string, err error)`

**User Experience:**
- The first time a cloud agent requests a signature, the local Standalone Desktop UI shows a Glassmorphism-styled "Approval Request" (20px blur, Outfit font).
- Users can "Always Allow" specific agents or "Allow Once".

## Implementation Prompt
"Implement the Hybrid Secure-Credential-Bridge (SCB) MCP tool in `srcs/server/lib/integrations/scb/`.
1. Create `bridge.go` defining the `SCBProxy` MCP tool.
2. Implement the `SignPayload` and `GetCredential` tools.
3. In Cloud mode, these tools should dial the user's registered Standalone Desktop endpoint (authenticated via SPIRE).
4. In Standalone mode, these tools should directly interface with local system-keychains or agent-daemons (e.g., `ssh-agent`).
5. Create a `daemon/` subdirectory with a lightweight Go binary that can be bundled with the Standalone Desktop to handle incoming signing requests.
6. Ensure 100% test coverage with mocks for the local signing backend.
7. Add an E2E test where a simulated cloud agent requests an SSH signature and receives it from a mocked local daemon."

## Priority
P1

## Estimated Scope
Large

</div>
