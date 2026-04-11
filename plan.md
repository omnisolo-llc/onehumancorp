1. Fix path traversal vulnerability in `provider.go`.
   - Update `resolvePath` in both `LocalFSProvider` and `CloudFSProvider` to ensure the trailing separator is present before checking the prefix, or by using `filepath.Rel` and checking for `..`.
2. Fix lack of context injection in `provider.go` and `server.go`.
   - The mission requires `auth.Claims` context injection to handle multi-tenancy dynamically, instead of reading a static `OHC_TENANT_ID` env variable in the factory.
   - We need to import the project's `auth` package (`github.com/onehumancorp/mono/srcs/server/auth`) or use the context object properly. Wait, I will need to inspect how auth is handled in this codebase first to implement `auth.Claims` context correctly. Let me run a command to find the auth package.
   - Update `FileSystemProvider` interface methods to accept `context.Context` as the first parameter.
   - In `CloudFSProvider`, extract the tenant ID from the context dynamically instead of taking it in the constructor.
3. Update `hybridfsmcp_test.go` to pass context and verify the new behavior.
4. Run tests `bazelisk test //srcs/server/tools/hybridfsmcp/...`.
5. Request code review again.
6. Complete pre-commit steps.
7. Create PR / Submit.
