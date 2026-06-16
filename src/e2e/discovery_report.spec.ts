import { test, expect } from '@playwright/test';

test.describe('Autonomous SEO & Local Discovery Agent', () => {
  test('Owner can view plain-language AI discovery report', async ({ page }) => {
    // 1. Visit storefront builder to trigger the SEO generation job
    await page.goto('/storefront-builder');

    // Fill the description to enable generation
    const textarea = page.locator('textarea');
    if (await textarea.isVisible()) {
        await textarea.fill('My test plumbing business needs a website');
    }

    // Click Generate
    const generateBtn = page.locator('#generate-btn');
    if (await generateBtn.isVisible()) {
        await generateBtn.click();
    }

    // Wait for generation to finish and click 1-Tap Launch
    const launchBtn = page.locator('button:has-text("1-Tap Launch")');
    await expect(launchBtn).toBeVisible({ timeout: 60000 });
    await launchBtn.click();

    // Verify it goes live
    await expect(page.locator('text=You\'re Live!')).toBeVisible({ timeout: 60000 });

    // Wait a moment for background publish job to finish generating the discovery report
    await page.waitForTimeout(3000);

    // 2. Visit the discovery report page
    await page.goto('/discovery-report');

    // 3. Verify the report was created
    await expect(page.locator('text=Optimized')).toBeVisible();
    await expect(page.locator('text=Your AI Discovery Agent')).toBeVisible();
  });
});
