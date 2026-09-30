import { test, expect } from './fixtures';
import { authenticateRequest } from './authenticate';

test.describe('Database-seeded authentication', () => {
  test('admin user logs in through the real UI', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByText('Welcome back')).toBeVisible();
  });

  test('regular team member logs in through the real UI', async ({ memberPage }) => {
    await expect(memberPage.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(memberPage.getByText('Welcome back')).toBeVisible();
  });
});

// A logout-capable audit must have a unique backend token, even for the same
// owner. Revoking a copied global storage state would poison later test files.
test('isolated owner logout leaves the shared read-only session usable', async ({ page, anonymousPage, adminUser, baseURL }) => {
  const origin = new URL(baseURL!).origin;
  const sharedBefore = await page.request.get('/api/v1/agent-feed');
  expect(sharedBefore.status()).toBe(200);
  await authenticateRequest(anonymousPage.request, {
    username: adminUser.email, password: adminUser.password, organizationId: adminUser.organizationId,
  }, origin);
  const logout = await anonymousPage.request.post('/api/v1/auth/logout', {
    headers: { origin, 'sec-fetch-site': 'same-origin' },
  });
  expect(logout.status()).toBe(200);
  expect((await anonymousPage.request.get('/api/v1/agent-feed')).status()).toBe(401);
  expect((await page.request.get('/api/v1/agent-feed')).status()).toBe(200);
});
