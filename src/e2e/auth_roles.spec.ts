import { test, expect } from './fixtures';
import type { Page } from '@playwright/test';
import type { E2EUser } from './identities';

async function signInThroughUi(page: Page, user: E2EUser) {
  await page.goto('/login');
  await page.getByLabel('Email or username').fill(user.email);
  await page.getByLabel('Password', { exact: true }).fill(user.password);
  await page.getByLabel('Organization', { exact: false }).fill(user.organizationId);
  const responsePromise = page.waitForResponse((response) =>
    new URL(response.url()).pathname === '/api/v1/auth/login' &&
    response.request().method() === 'POST',
  );
  await page.getByRole('button', { name: 'Log in', exact: true }).click();
  const response = await responsePromise;
  expect(response.status()).toBe(200);
  const body = await response.json();
  expect(body.user.organizationId).toBe(user.organizationId);
  expect(body.user.roles).toContain(user.role);
  await expect(page).toHaveURL(/\/dashboard$/);
  await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
  await expect(page.getByText('Welcome back', { exact: true })).toBeVisible();
  // Reload also proves that the real server-created session, not local component
  // state or the default pre-authenticated fixture, keeps this actor signed in.
  await page.reload();
  await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
}

test.describe('Database-seeded authentication', () => {
  test('admin user logs in through the real UI', async ({ anonymousPage, adminUser }) => {
    await signInThroughUi(anonymousPage, adminUser);
  });

  test('regular team member logs in through the real UI', async ({ anonymousPage, memberUser }) => {
    await signInThroughUi(anonymousPage, memberUser);
  });
});
