import { test, expect } from '@playwright/test';

test.describe('Viral AMA Generator', () => {
  test('should allow owner to create an AMA widget and show embed code', async ({ page }) => {
    // 1. Navigate directly to the generator
    await page.goto('/viral-ama-generator.html');

    // 2. Assert heading is visible
    await expect(page.locator('h1', { hasText: 'Viral AMA Generator' })).toBeVisible();

    // 3. Fill out the form
    await page.fill('input#ama-title', 'Design Q&A');
    await page.fill('input#ama-host', 'the Lead Designer');

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
    await expect(page.locator('div#preview-title')).toContainText('Design Q&A');
    await expect(page.locator('div#preview-subtitle')).toContainText('Ask the Lead Designer anything!');

    // 6. Verify embed code is generated
    const embedCode = page.locator('textarea#embed-code');
    const embedValue = await embedCode.inputValue();
    expect(embedValue).toContain('<iframe src="');
    expect(embedValue).toContain('api/v1/growth/ama/embed');
    expect(embedValue).toContain('Design%20Q%26A');
    expect(embedValue).toContain('the%20Lead%20Designer');

    // 7. Verify the copy button works
    const copyBtn = page.locator('button#copy-btn');
    await copyBtn.click();
    await expect(copyBtn).toContainText('Copied!');
  });

  test('should show soft paywall when attempting to remove branding without pro', async ({ page }) => {
    await page.goto('/viral-ama-generator.html');

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
