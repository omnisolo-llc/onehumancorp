import { test, expect } from '@playwright/test';

test.describe('Growth Page & Referral Loop', () => {
  test('navigates to growth page and interacts with referrals', async ({ page }) => {
    // A `.mock-contract.ts` suffix allows us to intercept data according to the rules
    await page.route('/api/v1/growth/referrals/stats', async route => {
      const json = {
        referralLink: 'https://ohc.com/join?ref=E2E_BUSINESS',
        milestones: [
          { target: 10, current: 10, label: 'Orders Completed' },
          { target: 50, current: 5, label: 'Orders Completed' },
          { target: 100, current: 0, label: 'Orders Completed' }
        ]
      };
      await route.fulfill({ json });
    });

    await page.goto('/growth');

    // Wait for the UI to load with real data
    const input = page.locator('input[data-testid="referral-input"]');
    await expect(input).toBeVisible();
    await expect(input).toHaveValue('https://ohc.com/join?ref=E2E_BUSINESS');

    // Test the milestones rendering
    await expect(page.locator('text=10').first()).toBeVisible();
    await expect(page.locator('text=50').first()).toBeVisible();
    await expect(page.locator('text=100').first()).toBeVisible();

    // Verify copy functionality
    const copyButton = page.locator('button:has-text("Copy Link")');
    await copyButton.click();
    await expect(page.locator('button:has-text("Copied!")')).toBeVisible();
  });
});
