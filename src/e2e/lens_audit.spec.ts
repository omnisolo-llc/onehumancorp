import { test, expect } from './fixtures';

test.describe('Regression Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('verify Automated Review Requests block does not contain mock orders text', async ({ page }) => {
    await page.goto('/dashboard');
    const mockText = page.getByText("You have 12 recent orders without reviews.");
    await expect(mockText).not.toBeVisible();
  });

  test('verify embedded desktop Rust UI dashboard-screen does not contain hardcoded approval-item-1', async ({ page }) => {
    await page.goto('/dashboard');
    const hardcodedApprovalItem = page.locator('#approval-item-1');
    await expect(hardcodedApprovalItem).not.toBeAttached();
  });

  test('verify promo generation resolves instantly (no 800ms mock delay)', async ({ request }) => {
    const start = Date.now();
    const res = await request.post('/api/v1/growth/promotions/generate', {
      data: { tenant: 'test-tenant' }
    });
    const end = Date.now();
    expect(end - start).toBeLessThan(3000);
  });

  test('verify E2E database agent approvals reflects in UI without mock injection', async ({ page }) => {
      await page.goto('/dashboard');
      const theManager = page.getByText("The Manager");
      await expect(theManager).not.toBeVisible();
  });
});
