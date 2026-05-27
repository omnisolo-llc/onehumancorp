import { test, expect } from './fixtures';

test.describe('Audit Verification E2E Tests', () => {
  test('verify dashboard loads without mock data', async ({ page }) => {
    await page.goto('/');
    const dashboardLink = page.getByRole('link', { name: 'Dashboard' }).first();
    await dashboardLink.click();

    // The hardcoded 5 abandoned carts and $240.00 should NOT be visible
    await expect(page.getByText('5 abandoned carts')).not.toBeVisible();
    await expect(page.getByText('$240.00')).not.toBeVisible();

    // The "Action Required: Connect Stripe" banner should NOT be visible
    await expect(page.locator('text=1 Action Required: Connect Stripe to accept payments.')).not.toBeVisible();
  });

  test('verify no mock delays when loading dashboard metrics', async ({ page }) => {
    const start = Date.now();
    await page.goto('/dashboard');
    // Ensure dashboard header is visible quickly
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible({ timeout: 5000 });
    const end = Date.now();
    // Real API fetch should be fast locally, not artificially delayed by setTimeout
    expect(end - start).toBeLessThan(5000);
  });

  test('verify database approvals are loaded correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Action Required' })).toBeVisible({ timeout: 10000 });
    // This text should come from the actual database seed data
    await expect(page.getByText('Draft email for review')).toBeVisible();
  });

  test('verify team activity section handles empty state or real data', async ({ page }) => {
    await page.goto('/dashboard');
    const teamActivityHeader = page.getByRole('heading', { name: 'Team Activity' });
    await expect(teamActivityHeader).toBeVisible();

    // We expect either real activity or the loading state
    const loadingState = page.getByText('Waiting for team activity...');
    const hasActivity = await page.locator('.ohc-hybrid-panel div').count() > 0;

    expect(await loadingState.isVisible() || hasActivity).toBeTruthy();
  });

  test('verify the product count increments correctly', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Products' })).toBeVisible();

    const countText = page.getByText('10 / 10 Products Used');
    await expect(countText).toBeVisible();

    const addProductBtn = page.getByRole('button', { name: '+ Add Product' });
    await addProductBtn.click();

    // The mock logic should no longer block this with a fake paywall modal
    await expect(page.getByText('11 / 10 Products Used')).toBeVisible();
  });
});
