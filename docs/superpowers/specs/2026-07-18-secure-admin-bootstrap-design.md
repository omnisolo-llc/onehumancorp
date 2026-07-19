# Secure Admin Bootstrap Design

## Goal

Provide a real, authenticated first-user path for container and Kubernetes deployments without reopening the Next.js authentication bypass or exposing a permanent unauthenticated registration endpoint.

## Decision

Use a one-time `/api/v1/setup/admin` endpoint protected by a deployment-supplied setup token. Direct database mutation was rejected because it makes E2E backend-specific and bypasses application behavior. An unauthenticated first-user endpoint was rejected because network races could let an attacker claim a fresh deployment.

## Server contract

- The endpoint is enabled only when `OHC_SETUP_TOKEN` is at least 32 bytes.
- Requests use `Authorization: Bearer <setup-token>` and are compared without leaking token contents.
- The request accepts `username`, `email`, `password`, and `organizationId`; the server assigns the fixed `ADMIN` role.
- The server creates the tenant when absent, creates the first admin transactionally, and returns conflict once an admin exists.
- Invalid configuration, credentials, database errors, and duplicate setup fail closed and never expose sensitive details.
- The route is mounted only at `/api/v1/setup/admin`; all normal API calls still require a login bearer token.

## Deployment and test flow

Compose and Kind inject short-lived E2E setup/admin credentials. Their smoke tests wait for readiness, call setup once, log in through `/api/v1/auth/login`, extract the real JWT, and attach it to every protected request. The Compose bootstrap helper writes its marker only after a successful or already-complete response and exits nonzero for every unexpected result.

## Verification

Rust tests cover missing, short, and incorrect setup tokens; successful creation; fixed role; and one-time conflict behavior. Shell contracts cover the versioned endpoint, bearer use, and fail-closed marker behavior. Real Kind and Compose E2E runs prove the complete setup-login-protected-data path.
