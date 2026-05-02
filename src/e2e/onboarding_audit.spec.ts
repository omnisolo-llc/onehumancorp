import { test, expect } from '@playwright/test';

test.describe('Onboarding Flow Audit', () => {
  test('should complete the full 11-step onboarding journey', async ({ page }) => {
    await page.goto('/onboarding');

    // Step 0: Welcome
    await expect(page.locator('text=/your business,.*live in minutes/i')).toBeVisible();
    await page.click('text=/Guided Setup/i');

    // Step 1: Business Type
    await expect(page.locator('text=/what kind of business/i')).toBeVisible();
    await page.click('text=/Online Store/i');

    // Step 2: Name & Description
    await expect(page.locator('text=/give your business a name/i')).toBeVisible();
    await page.fill('input[placeholder*="Maya\'s Cakes"]', 'Audit Bakery');
    await page.click('text=/Next/i');

    // Step 3: What do you sell
    await expect(page.locator('text=/what do you sell/i')).toBeVisible();
    await page.click('text=/Physical products/i');
    await page.click('text=/Next/i');

    // Step 4: Payments
    await expect(page.locator('text=/receive payments/i')).toBeVisible();
    await page.click('text=/Online only/i');

    // Step 5: Admin Account
    await expect(page.locator('text=/create your account/i')).toBeVisible();
    await page.fill('input[placeholder*="Maya Smith"]', 'Auditor');
    await page.fill('input[placeholder*="you@email.com"]', 'audit@test.com');
    await page.fill('input[type="password"]', 'securePassword123');
    await page.click('text=/Next/i');

    // Step 6: Choose a Template
    await expect(page.locator('text=/choose a template/i')).toBeVisible();
    await page.click('text=/Modern/i');

    // Step 7: Add first product
    await expect(page.locator('text=/add your first product/i')).toBeVisible();
    await page.fill('input[placeholder*="Birthday Cake"]', 'Classic Croissant');
    await page.fill('input[placeholder*="50.00"]', '4.50');
    await page.click('text=/Next/i');

    // Step 8: Choose a Domain
    await expect(page.locator('text=/choose a domain/i')).toBeVisible();
    await page.click('text=/Free OHC Domain/i');

    // Step 9: Review & Launch
    await expect(page.locator('text=/ready to launch/i')).toBeVisible();
    await expect(page.locator('text=/Business: Audit Bakery/i')).toBeVisible();
    await page.click('text=/Launch My Business/i');

    // Step 10: Checklist (Post-launch)
    await expect(page.locator('text=/you\'re set up/i')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=/Business live/i')).toBeVisible();

    await page.click('text=/Go to Dashboard/i');
    await expect(page.locator('text=/My Business/i')).toBeVisible();
  });

  test('should handle AI-powered Instant Build', async ({ page }) => {
    await page.goto('/onboarding');
    await page.click('text=/Instant Build/i');

    // Step 11: Instant Build
    await expect(page.locator('text=/tell us about your business/i')).toBeVisible();
    await page.fill('textarea, input', 'I sell artisanal bread in San Francisco.');
    await page.click('text=/Generate Storefront/i');

    // Should skip to Step 9: Review
    await expect(page.locator('text=/ready to launch/i')).toBeVisible({ timeout: 10000 });
    await page.click('text=/Launch My Business/i');

    // Step 10: Checklist
    await expect(page.locator('text=/you\'re set up/i')).toBeVisible({ timeout: 15000 });
  });
});
