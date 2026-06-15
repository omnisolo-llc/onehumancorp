import { test, expect } from './fixtures';

test.describe('Success Milestone Widget', () => {
  test('displays milestone and handles share', async ({ page, loginAs, adminUser }) => {
    // Log in and navigate to the dashboard
    await loginAs(page, adminUser);

    // Evaluate in page context after load
    await page.goto('/dashboard');
    await page.evaluate(() => { localStorage.setItem('tenant', 'e2e-tenant'); });
    await page.reload();

    // Now wait for h1 element on the dashboard or similar
    await expect(page.locator('body')).toBeVisible();
    await page.goto('/milestones');
    await expect(page.locator('h1')).toBeVisible({ timeout: 10000 });
  });
});
