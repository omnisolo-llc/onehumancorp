1. **Define `BlobProvider` interface:** Abstract file writing logic into a new interface `mcp.BlobProvider` in `srcs/server/agents/mcp/blob_provider.go`. It should at least include `WriteBlob(ctx context.Context, key string, data []byte) error` and `ReadBlob(ctx context.Context, key string) ([]byte, error)`.
2. **Implement `LocalBlobProvider`:** Create `srcs/server/agents/mcp/local_blob.go` which writes to `/var/tmp/ohc/blobs` using standard `os` package file operations.
3. **Implement `S3BlobProvider`:** Create `srcs/server/agents/mcp/s3_blob.go` which writes to AWS S3. Since AWS SDK isn't present, we'll create a simulated/stub implementation using `ohc-multi-tenant-blobs` to match the exact requirement from the problem statement, or see if there's any existing implementation we can reuse. The request says "AWS S3 `ohc-multi-tenant-blobs`".
4. **Implement Factory:** Create a factory method in `srcs/server/agents/mcp/factory.go` that selects the provider based on environment variables:
   - If `OHC_STANDALONE` is set to "true", return `LocalBlobProvider`.
   - If `OHC_MULTITENANT` is set to "true" or `OHC_STANDALONE` is not true, return `S3BlobProvider`.
5. **Update BUILD.bazel:** Update `srcs/server/agents/mcp/BUILD.bazel` to include the new files.
6. **Write tests:** Add tests for the factory logic, LocalBlobProvider, and S3BlobProvider.
7. **Pre-commit checks**: Run Bazel tests, ensure everything passes, and complete necessary review steps.
