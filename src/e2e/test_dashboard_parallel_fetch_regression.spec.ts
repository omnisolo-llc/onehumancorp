import { test, expect } from './fixtures';

test.describe('Parallel Execution Optimization - Cart & Growth', () => {
  test('Cart loading is successful and data is present', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/checkout');
    await expect(page.locator('text=Checkout').first()).toBeVisible({ timeout: 10000 });
  });

  test('Affiliate stats loading in growth dashboard displays correct UI elements', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard/growth/affiliates');
    await expect(page.locator('text=Affiliate').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Active Affiliates').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Paid Commissions').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Track and manage your affiliate').first()).toBeVisible({ timeout: 10000 });
  });

  test('Affiliate endpoint returns properly structured parallel fetch payload', async ({ request, loginAs, adminUser, page }) => {
    await loginAs(page, adminUser);
    const response = await request.get('/api/v1/growth/affiliate/stats', {
      headers: {
        "x-tenant-id": adminUser.tenantId || "e2e-tenant",
        "Authorization": `Bearer ${adminUser.token || "e2e-token"}`
      }
    });
    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('total_affiliates');
    expect(data).toHaveProperty('total_commission_cents');
  });

  test('Reputation stats loading in growth dashboard displays correct UI elements', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard/growth/reputation');
    await expect(page.locator('text=Reputation').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Recent Reviews').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Average Rating').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Monitor your brand reputation').first()).toBeVisible({ timeout: 10000 });
  });

  test('Reputation endpoint returns properly structured parallel fetch payload', async ({ request, loginAs, adminUser, page }) => {
    await loginAs(page, adminUser);
    const response = await request.get('/api/v1/growth/reputation/stats', {
      headers: {
        "x-tenant-id": adminUser.tenantId || "e2e-tenant",
        "Authorization": `Bearer ${adminUser.token || "e2e-token"}`
      }
    });
    expect(response.status()).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('average_rating');
    expect(data).toHaveProperty('total_reviews');
  });

  test('Unified Feed is fast and functional', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('text=Recent Orders').first()).toBeVisible({ timeout: 10000 });
    await expect(page.locator('text=Operations Map').first()).toBeVisible({ timeout: 10000 });
  });

  test('Agent feed loads correctly', async ({ page, loginAs, adminUser }) => {
    await loginAs(page, adminUser);
    await page.goto('/dashboard');
    await expect(page.locator('text=Action Required').first()).toBeVisible({ timeout: 10000 });
  });
});
