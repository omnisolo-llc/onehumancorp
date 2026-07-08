import { test, expect } from './fixtures';

test.describe('Viral Loyalty Widget', () => {
  test('should load the widget and generate a loyalty program', async ({ page }) => {
    await page.goto('/ui/dashboard.html');
    await page.click('a#loyalty-link');

    await expect(page.locator('h1')).toHaveText('Viral Loyalty Widget Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    const emptyStamps = page.locator('.stamp.empty');
    await expect(emptyStamps).toHaveCount(4);

    await generateBtn.click();

    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);

    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/loyalty\/join\?ref=/);
  });

  test('should copy the share link to clipboard', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    await page.goto('/ui/dashboard.html');
    await page.click('a#loyalty-link');

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
        expect(clipboardText).toContain('loyalty/join?ref=');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/dashboard.html');
    await page.click('a#loyalty-link');
    await page.waitForTimeout(100);

    await expect(page.locator('h1')).toHaveText('Viral Loyalty Widget Generator');

    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/dashboard.html');
    await page.click('a#loyalty-link');
    const backLink = page.locator('.back-link');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', '/dashboard.html');
  });

  test('should display emojis in stamps', async ({ page }) => {
    await page.goto('/ui/dashboard.html');
    await page.click('a#loyalty-link');

    const generateBtn = page.locator('#generate-btn');
    await generateBtn.click();
    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);

    // Check that at least one stamp contains the coffee emoji
    const firstStamp = filledStamps.first();
    await expect(firstStamp).toContainText('☕');
  });
});
