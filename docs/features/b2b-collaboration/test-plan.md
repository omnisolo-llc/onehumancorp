<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Test Plan: B2B Agent Exchange

## 1. Objective
To verify the secure establishment of Trust Agreements, the routing of messages between independent organizational Hubs, and the enforcement of data boundary constraints during inter-org collaboration.

## 2. Scope
This test plan covers the `TrustAgreement` lifecycle, the `b2b-gateway` message tunneling over mTLS, role-based filtering, and memory segregation.

## 3. Test Environments
- **Federated Test Cluster:** Two isolated K8s namespaces (`acme-ns`, `globex-ns`), each running a full OHC stack including independent SPIRE servers and Postgres instances.

## 4. Test Cases

### TC-01: Trust Agreement Establishment
- **Description:** Verify that two organizations can establish mutual trust via OIDC JWKS exchange.
- **Action:** Create `TrustAgreement` resources in both `acme.corp` and `globex.com` pointing to each other's public `.well-known/jwks.json` endpoints.
- **Expected Result:** The `b2b-gateway` successfully fetches and caches the public keys. The `Status` of the agreements shifts to `ACTIVE`.

### TC-02: Authorized Message Tunneling
- **Description:** Verify that an allowed agent can send a message to a partner organization.
- **Pre-condition:** Trust agreement active. `Buyer Agent` allowed in `acme.corp`, `Sales Agent` allowed in `globex.com`.
- **Action:** A `Buyer Agent` in `acme.corp` publishes a message directed at a `Sales Agent` in `globex.com` via the Hub.
- **Expected Result:** The message is enveloped, sent over mTLS to the partner gateway, verified via SPIFFE SVID, and successfully delivered to the `Sales Agent`'s inbox in `globex.com`.

### TC-03: Unauthorized Role Rejection
- **Description:** Verify that agents not explicitly whitelisted cannot participate in the exchange.
- **Action:** An `Engineering Director` in `acme.corp` attempts to send a message to `globex.com`.
- **Expected Result:** The `b2b-gateway` instantly drops the message. An audit event `B2B_UNAUTHORIZED_ROLE` is logged.

### TC-04: Immediate Trust Revocation
- **Description:** Verify that "Severing the Bridge" immediately halts all communication.
- **Action:** Delete the `TrustAgreement` in `acme.corp` while an active negotiation is in progress. Attempt to send another message from `globex.com`.
- **Expected Result:** The message is rejected with a `401 Unauthorized` or equivalent mTLS handshake failure. The Inter-Org meeting room is forcibly closed on both sides.

### TC-05: Memory Segregation
- **Description:** Ensure cross-org messages do not pollute internal semantic memory.
- **Action:** Complete a successful negotiation in an Inter-Org Room. Trigger the Semantic Distillation Worker.
- **Expected Result:** Inspect the `swarm_memory_embeddings` table. Verify that no messages flagged with `CrossOrg: true` were embedded or stored in the long-term knowledge base.

</div>
