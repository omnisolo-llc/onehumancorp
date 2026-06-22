import { test, expect } from '@playwright/test';

test.describe('Zero Click Builder Mobile E2E', () => {
  // Mobile viewport configuration
  test.use({ viewport: { width: 375, height: 812 } });

  test('generates a new store through natural language prompt on mobile', async ({ page }) => {
    // 1. Navigate to the zero-click builder page
    await page.goto('/zero-click-builder');

    // 2. Assert initial mobile UI state
    await expect(page.locator('h1').filter({ hasText: 'Zero-Click Business Generator' })).toBeVisible();
    await expect(page.locator('textarea[placeholder*="I am a home baker"]')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Generate My Business' })).toBeDisabled();

    // 3. Fill in the prompt
    await page.fill('textarea[placeholder*="I am a home baker"]', 'I run a local flower shop in Seattle and need an online store for pre-orders.');

    // 4. Assert button is enabled
    const generateBtn = page.locator('button', { hasText: 'Generate My Business' });
    await expect(generateBtn).toBeEnabled();

    // 5. Submit the form
    // Note: We're calling the real backend API in this E2E test, which handles the generation
    await generateBtn.click();

    // 6. Assert loading state
    await expect(page.locator('text=Analyzing your business...')).toBeVisible();

    // 7. Wait for completion and assert completion state
    // This could take a while if the real backend is calling an LLM
    await expect(page.locator('h2').filter({ hasText: 'Your business is live!' })).toBeVisible({ timeout: 30000 });
    await expect(page.locator('iframe[title="Live Storefront Preview"]')).toBeVisible();

    // 8. Launch the store and assert navigation
    const launchBtn = page.locator('button', { hasText: 'Launch My Store' });
    await expect(launchBtn).toBeVisible();

    await launchBtn.click();
    await expect(page).toHaveURL(/\/dashboard/);
  });
});
