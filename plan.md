1. **Database Schema Update**:
   - Add `has_completed_setup` to the `users` table using a Goose migration. I will create `20260430000000_add_has_completed_setup_pg.sql` and `20260430000000_add_has_completed_setup_sqlite.sql` under `src/server/db/migrations/`.
   - Update `src/server/db/BUILD.bazel` to register these two new migration files.
   - Run `ls src/server/db/migrations` and `cat src/server/db/BUILD.bazel` to verify the creation and modifications are correct.

2. **Backend Domain Updates**:
   - Update `User` and `UserPublic` structs in `src/server/auth/store.go` to include `HasCompletedSetup bool` mapped to JSON `has_completed_setup`.
   - Update `PublicView()` method in `src/server/auth/store.go` to copy this field.
   - Update `scanUser` and all `SELECT` queries in `src/server/auth/postgres_store.go` to fetch `has_completed_setup`. Also update `CreateUser` to map the boolean value in the insert query.
   - Add `MarkSetupCompleted(ctx context.Context, id string, orgID string) error` to `UserRepository` in `src/server/auth/repository.go`, `PgUserRepository` in `src/server/auth/postgres_store.go`, and to `auth.Store` in `src/server/auth/store.go`.
   - Run `bazelisk test //src/server/auth/...` to ensure the package still compiles and passes tests.

3. **Backend Logic Update**:
   - In `handleWizardConfigure` (`src/server/dashboard/handlers_wizard.go`), fetch the user from context using `auth.ClaimsFromContext(r.Context())`. If a valid user is present, call `s.authStore.MarkSetupCompleted(r.Context(), claims.Subject, claims.OrganizationID)` on the auth store to flag the user as having completed the setup.

4. **Frontend Domain Update**:
   - Update `AuthUser` struct and `AuthUser.fromJson` in `src/app/lib/services/auth_service.dart` to decode `hasCompletedSetup` from JSON mapping to `has_completed_setup` field in dart.
   - Update `UserPublic` class in `src/app/lib/models/user.dart` to decode `hasCompletedSetup` from JSON mapping to `has_completed_setup` field in dart.

5. **Frontend Auto-redirect Logic**:
   - In `src/app/lib/router.dart`, update the `redirect` method in `GoRouter`.
   - If the user is logged in, their `hasCompletedSetup` flag is false, and the current route is not `/business_setup` or `/login` or `/landing`, return `/business_setup` as a redirect target.
   - Upon successful login via `/login`, if `hasCompletedSetup` is false, they should go to `/business_setup` instead of `/dashboard`.
   - In `src/app/lib/screens/business_setup_wizard_screen.dart`, update the `launch` method to call `ref.invalidate(authStateProvider)` to force a re-fetch of the `/api/auth/me` endpoint to reflect the updated `hasCompletedSetup` status so they can access the dashboard.

6. **Submit Changes**:
   - Push and merge changes to close the task.
