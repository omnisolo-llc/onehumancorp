1.  **Add Database Migrations**:
    *   Create a new migration file (e.g., `srcs/server/db/migrations/20260416030000_add_referral_fields_to_users.sql`).
    *   Add `referral_code` (TEXT UNIQUE) and `referred_by` (TEXT, foreign key to `users(id)`) columns to the `users` table.
    *   Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.

2.  **Update `growth` Service API**:
    *   Currently `referral_api.go` contains a `/api/referrals` handler (though it's not actually applying the referral yet). We need to implement `POST /api/referrals/apply` endpoint.
    *   Wait, the issue states: "Create `POST /api/referrals/apply` endpoint." Let's add an `ApplyReferralHandler` to `referral_api.go` (or create an `apply_referral.go`). This handler should accept a `user_id` and a `referral_code`.
    *   It should interact with the database to set the `referred_by` field for the user and increment referral counts. Or, wait, the `ReferralTracker` in `referrals.go` is an in-memory struct right now. Let's see if we should use DB instead of the in-memory `ReferralTracker`, or update the `ReferralTracker` to interact with the database. The issue explicitly states: "Add a referral_code to the users table. Add a referred_by to the users table (foreign key to users.id)." This implies the database must be used.

3.  **Update Database Layer (`users` representation)**:
    *   If there's a `srcs/server/db/models/user.go` or similar, update the Go structs. I should check if one exists.
    *   If no explicit model file, the handler can just execute raw SQL using `db.Provider`.

4.  **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done**:
    *   Run tests using `bazelisk test //srcs/server/services/growth/... //srcs/server/db/...`.

5.  **Submit PR**:
    *   Commit changes and create a PR with the required format `🚀 Nova: [growth] Implement user referral tracking system`. Update the GitHub issue.
