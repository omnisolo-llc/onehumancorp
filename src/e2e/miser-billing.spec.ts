import { test, expect } from '@playwright/test';

test('Cost Transparency and Billing CUJ', async ({ page }) => {
  // Start at the home page (simulated via /dashboard or similar)
  await page.goto('/');

  // 1. Navigate to Cost Dashboard
  // Assuming there is a link or we can go directly to /cost-dashboard
  await page.goto('/cost-dashboard');
  await expect(page.locator('text=Miser Cost Dashboard')).toBeVisible();
  await expect(page.locator('text=Monthly Budget')).toBeVisible();
  await page.screenshot({ path: 'cost-dashboard.png', fullPage: true });

  // 2. Check "My Plan"
  // Assuming "My Plan" is a link in the navigation or sidebar
  await page.goto('/my-plan');
  await expect(page.locator('text=Current Plan')).toBeVisible();
  await page.screenshot({ path: 'my-plan.png', fullPage: true });

  // 3. View Pricing comparison
  await page.goto('/pricing');
  await expect(page.locator('text=Simple, Transparent Pricing')).toBeVisible();
  await page.screenshot({ path: 'pricing.png', fullPage: true });

  // 4. Verify specific "Miser" optimizations in UI
  // Check if backdrop-blur (translucent glass) is present in the computed style
  const pricingCard = page.locator('div.backdrop-blur-md').first();
  await expect(pricingCard).toBeVisible();
});
