1. **Explore & Analyze Onboarding Flow Requirements**: Understand the required changes based on the user instruction. The user instruction mentions "Sign-Up & Account Creation" as step 1. The instruction says: "Frictionless registration — email + password, or one-tap Google / Apple SSO. Email verification with a prominent 'Resend' link. Auto-redirect to the business setup wizard on first login." We will build the email + password flow with simulated SSO and email verification states.
2. **Backend: Add `/api/auth/register` Endpoint**:
   - Create a `HandleRegister` method in `src/server/auth/handlers.go`.
   - Update `src/server/dashboard/server.go` to route POST `/api/auth/register` to `HandleRegister`.
   - The handler should call `store.CreateUser` with `RoleAdmin`.
   - If successful, it should immediately issue a JWT token like the `login` endpoint, so the user is logged in automatically.
3. **Frontend: Update `AuthService`**:
   - Add `register(String email, String password)` to `AuthService` and `AuthNotifier` in `src/app/lib/services/auth_service.dart`.
4. **Frontend: Create `SignUpScreen`**:
   - Create `src/app/lib/screens/signup_screen.dart` to allow users to sign up. Modeled after `login_screen.dart`.
   - Handle the API call to `/api/auth/register`.
   - Add UI states for "Email verification" (simulate sending the email and a "Resend" button, but for this non-blocking flow, proceed on successful backend creation).
5. **Frontend: Update Routing & Auth Flow**:
   - Update `src/app/lib/router.dart` to include `/signup`.
   - Ensure a "Create an account" link is on the `LoginScreen`.
   - Add logic so new users go to `/business_setup` instead of `/dashboard`. (To do this simply, `/signup` can push to `/business_setup` on success).
6. **E2E Testing**:
   - Create `src/app/e2e/signup_ux.spec.ts` to test the end-to-end signup flow.
   - Run tests to verify the UI.
7. **Pre-commit Checks**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
