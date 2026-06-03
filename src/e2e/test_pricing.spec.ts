import { test, expect } from '@playwright/test';

test('pricing page checkout', async ({ page }) => {
  await page.goto('http://127.0.0.1:18789/pricing');
  await expect(page.locator('#pricing-screen')).toBeVisible();

  // Wait a bit to ensure it loads
  await page.waitForTimeout(1000);

  // Click the checkout button for starter
  const checkoutPromise = page.waitForResponse(response => response.url().includes('/api/billing/checkout'));
  await page.getByRole('button', { name: 'Upgrade to Starter via Stripe' }).click();
  const response = await checkoutPromise;

  expect(response.status()).toBe(200);
});
