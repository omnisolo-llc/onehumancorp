import { test, expect } from './fixtures';

test.describe('Database-seeded authentication', () => {
  test('admin user logs in through the real UI', async ({ anonymousPage: page, adminUser }) => {
    await page.goto('/login');
    await page.getByLabel('Email or username').fill(adminUser.email);
    await page.getByLabel('Password', { exact: true }).fill(adminUser.password);
    await page.getByLabel('Organization', { exact: false }).fill(adminUser.organizationId);
    const responsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/auth/login' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Log in', exact: true }).click();
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.user.username).toBe(adminUser.email);
    expect(body.user.roles).toContain(adminUser.role);
    expect(body.user.organizationId).toBe(adminUser.organizationId);
    await expect(page).toHaveURL(/\/dashboard(?:[?#].*)?$/);
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(page.getByText('Welcome back', { exact: true })).toBeVisible();
  });

  test('regular team member logs in through the real UI', async ({ anonymousPage: page, memberUser }) => {
    await page.goto('/login');
    await page.getByLabel('Email or username').fill(memberUser.email);
    await page.getByLabel('Password', { exact: true }).fill(memberUser.password);
    await page.getByLabel('Organization', { exact: false }).fill(memberUser.organizationId);
    const responsePromise = page.waitForResponse(response =>
      new URL(response.url()).pathname === '/api/v1/auth/login' && response.request().method() === 'POST');
    await page.getByRole('button', { name: 'Log in', exact: true }).click();
    const response = await responsePromise;
    expect(response.status()).toBe(200);
    const body = await response.json();
    expect(body.user.username).toBe(memberUser.email);
    expect(body.user.roles).toContain(memberUser.role);
    expect(body.user.organizationId).toBe(memberUser.organizationId);
    await expect(page).toHaveURL(/\/dashboard(?:[?#].*)?$/);
    await expect(page.getByRole('heading', { name: 'Dashboard', exact: true })).toBeVisible();
    await expect(page.getByText('Welcome back', { exact: true })).toBeVisible();
  });
});
