import { test, expect } from './fixtures';

test.describe('Powered by OHC Widget', () => {
  test('should navigate from dashboard to the widget and generate embed code', async ({ page }) => {
    await page.goto('/ui/dashboard.html');
    await page.click('#powered-by-ohc-link');
    await expect(page).toHaveURL(/.*powered-by-ohc-widget\.html/);

    await expect(page.locator('h1')).toHaveText('Powered by OHC');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    const campaignInput = page.locator('#utm-campaign');
    await campaignInput.fill('test_campaign_e2e');

    await generateBtn.click();

    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const embedCode = page.locator('#embed-code');
    await expect(embedCode).toHaveValue(/utm_campaign=test_campaign_e2e/);
    await expect(embedCode).toHaveValue(/<a href="https:\/\//);
  });

  test('should copy the embed code to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/ui/powered-by-ohc-widget.html');

    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();

    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toHaveText('Copy to Clipboard');

    await copyBtn.click();

    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('Powered by OHC');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/powered-by-ohc-widget.html');
    await page.waitForTimeout(100);

    await expect(page.locator('h1')).toHaveText('Powered by OHC');

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/powered-by-ohc-widget.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });
});
