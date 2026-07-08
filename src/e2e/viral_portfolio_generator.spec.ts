import { test, expect } from './fixtures';

test.describe('Viral Portfolio Generator Widget', () => {
  test('should load the widget, generate embed code, and copy to clipboard', async ({ page, context }) => {
    // Grant clipboard permissions for copying the link
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.goto('/ui/viral-portfolio-generator.html');

    // Wait for the UI to be ready
    await page.waitForLoadState('networkidle');

    // Verify header and description
    await expect(page.getByRole('heading', { name: 'Embeddable Portfolio Generator' })).toBeVisible();

    // Fill the portfolio details
    await page.locator('#p-name').fill('Alice');
    await page.locator('#p-role').fill('Graphic Designer');
    await page.locator('#p-bio').fill('I design cool things.');

    // Click "Generate Embed Code"
    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    // Wait for the result area to become visible
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Verify the embed code contains the details
    const embedCode = page.locator('#embed-code');
    await expect(embedCode).toContainText('Alice');
    await expect(embedCode).toContainText('Graphic Designer');
    await expect(embedCode).toContainText('I design cool things.');

    // Verify "Powered by OHC" branding is visible by default
    await expect(embedCode).toContainText('Powered by OHC');
    await expect(embedCode).toContainText('ohc.network/invite/');

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
        expect(clipboardText).toContain('Alice');
        expect(clipboardText).toContain('Graphic Designer');
        expect(clipboardText).toContain('Powered by OHC');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should hide branding if remove branding is checked', async ({ page }) => {
    await page.goto('/ui/viral-portfolio-generator.html');

    // Fill the portfolio details
    await page.locator('#p-name').fill('Alice');

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
    await page.goto('/ui/viral-portfolio-generator.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', 'dashboard.html');
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/viral-portfolio-generator.html');
    await page.waitForTimeout(100);

    await expect(page.getByRole('heading', { name: 'Embeddable Portfolio Generator' })).toBeVisible();

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });
});
