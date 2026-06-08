# Multi-tenant Isolation Architecture

This document describes the multi-tenant isolation architecture and policies deployed via Helm in Cloud mode.

## Overview

In Cloud mode, the OneHumanCorp platform operates as a multi-tenant SaaS application where a single cluster and logical database serve multiple tenants. To ensure strict isolation and zero-trust security between tenants, we enforce a combination of Kubernetes Network Policies, Resource Quotas, Horizontal/Vertical Pod Autoscaling (HPA/VPA) policies, and application-level Row-Level Security (RLS) in PostgreSQL.

## Application-Level Isolation

*   **Row-Level Security (RLS):** Every tenant workspace uses a `tenant_id` column on all tables. PostgreSQL enforces row-level isolation via `ENABLE ROW LEVEL SECURITY`.
*   **Tenant Context:** The Go API server injects the authenticated `tenant_id` into every database transaction.

## Network Isolation (Zero Trust)

We enforce strict network boundaries using Kubernetes `NetworkPolicy` objects.

*   **Default Deny:** A `default-deny` policy drops all ingress and egress traffic across the namespace by default, except for DNS resolution (`kube-system` on port 53).
*   **Explicit Allow-listing:**
    *   **Backend to Database:** The `ohc-backend` deployment is explicitly permitted to connect to the CNPG (PostgreSQL) cluster on port 5432.
    *   **Backend to Cache:** The `ohc-backend` deployment is explicitly permitted to connect to the Valkey (Redis) cluster on port 6379.
    *   **External Egress:** Egress to internal metadata services and private CIDR blocks (e.g., `169.254.169.254/32`) is strictly denied to prevent SSRF and metadata exfiltration attacks. Egress to the public internet is permitted only for specific agent endpoints.

## Resource Management (HPA/VPA)

To prevent the "noisy neighbor" problem in a multi-tenant environment, resource utilization is strictly managed:

*   **Horizontal Pod Autoscaler (HPA):** Scales the backend, core, chatwoot, and powersync deployments horizontally based on CPU and memory utilization targets (80%). This ensures responsive multi-tenant pod scaling during traffic spikes.
*   **Vertical Pod Autoscaler (VPA):** For workloads that require vertical scaling rather than horizontal, VPA ensures that containers receive the necessary CPU and memory limits within strict boundaries. Note that VPA and HPA are mutually exclusive on the same metrics by default in Kubernetes, so VPA is set to "Off" update mode when HPA is enabled.
*   **Resource Quotas:** Hard limits are enforced at the namespace level to ensure no single tenant or workload can exhaust cluster resources (CPU, memory, ephemeral storage, and total pods).

## Summary

The combination of RLS, explicit Network Policies, and strict Resource Quotas provides a defense-in-depth architecture that guarantees tenant isolation and maintains system stability under variable load.
