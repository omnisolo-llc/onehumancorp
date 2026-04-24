1.  **Add `SPIFFEIDMiddleware`**:
    *   Create a new middleware function `SPIFFEIDMiddleware` in `src/server/auth/middleware.go` or a new file like `src/server/auth/spiffe.go` (if creating a new file, ensure it's registered in `BUILD.bazel`). Let's add it to `src/server/auth/middleware.go` to be simpler.
    *   This middleware will extract the Bearer token from the `Authorization` header.
    *   It will pass the token to `interop.ValidateSPIFFEID()`.
    *   If validation fails, it returns a 401/403 status.
    *   If it succeeds, it injects the token into the request context (or just lets the request pass if the inner handler doesn't need the token string explicitly). Wait, let's inject the token into the context so `next.ServeHTTP` can access it if needed. The instruction says: "To enforce secure internal service-to-service authentication in OHC HTTP handlers, implement middleware that extracts the Bearer token and validates it using interop.ValidateSPIFFEID."

2.  **Add tests for `SPIFFEIDMiddleware`**:
    *   Update `src/server/auth/auth_test.go` or create `src/server/auth/spiffe_test.go` to test the new middleware:
        *   No token -> error.
        *   Invalid token (`interop.ValidateSPIFFEID` fails) -> error.
        *   Valid token (`interop.ValidateSPIFFEID` succeeds) -> pass.

3.  **Run full suite tests (`bazelisk test //...`)**.
