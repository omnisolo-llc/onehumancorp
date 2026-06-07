import { test, expect } from '@playwright/test';
import { memberPage } from './fixtures';

test.describe('Auto Milestone Alert Growth Feature', () => {
  test('renders the milestone alert when orders cross threshold and handles sharing', async ({ memberPage }) => {
    // 1. Setup mock route for /api/ui/dashboard/metrics
    await memberPage.route('/api/ui/dashboard/metrics**', async route => {
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          active_customers: 25,
          pending_orders: 2,
          total_sales: 10, // Milestone 10
          total_revenue: 1500,
          total_campaigns_sent: 5,
        })
      });
    });

    await memberPage.addInitScript(() => {
        window.localStorage.removeItem('dismissed_milestone_e2e-tenant');
    });

    await memberPage.goto('/dashboard');

    // Check that the Milestone Alert is rendered
    await expect(memberPage.locator('text=Achievement Unlocked: 10th Order! 🚀')).toBeVisible();

    // Check share links
    const shareXBtn = memberPage.locator('[data-testid="share-milestone-x"]');
    await expect(shareXBtn).toBeVisible();
    await expect(shareXBtn).toHaveAttribute('href', /twitter.com\/intent\/tweet\?text=.*10th%20Order!%20%F0%9F%9A%80.*ref=e2e-tenant/);

    const copyBtn = memberPage.locator('[data-testid="share-milestone-copy"]');
    await expect(copyBtn).toBeVisible();

    // Dismiss the alert
    const dismissBtn = memberPage.locator('button[aria-label="Dismiss"]');
    await dismissBtn.click();

    // Ensure it's dismissed
    await expect(memberPage.locator('text=Achievement Unlocked: 10th Order! 🚀')).toBeHidden();

    // Reload and check it's still dismissed (from localStorage)
    await memberPage.goto('/dashboard');
    await expect(memberPage.locator('text=Achievement Unlocked: 10th Order! 🚀')).toBeHidden();
  });
});
