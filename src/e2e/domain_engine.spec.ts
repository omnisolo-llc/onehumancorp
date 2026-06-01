import { test, expect } from '@playwright/test';

test('Domain Engine Custom Domain Purchase Flow', async ({ page }) => {
  await page.goto('http://localhost:3000/');

  // Click login
  await page.locator('text=Login Sign In').click();

  // Show domains setup
  await page.evaluate(() => {
    // @ts-ignore
    window.showDomainSetup();
  });

  // Verify modal opens
  await expect(page.locator('#domain-setup-sheet')).toHaveClass(/open/);

  // Click Connect Custom Domain
  await page.locator('button', { hasText: '🔗 Connect Custom Domain' }).click();

  // Verify custom domain step is active
  await expect(page.locator('#domain-step-custom')).toHaveClass(/active/);

  // Search for domain
  await page.fill('#custom-domain-search-input', 'mayascakes.com');
  await page.locator('button', { hasText: 'Search' }).click();

  // Wait for available message
  await expect(page.locator('#custom-domain-status')).toContainText('mayascakes.com is available');
  await expect(page.locator('#custom-domain-status')).toContainText('$12/yr');

  // Click Buy & Configure
  await page.locator('#custom-domain-buy-btn').click();

  // Wait for loading to show and disappear
  await expect(page.locator('#custom-domain-loading')).toBeVisible();

  // Should navigate to dashboard eventually (via confetti and timeout)
  await expect(page.locator('#dashboard-screen')).toBeVisible({ timeout: 10000 });
});
