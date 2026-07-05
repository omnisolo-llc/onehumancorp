import { test, expect } from '@playwright/test';

test.describe('Viral Countdown Drop Generator', () => {
  test('should allow owner to create a countdown widget and show embed code', async ({ page }) => {
    // 1. Navigate directly to the generator
    await page.goto('/viral-countdown-drop.html');

    // 2. Assert heading is visible
    await expect(page.locator('h1', { hasText: 'Viral Countdown Drop' })).toBeVisible();

    // 3. Fill out the form
    await page.fill('input#drop-title', 'Spring Collection Launch');

    // Check toggle logic
    const brandingToggle = page.locator('input#branding-toggle');
    await expect(brandingToggle).toBeChecked();

    // 4. Click generate
    const generateBtn = page.locator('button#generate-btn');
    await expect(generateBtn).toBeEnabled();
    await generateBtn.click();

    // 5. Wait for preview box to appear and verify contents
    const previewContainer = page.locator('#preview-container');
    await expect(previewContainer).toBeVisible();
    await expect(page.locator('h2#preview-title')).toContainText('Spring Collection Launch');

    // 6. Verify embed code is generated
    const embedCode = page.locator('textarea#embed-code');
    const embedValue = await embedCode.inputValue();
    expect(embedValue).toContain('<iframe src="');
    expect(embedValue).toContain('api/v1/growth/countdown/embed');
    expect(embedValue).toContain('Spring%20Collection%20Launch');

    // 7. Verify the copy button works
    const copyBtn = page.locator('button#copy-btn');
    await copyBtn.click();
    await expect(copyBtn).toContainText('Copied!');
  });

  test('should show soft paywall when attempting to remove branding without pro', async ({ page }) => {
    await page.goto('/viral-countdown-drop.html');
    await page.evaluate(() => {
        localStorage.setItem('tenant', 'e2e-test-store');
        localStorage.setItem('has_pro', 'false');
    });
    await page.reload();

    const toggle = page.locator('input#branding-toggle');
    await toggle.uncheck({ force: true });

    // Soft paywall should appear
    await expect(page.locator('text=Pro Feature')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Pro' })).toBeVisible();

    // Dismiss paywall
    await page.locator('button', { hasText: 'Maybe Later' }).click();
    await expect(page.locator('text=Pro Feature')).not.toBeVisible();
  });
});
