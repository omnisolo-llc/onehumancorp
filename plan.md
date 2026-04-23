1. **Add Registration Backend:**
   - Modify `srcs/server/auth/handlers.go`: Add `HandleRegister` which receives a `registerRequest` (Username, Email, Password). It calls `h.store.CreateUser` (with default role `viewer` and org `sys` or empty if not provided), and then generates a token exactly like `HandleLogin`.
   - Modify `srcs/server/dashboard/server.go`: Expose `mux.HandleFunc("/api/auth/register", server.authHandlers.HandleRegister)`.
   - Update `srcs/server/auth/middleware.go` to add `"/api/auth/register"` to the `publicRoutes` array.
   - Update `srcs/server/dashboard/tenant.go` to allow `"/api/auth/register"` to hit the public router gracefully.
2. **Update Flutter Auth Service (`srcs/app/lib/services/auth_service.dart`):**
   - Add `Future<AuthUser> register(String email, String password, String name)` to `AuthService`, sending `POST /api/auth/register` with `{'username': name, 'email': email, 'password': password}`.
   - Add `Future<void> register(String email, String password, String name)` to `AuthNotifier`, calling `service.register` and updating the `state` & preferences.
3. **Update Flutter Login UI (`srcs/app/lib/screens/login_screen.dart`):**
   - Add a boolean `_isLogin = true;` to `_LoginScreenState`.
   - Add a name field (`_nameCtrl`) for registration.
   - Add a toggle button or text button (e.g. "Don't have an account? Sign Up" / "Already have an account? Sign In").
   - Update `_submit()` to call `login` or `register` depending on `_isLogin`.
   - Change strings like "Sign in to orchestrate your swarm" dynamically based on `_isLogin`.
4. **Testing:**
   - Execute tests (`bazelisk test //srcs/server/...` and Flutter tests `cd srcs/app && flutter test`).
5. **Pre-commit steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
