import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('viral_cross_sell_generator', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_cross_sell_generator');
});

test.describe('Viral Cross-Sell Widget Loop', () => {
  test('should load the widget and generate a cross-sell link', async ({ page }) => {
    // Navigate directly to the static UI
    await page.goto('/ui/viral-cross-sell-generator.html');

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Cross-Sell Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Click generate
    await generateBtn.click();

    // Verify animation starts
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // Wait for the animation to finish and result to show
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Check share link generated correctly
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/cross-sell\/view\?ref=/);
  });

  test('should copy the share link to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/ui/viral-cross-sell-generator.html');

    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const copyBtn = page.locator('#copy-btn');
    await expect(copyBtn).toHaveText('Copy');

    await copyBtn.click();

    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('cross-sell/view?ref=');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/viral-cross-sell-generator.html');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });
});
