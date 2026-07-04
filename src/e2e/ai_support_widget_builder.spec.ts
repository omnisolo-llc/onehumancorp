import { test, expect } from './fixtures';

test.describe('Embeddable AI Support Widget Builder', () => {
  test('should load the builder, generate embed code, and copy it', async ({ page, context }) => {
    // Grant clipboard permissions for copying
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    // Navigate to the AI Support Widget Builder page
    await page.goto('/ui/ai-support-widget-builder.html');

    // Wait for main heading to be visible
    await expect(page.locator('h1')).toHaveText('Embeddable AI Support Widget 💬');

    // Check initial preview state
    const mockGreeting = page.locator('#mockGreeting');
    await expect(mockGreeting).toHaveText('Hi! How can I help you today?');

    // Change the greeting input
    const greetingInput = page.locator('#agentGreeting');
    await greetingInput.fill('Welcome to my awesome store!');

    // Verify preview updates
    await expect(mockGreeting).toHaveText('Welcome to my awesome store!');

    // Change the theme color
    const themeInput = page.locator('#themeColor');
    await themeInput.fill('#ff0000');

    // Click generate
    const generateBtn = page.locator('#generateBtn');
    await generateBtn.click();

    // Verify button state changes to generating
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    // Wait for the result area to become visible
    const resultArea = page.locator('#resultArea');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Verify embed code is generated with correct values
    const embedCode = page.locator('#embedCode');
    await expect(embedCode).toHaveValue(/Welcome to my awesome store!/);
    await expect(embedCode).toHaveValue(/#ff0000/);
    await expect(embedCode).toHaveValue(/<script src="https:\/\/(ohc\.app|127\.0\.0\.1:18789|localhost:\d+)\/widget\.js" async defer><\/script>/);

    // Test copy functionality
    const copyBtn = page.locator('#copyBtn');
    await copyBtn.click();

    // Verify button text changes to "Copied!"
    await expect(copyBtn).toHaveText('Copied!', { timeout: 3000 });

    // Optionally check clipboard text if environment allows
    try {
        const clipboardText = await page.evaluate(async () => {
            return await navigator.clipboard.readText();
        });
        expect(clipboardText).toContain('Welcome to my awesome store!');
        expect(clipboardText).toContain('#ff0000');
    } catch (e) {
        console.warn('Clipboard read failed (expected in some headless environments): ', e);
    }
  });

  test('should show responsive layout on mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto('/ui/ai-support-widget-builder.html');
    await page.waitForTimeout(100);

    // Ensure the main heading is still visible and fits
    await expect(page.locator('h1')).toHaveText('Embeddable AI Support Widget 💬');

    // Check container width fits mobile screen
    const container = page.locator('.container');
    const box = await container.boundingBox();
    expect(box?.width).toBeLessThanOrEqual(375);
  });

  test('should navigate back to the dashboard', async ({ page }) => {
    await page.goto('/ui/ai-support-widget-builder.html');
    const backLink = page.locator('.back-btn');
    await expect(backLink).toBeVisible();
    await expect(backLink).toHaveAttribute('href', 'dashboard.html');
  });
});