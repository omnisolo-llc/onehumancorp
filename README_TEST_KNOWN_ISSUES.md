## Testing Postgres/Docker Setup Notes
During E2E testing, the local Bazel test runner tries to spin up `pgvector/pgvector:pg16` via Docker. If you encounter Docker overlayfs permission errors on Linux (`failed to convert whiteout file ... operation not permitted`), you can skip E2E test execution locally, as this is a known issue with the underlying host OS filesystem in the sandbox.
