# Global Identity Email Claims Design

## Goal

Guarantee that one normalized email maps to at most one persisted user globally across tenants and authentication methods.

## Architecture

Add a portable `identity_email_claims` table keyed by `normalized_email`. Each row stores the owning `user_id` and `claimed_at`. Password registration and first-time OIDC registration insert the claim and user in the same database transaction; a primary-key conflict denies the second identity. OIDC continues to report the explicit-link denial when the email is already reserved.

Database migration creates the table and backfills all existing users. It normalizes and sorts users deterministically, validates the complete set before inserting anything, and fails with the colliding normalized email when two user IDs normalize to the same address. It never chooses or overwrites an owner.

The portable admin bootstrap reconciles its claim in the same transaction as user creation or update. An absent claim is inserted, a claim already owned by that user is accepted, and a claim owned by another user fails closed.

## Data Flow and Errors

- Password registration conditionally claims the ticket and invitation, claims the normalized email, then inserts the user before committing.
- OIDC registration reads mode, selects and conditionally consumes any required invitation, claims the normalized email, inserts the user and external identity, then commits.
- A failed claim rolls back ticket, invitation, user, and external-identity mutations.
- OIDC claim conflicts return `existing account must explicitly link this provider`.
- Password claim conflicts return `account already exists`.
- Migration collisions report only the normalized email, not hashes or credentials.

## Testing

Use file-backed SQLite with independent connections to prove concurrent password tickets and independent OIDC subjects cannot create duplicate normalized emails. Add migration tests for successful backfill and fail-closed collision detection, plus bootstrap tests for absent, same-owner, and different-owner claims. Run targeted tests and the complete serialized `server_auth` suite.
