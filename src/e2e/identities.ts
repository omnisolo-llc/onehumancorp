// Accounts exist only in the isolated e2e-seed.sql database. This module has no
// Playwright runtime import, so setup can share identities without defining tests.
export const E2E_ADMIN_USER = {
  email: process.env.OMNISOLO_E2E_ADMIN_EMAIL ?? process.env.OMNISOLO_ADMIN_EMAIL ?? 'test@example.com',
  password: process.env.OMNISOLO_E2E_ADMIN_PASSWORD ?? process.env.OMNISOLO_ADMIN_PASSWORD ?? 'password123',
  role: 'ADMIN',
  organizationId: process.env.OMNISOLO_E2E_ADMIN_ORGANIZATION_ID ?? process.env.OMNISOLO_ADMIN_ORGANIZATION_ID ?? 'e2e-tenant',
} as const;

export const E2E_UNLIMITED_ADMIN_USER = {
  email: 'pro@example.com', password: 'password123', role: 'ADMIN', organizationId: 'e2e-tenant-unlimited',
} as const;

export const E2E_MEMBER_USER = {
  email: 'member@example.com', password: 'MemberPass123!', role: 'OPERATOR', organizationId: 'e2e-tenant',
} as const;

export const E2E_STARTER_USER = {
  email: 'starter@example.com', password: 'password123', role: 'ADMIN', organizationId: 'e2e-tenant',
} as const;

export type E2EUser = typeof E2E_ADMIN_USER | typeof E2E_UNLIMITED_ADMIN_USER | typeof E2E_MEMBER_USER | typeof E2E_STARTER_USER;
