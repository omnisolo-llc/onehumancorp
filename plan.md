1. **Analyze Requirements:**
   - Create a Go AST traversal test named `TestAutomatedComplianceGuardrailsForPIILogging`.
   - The test must statically scan the codebase for unredacted sensitive keywords in log/print statements.
   - It should identify calls to logging functions (`slog.Info`, `fmt.Printf`, `log.Println`, etc.).
   - It must check the string arguments passed to these functions. If a string argument contains a sensitive keyword (like "password", "email", "ssn", "secret", "token") AND is not wrapped in a redaction function (`telemetry.RedactPII`, `RedactInterfacePII`, etc.), it should flag it.
   - However, many valid logs contain the word "error" or "token" safely. Let's make the test specific to avoid false positives, or we can use a list of common PII keys that appear in `slog` key-value pairs (e.g. `slog.String("email", email)`). If the key is exactly "email" or "password", and the value is not a redaction call.

2. **Develop the Test Logic in `src/server/telemetry/compliance_guardrails_test.go`:**
   - Iterate over all `.go` files in `src/server` (skipping tests `_test.go` and vendor directories).
   - Parse each file using `go/parser`.
   - Traverse the AST using `ast.Inspect`.
   - Look for `ast.CallExpr`. Check if the function called is a logging method (from packages `slog`, `fmt`, `log`).
   - If it is a log call, check its arguments.
   - For PII keywords: "password", "ssn", "secret", "token", "email", "credit_card", "phone".
   - How to check if unredacted?
     - Option A: Look at `slog` key-value arguments. If an argument is a string literal containing a sensitive keyword (like `"user_email"` or `"password"`), check the *next* argument (the value). If the next argument is NOT a call to a redactor function, flag it.
     - Option B: Just check if *any* string literal inside a log call matches exactly `"password"`, `"ssn"`, etc. and if the log call doesn't have a redactor in its arguments, it's a violation.
     - Let's look at `slog.X` calls specifically because they take alternating keys and values. The test could check `slog.Info`, `slog.Warn`, `slog.Error`, `slog.Debug` and their Context variants.
     - A simple and robust check: For any `CallExpr` where the function is `slog.*`, we iterate over arguments. If an argument `i` is a `BasicLit` (string) whose value (stripped of quotes) exactly matches or loosely matches a sensitive key, we look at argument `i+1`. If argument `i+1` is an `ast.Ident` or `ast.SelectorExpr` (a variable) and not an `ast.CallExpr` to a redactor, we report an error.
     - Also we should check `fmt.Printf` and `log.Printf` format strings for these keywords. If the format string contains "password=" or "email=", it's a violation.

3. **Refine PII Keys and Logging Functions:**
   - Logging functions: `slog.Info`, `slog.Error`, `slog.Warn`, `slog.Debug`, `slog.InfoContext`, `slog.ErrorContext`, `slog.WarnContext`, `slog.DebugContext`, `fmt.Print`, `fmt.Printf`, `fmt.Println`, `log.Print`, `log.Printf`, `log.Println`.
   - Sensitive Keywords: `"password"`, `"ssn"`, `"secret"`, `"token"`, `"email"`.

4. **Add to `src/server/telemetry/compliance_guardrails_test.go`:**
   - Set up the file.
   - Run `bazelisk test //...` to ensure it passes on the codebase, or if it finds violations, add an exception list or fix the violations in the codebase! The problem says: "implement explicit `go/ast` traversal tests... rather than relying on runtime dummy tests."

5. **Verify and Pre-commit:**
   - Ensure the new test passes and is integrated properly.
   - Call `pre_commit_instructions` before submitting.
