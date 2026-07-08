import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Viral Give-Get Widget', () => {
  test('should verify empty fields update preview correctly', async ({ page }) => {
    // Navigate via login fixture and ensure proper authentication context
    await adminPage(page, async () => {
      // Load actual dashboard then navigate
      await page.goto('/ui/dashboard.html');
      await page.locator('#give-get-link').click();

      // Check main element loaded
      await expect(page.locator('h1')).toHaveText('Viral Give-Get Generator');

      // Check input state
      await expect(page.locator('#give-reward')).toHaveValue('20% Off');
      await expect(page.locator('#get-reward')).toHaveValue('$10 Credit');

      // Change value
      await page.locator('#give-reward').fill('50% Off');
      await page.locator('#get-reward').fill('$20 Cash');

      // Assert update
      await expect(page.locator('#give-display')).toHaveText('50% Off');
      await expect(page.locator('#get-display')).toHaveText('$20 Cash');
    });
  });

  test('should load the widget and generate a give-get referral program', async ({ page }) => {
    // Use the adminPage fixture to run the CUJ properly (from dashboard)
    await adminPage(page, async () => {
      // Start from dashboard and click the widget link
      await page.goto('/ui/dashboard.html');
      await page.locator('#give-get-link').click();

      // Wait for main elements
      await expect(page.locator('h1')).toHaveText('Viral Give-Get Generator');
      const generateBtn = page.locator('#generate-btn');
      await expect(generateBtn).toBeVisible();

      // Click generate (this hits the real `/api/v1/growth/referrals/generate` endpoint)
      await generateBtn.click();

      // Verify button goes to generating state
      await expect(generateBtn).toBeDisabled();
      await expect(generateBtn).toHaveText('Generating...');

      // Wait for the animation to finish and result to show
      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      // Verify button restored
      await expect(generateBtn).not.toBeDisabled();
      await expect(generateBtn).toHaveText('Generate Referral Link');

      // Check share link generated correctly (the real endpoint returns ohc.app/ref/UUID format)
      const shareLink = page.locator('#share-link');
      await expect(shareLink).toHaveValue(/\/give-get\/join\?ref=.+/);

      // Check that boxes are active
      const giveBox = page.locator('#give-box');
      await expect(giveBox).toHaveClass(/active/);
      const getBox = page.locator('#get-box');
      await expect(getBox).toHaveClass(/active/);
    });
  });

  test('should copy text successfully', async ({ page, context }) => {
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await adminPage(page, async () => {
      await page.goto('/ui/viral-give-get-widget.html');

      const generateBtn = page.locator('#generate-btn');
      await generateBtn.click();

      // Wait for the animation to finish and result to show
      const resultArea = page.locator('#result-area');
      await expect(resultArea).toBeVisible({ timeout: 5000 });

      const copyBtn = page.locator('#copy-btn');
      await copyBtn.click();

      // Check copied status
      await expect(copyBtn).toHaveText('Copied!');
    });
  });
});
