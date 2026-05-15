# OHC Container Hardening & Tenant Isolation

## Network Policies
Strict Kubernetes NetworkPolicies have been implemented across all services to enforce a Default Deny posture and strict microsegmentation.
- **Backend**: Can only receive ingress from `ohc-core` and its own namespace. Egress restricted to explicit services (Redis, PG, DNS).
- **OhcCore**: Can only receive ingress from the backend. Egress restricted to standard DNS.
- **Chatwoot**: Isolated ingress from the backend only. Egress restricted to DNS.
- **PowerSync**: Isolated ingress from backend only.
- **Redis & PostgreSQL (CNPG)**: Isolated to only allow connections from the backend.

## Resource Quotas
A namespace-level `ResourceQuota` ensures no single tenant deployment can overrun the cluster:
- **CPU**: 60 max cores
- **Memory**: 120Gi max
- **Pods**: 200 max pods
- **Ephemeral Storage**: 150Gi max

## Local Standalone Wrapper Security
The Desktop standalone runner scripts have been hardened:
- `OHC_RUNTIME_DIR` is now scoped to an absolute user-path (`${HOME}/.ohc/runtime`) to prevent malicious symlink traversals or relative path vulnerabilities.
- Malloc trim threshold enabled to forcefully return unused memory back to the host system.
