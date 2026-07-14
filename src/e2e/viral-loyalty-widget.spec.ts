import { test, expect } from './fixtures';

test.describe('Viral Loyalty Widget', () => {
  test('should load the widget and generate a loyalty program', async ({ page, loginAs, adminUser }) => {
    // Start at dashboard after login as required
    await loginAs(page, adminUser);

    // Navigate via UI click as required by E2E standards
    await page.locator('#loyalty-link').click();

    // Wait for main elements
    await expect(page.locator('h1')).toHaveText('Viral Loyalty Widget Generator');
    const generateBtn = page.locator('#generate-btn');
    await expect(generateBtn).toBeVisible();

    // Check initial stamps state
    const emptyStamps = page.locator('.stamp.empty');
    await expect(emptyStamps).toHaveCount(4);

    // Click generate
    await generateBtn.click();

    // Verify animation starts
    await expect(generateBtn).toBeDisabled();
    await expect(generateBtn).toHaveText('Generating...');

    const resultArea = page.locator('#result-area');
    await expect(resultArea).toBeVisible({ timeout: 5000 });

    // Verify filled stamps
    const filledStamps = page.locator('.stamp.filled');
    await expect(filledStamps).toHaveCount(4);
    await expect(filledStamps.first()).toHaveText('☕');

    // Check share link generated correctly
    const shareLink = page.locator('#share-link');
    await expect(shareLink).toHaveValue(/loyalty\/join\?ref=[a-zA-Z0-9_-]+/);

    // Verify powered by OHC footer widget exists
    const poweredByFooter = page.locator('#ohc-powered-footer');
    await expect(poweredByFooter).toBeVisible();
    await expect(poweredByFooter).toContainText('Powered by OHC');
  });
});
