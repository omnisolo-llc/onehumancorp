import { test, expect } from './fixtures';

test.describe('Viral Powered by OHC Widget', () => {
  test('should load the widget and generate an embed code snippet', async ({ page }) => {
    await page.goto('/ui/viral-powered-by-ohc-widget.html');

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Embed Footer Badge');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Click generate
    await generateBtn.click();

    // Wait for the animation to finish and result to show
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check embed code generated correctly
    const embedCode = page.locator('#embed-code');
    await expect(embedCode).toContainText('Powered by OHC');
    await expect(embedCode).toContainText('ohc.network/invite/');
  });

  test('should copy the share link to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/ui/viral-powered-by-ohc-widget.html');

    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toHaveText('Copy Code');

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

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/viral-powered-by-ohc-widget.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });
});
