<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Title: [integrations] Hybrid Secure-Credential-Bridge (SCB) MCP

## Problem Statement
In the OHC Hybrid Architecture, Cloud-Native agents often need to perform actions that require sensitive user credentials (e.g., signing a Git commit with GPG, authenticating an AWS CLI command, or establishing an SSH connection). Currently, this requires either hardcoding secrets in the Cloud (unsafe) or manual user intervention (low autonomy). There is no secure, zero-trust mechanism that allows Cloud agents to request cryptographic signatures or temporary authentication tokens from the user's Standalone Desktop without the private keys ever leaving the local hardware.

## Research Report
Market analysis of agentic frameworks (Claude Code, Replit Agent, OpenClaw) reveals a common failure mode: they either operate purely locally (limiting swarm scale) or require users to upload sensitive `.pem` or `.env` files to the cloud.
- **Competitors:**
  - **Claude Code:** Operates locally, so it has access to `ssh-agent`, but cannot easily scale to a K8s-based swarm.
  - **OpenClaw:** Relies on cloud-stored secrets, increasing the attack surface.
- **OHC Unfair Advantage:** By leveraging OHC-HA and our existing reverse-tunnel infrastructure, we can implement a "Signing Proxy." The Cloud agent dispatches a `SignRequest` via the MCP Switchboard; the Standalone Desktop receives it, prompts the user (or uses a pre-approved policy), performs the signature locally using `ssh-agent` or `gpg-agent`, and returns only the signature. This preserves **Absolute Autonomy** while maintaining **Zero Trust** security.

## Design Doc
**Architecture:**
- **Cloud Side:** A new MCP Tool `credential_bridge.sign` and `credential_bridge.get_token` registered in the Switchboard.
- **Transport:** Utilizes the existing gRPC/WebSocket reverse-tunnel (see `[backend]_mcp_local_to_cloud_proxy.md`).
- **Local Side:** A Go-based provider in the Standalone Desktop that interfaces with local credential daemons (`ssh-agent`, `gpg-agent`, or the Windows/macOS Keyring).

**API Contracts:**
```protobuf
service CredentialBridge {
  rpc SignPayload(SignRequest) returns (SignResponse);
  rpc GetTemporaryToken(TokenRequest) returns (TokenResponse);
}

message SignRequest {
  string key_id = 1; // e.g., SSH fingerprint or GPG key ID
  bytes payload = 2;
  string purpose = 3; // For HITL gating (e.g., "Sign git commit")
}
```

**Security:**
- **Zero-Exfiltration:** Private keys NEVER leave the Standalone Desktop.
- **Approval Gating:** The Standalone UI MUST show a "Glassmorphism" popup for any signing request unless a specific TTL-based policy is active.
- **Identity:** All requests are signed with the Standalone instance's SPIFFE SVID.

## Implementation Prompt
"Implement the Hybrid Secure-Credential-Bridge MCP in `srcs/server/lib/integrations/credential_bridge/`.
1. Create `bridge.go` defining the `BridgeManager` and its MCP capabilities (`SignPayload`, `GetToken`).
2. Implement the local driver that communicates with `ssh-agent` via the `SSH_AUTH_SOCK` and `gpg` via command execution.
3. Integrate with the existing `ProxyServer` to route requests from Cloud agents to the local `BridgeManager`.
4. Create a Flutter-based approval dialog in `srcs/app/lib/widgets/credential_approval_dialog.dart` that utilizes the OHC Premium Aesthetic (blur: 20px, border: 1px solid rgba(255,255,255,0.1)).
5. Ensure 100% Go test coverage in `bridge_test.go` using mocked ssh-agent sockets.
6. Add an E2E test where a Cloud agent successfully requests a mock signature from a Standalone instance."

## Priority
P1

## Estimated Scope
Large

</div>
