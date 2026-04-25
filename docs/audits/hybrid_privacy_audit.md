# Hybrid Privacy Audit

## 1. Executive Summary
This document outlines the privacy design constraints and implementation standards for the OHC Hybrid Agentic OS, contrasting data handling in multi-tenant Cloud mode versus Standalone Desktop mode.

## 2. Multi-Tenant Cloud Mode (Privacy-by-Design)
In Cloud mode, the system handles data for multiple independent tenants. To ensure data privacy and prevent PII leakage:

- **Row-Level Security (RLS)**: Mandatory `tenant_id` column on all persistent storage. RLS policies enforce tenant isolation at the database level.
- **Structured Logging Redaction**: PII (e.g., emails, phone numbers, API keys) must be explicitly scrubbed or hashed before being written to centralized logging systems.
- **Telemetry Scrubbing**: Traces and metrics exported to centralized observability platforms must not contain unhashed PII.

## 3. Standalone Desktop Mode (User Data Sovereignty)
In Standalone mode, the system runs locally on the user's hardware. The core principle is **Zero Exfiltration**:

- **Local Storage Only**: Data, including vector embeddings and agent memories, is stored in local SQLite databases.
- **Telemetry Disabled**: By default, no metrics, traces, or logs are transmitted to central servers unless explicitly opted-in by the user for diagnostic purposes.
- **Local Secrets**: API keys and external service credentials reside solely on the local filesystem and are not synchronized to the cloud.

## 4. Policy-as-Code Implementation
To enforce these privacy constraints, we implement automated guardrails within the `src/server/telemetry` module:

### 4.1 PII Scrubbing Tests
- `TestStandaloneNoTelemetry`: Verifies that telemetry export mechanisms are inert when operating in standalone mode.
- `TestCloudLogRedaction`: Simulates logging of sensitive data in cloud mode and asserts that the output is properly redacted (e.g., `email=***@***.***`).

### 4.2 AST Guardrails
To prevent accidental hardcoding or unsanitized environmental access in logging, we use Go's `go/ast` to scan the telemetry package for direct usage of `os.Getenv`, ensuring configuration values are routed through the secure configuration manager.

## 5. Conclusion
These dual-mode constraints ensure OHC meets rigorous enterprise multi-tenant privacy standards while simultaneously providing an uncompromised "air-gapped" experience for standalone desktop users.
