1. **Update `src/server/auth/handlers.go`**
   - Add `registerRequest` struct containing `username`, `email`, `password`, `organizationId`.
   - Add `HandleRegister` method to `Handlers` that accepts `POST /api/auth/register`.
   - Decode JSON request, validate inputs.
   - Call `user, err := h.store.CreateUser(req.Username, req.Email, req.Password, []string{RoleViewer}, req.OrganizationID)`.
   - If error, return `400 Bad Request` using `jsonError(w, err.Error(), http.StatusBadRequest)`.
   - Issue a JWT via `token, err := h.store.IssueToken(user)`.
   - Write response using `writeJSON(w, http.StatusCreated, loginResponse{Token: token, User: user.PublicView(), ExpiresAt: time.Now().UTC().Add(tokenTTL)})`.

2. **Update `src/server/dashboard/server.go`**
   - Add `mux.HandleFunc("/api/auth/register", server.authHandlers.HandleRegister)`.

3. **Update `src/app/lib/services/auth_service.dart`**
   - We verified `login` method implementation in `AuthNotifier` explicitly via grep.
   - Add `Future<AuthUser> register(String username, String email, String password)` to `AuthService`. It will `POST /api/auth/register` with `{'username': username, 'email': email, 'password': password}`, parse `data['token']` and `data['user']`, and return `AuthUser.fromJson(user, token)`.
   - Add `Future<void> register(String username, String email, String password)` to `AuthNotifier`. Set `state = const AsyncLoading()`, call `ref.read(authServiceProvider).register`, save `user.token` to `SharedPreferences` via `_prefsProvider` within `AsyncValue.guard`, and return `user`.

4. **Update `src/app/lib/screens/login_screen.dart`**
   - Add `bool _isRegistering = false` state and `_emailCtrl` TextEditingController.
   - Conditionally adjust headers based on `_isRegistering`.
   - Insert an "Email" `TextFormField` when `_isRegistering` is true.
   - Update `_submit()`: `if (_isRegistering)` call `await ref.read(authStateProvider.notifier).register(_usernameCtrl.text.trim(), _emailCtrl.text.trim(), _passwordCtrl.text)`, `else` call `login`.
   - Toggle button below SSO for `_isRegistering`: `_isRegistering ? "Already have an account? Sign In" : "Don't have an account? Sign Up"`.
   - Add dummy "Resend Email Verification" toggle button if `_isRegistering`.

5. **Verify changes are made properly**
   - Use `cat` or `read_file` to verify the changes written to `src/server/auth/handlers.go`, `src/server/dashboard/server.go`, `src/app/lib/services/auth_service.dart`, and `src/app/lib/screens/login_screen.dart`.

6. **Update Backend Tests**
   - Use `replace_with_git_merge_diff` to add `TestHandleRegister_Success`, `TestHandleRegister_Duplicate`, and `TestHandleRegister_InvalidJSON` in `src/server/auth/auth_test.go` testing the logic of `HandleRegister`.

7. **Verify backend test modifications**
   - Run `cat src/server/auth/auth_test.go | grep TestHandleRegister` to ensure tests were added correctly.

8. **Update E2E testing**
   - Create `src/app/e2e/register.spec.ts` using `write_file`.
   - Load `/`, click "Don't have an account? Sign Up", fill in Email, Username, Password, click "Sign Up", and expect successful login (e.g. verify routing to `/business_setup` or `/dashboard`).

9. **Verify E2E test file creation**
   - Run `cat src/app/e2e/register.spec.ts` to ensure the test was written correctly.

10. **Final Verification Step**
    - Run `bazelisk test //...` to ensure all backend, frontend, and E2E tests pass and that coverage is maintained.

11. **Pre-commit Steps**
    - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
