import { test, expect } from './fixtures';

async function loginThroughUI(
  page: Parameters<typeof test>[0] extends never ? never : any,
  user: { email: string; password: string; organizationId: string },
) {
  await page.goto('/login');
  await page.getByLabel('Email or username').fill(user.email);
  await page.getByLabel('Password').fill(user.password);
  await page.getByLabel('Organization').fill(user.organizationId);
  await page.getByRole('button', { name: 'Log in' }).click();
  await expect(page).toHaveURL(/\/dashboard(?:[?#].*)?$/, { timeout: 15000 });
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  await expect(page.getByText('Welcome back')).toBeVisible();
}

test.describe('Database-seeded authentication', () => {
  test('admin user logs in through the real UI', async ({ anonymousPage, adminUser }) => {
    await loginThroughUI(anonymousPage, adminUser);
  });

  test('regular team member logs in through the real UI', async ({ anonymousPage, memberUser }) => {
    await loginThroughUI(anonymousPage, memberUser);
  });
});
