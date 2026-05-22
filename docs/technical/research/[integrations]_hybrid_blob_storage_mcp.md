# Title: Integration Blueprint: Hybrid Blob Storage MCP

## Problem Statement
While OHC agents have robust data synchronization for structured SQLite-to-Postgres data, there is a lack of a unified interface for handling unstructured blob storage across both environments. Local agents need a way to seamlessly read and write large files, images, and binary artifacts to local file systems (when running standalone) and effortlessly synchronize or route these blobs to cloud-based object storage (like AWS S3 or GCP Cloud Storage) when operating in the multi-tenant Postgres cloud environment. This capability is missing from the current Model Context Protocol (MCP) toolset.

## Research Report
Current blob storage solutions in MCP are highly fragmented. Local tools like Replit's file system agents focus purely on the local disk, whereas cloud tools focus heavily on S3/GCP APIs.
- **Competitors:** Existing MCP file servers are usually strictly local or strictly cloud. OHC's value lies in seamless hybrid transitions.
- **Proposed Solution:** Implement an application-level Hybrid Blob Storage MCP tool that provides a unified `ReadBlob`/`WriteBlob` API. The tool will intelligently inspect the running environment (Standalone vs Cloud-native) and persist blobs either to a temporary local file system directory created via `os.MkdirAll` (when running standalone) or an S3-compatible backend, ensuring agents don't need to change their logic based on where they are deployed. Do NOT create hidden directories like `.ohc/` in the repository root.

## Design Doc
**Architecture:**
- Add a new package `src/server/lib/integrations/hybrid_blob/`.
- Introduce a `BlobManager` that implements the MCP Tool interface.
- Must support a local File System driver and a cloud S3-compatible driver, dynamically choosing based on configuration or environment variables (e.g., presence of `S3_ENDPOINT`).

**API Contracts:**
- `WriteBlob(ctx async context, key string, data []byte) error`
- `ReadBlob(ctx async context, key string) ([]byte, error)`

**DB Schema Changes:**
- None required; relies on file system or object storage APIs.

**Security:**
- Must validate `organization_id` prefixes in cloud mode to prevent cross-tenant blob access.
- Ensure strict path traversal protections for the local File System driver.

## Implementation Prompt
"Implement the Hybrid Blob Storage MCP tool in `src/server/lib/integrations/hybrid_blob/`.
1. Create `blob.rs` defining the `BlobManager` and its MCP capabilities (`ReadBlob` and `WriteBlob`).
2. Implement environment-agnostic logic. To determine if the backend should be S3, check for `S3_ENDPOINT` environment variable. If missing, fall back to a local temporary directory configured by the environment (do not use `.ohc/`).
3. Implement strict path sanitization to prevent path traversal attacks in the local driver.
4. For the S3 driver, use the official AWS SDK for Go v2 (or an S3-compatible equivalent) to handle `PutObject` and `GetObject`. Ensure object keys are prefixed with the tenant's `organization_id` to enforce isolation.
5. Create tests in `blob_test.rs` using `tempfile::tempdir()` for isolated local testing. Mock the S3 client for cloud-mode tests. Never hardcode workspace directories.
6. Create at least one comprehensive E2E test for the new feature that fulfills the E2E Test Standard, testing from the UI layer to the new blob capability and verifying the final state. No network mocking is allowed, but AI responses should be mocked.
7. Update or create the adjacent `BUILD.bazel` file, ensuring `srcs` array accurately reflects the new files and dependencies."

## Priority
P2

## Estimated Scope
Medium
