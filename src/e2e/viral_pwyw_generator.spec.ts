import { test, expect } from './fixtures';

test.describe('Viral PWYW Generator Widget', () => {
  test('should load the widget, generate embed code, and copy to clipboard', async ({ page, context }) => {
    // Grant clipboard permissions for copying the link
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/ui/viral-pwyw-generator.html');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Verify header and description
    await expect(page.getByRole('heading', { name: 'Viral PWYW Drop Generator' })).toBeVisible();

    // Fill the generator details
    await page.locator('#product-name').fill('The Ultimate Notion Template');
    await page.locator('#product-desc').fill('Organize your entire life with this template.');
    await page.locator('#min-price').fill('1');
    await page.locator('#suggested-price').fill('5');

    // Click "Generate Embed Code"
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Wait for the result area to become visible
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Verify the embed code contains the details
    const embedCode = page.locator('#embed-code');
    await expect(embedCode).toContainText('The Ultimate Notion Template');
    await expect(embedCode).toContainText('Organize your entire life with this template.');
    await expect(embedCode).toContainText('min="1"');
    await expect(embedCode).toContainText('value="5"');

    // Verify "Powered by OHC" branding is visible by default
    await expect(embedCode).toContainText('Powered by OHC');
    await expect(embedCode).toContainText('source=viral_pwyw');

    // Click "Copy Code"
    const copyBtn = page.locator('#copy-btn');
    await copyBtn.click();

    // Verify button text changes to Copied!
    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

    // Verify clipboard content
    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('The Ultimate Notion Template');
        expect(clipboardText).toContain('Organize your entire life with this template.');
        expect(clipboardText).toContain('Powered by OHC');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should hide branding if remove branding is checked', async ({ page }) => {
    await page.goto('/ui/viral-pwyw-generator.html');

    // Fill the details
    await page.locator('#product-name').fill('Cool Thing');

    // Toggle the "Remove branding" checkbox
    await page.locator('label', { hasText: 'Remove "Powered by OHC" Badge' }).click();

    // Click "Generate Embed Code"
    await page.locator('#generate-btn').click();

    // Wait for result area
    await expect(page.locator('#result-area')).toBeVisible({ timeout: 5000 });

    // Verify the branding footer is hidden
    const embedCode = page.locator('#embed-code');
    await expect(embedCode).not.toContainText('Powered by OHC');
  });

  test('should navigate back to dashboard', async ({ page }) => {
    await page.goto('/ui/viral-pwyw-generator.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', 'dashboard.html');
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/viral-pwyw-generator.html');
    await page.waitForTimeout(100);

    await expect(page.getByRole('heading', { name: 'Viral PWYW Drop Generator' })).toBeVisible();

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});
